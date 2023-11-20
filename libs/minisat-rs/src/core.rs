use crate::{Conflict, Model, SatResult};
use logic_form::{Lit, Var};
use std::{
    ffi::{c_int, c_void},
    marker::PhantomData,
};

extern "C" {
    fn solver_new() -> *mut c_void;
    fn solver_free(s: *mut c_void);
    fn solver_new_var(s: *mut c_void) -> c_int;
    fn solver_num_var(s: *mut c_void) -> c_int;
    fn solver_add_clause(s: *mut c_void, clause: *mut c_int, len: c_int) -> bool;
    fn solver_solve(s: *mut c_void, assumps: *mut c_int, len: c_int) -> bool;
    fn solver_simplify(s: *mut c_void) -> bool;
    fn solver_release_var(s: *mut c_void, lit: c_int);
    fn solver_set_random_seed(s: *mut c_void, seed: f64);
    fn solver_set_rnd_init_act(s: *mut c_void, enable: bool);
    fn solver_set_polarity(s: *mut c_void, var: c_int, pol: c_int);
}

pub struct Solver {
    solver: *mut c_void,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            solver: unsafe { solver_new() },
        }
    }

    pub fn new_var(&mut self) -> Var {
        Var::new(unsafe { solver_new_var(self.solver) } as usize)
    }

    pub fn num_var(&self) -> usize {
        unsafe { solver_num_var(self.solver) as _ }
    }

    pub fn add_clause(&mut self, clause: &[Lit]) {
        assert!(unsafe { solver_add_clause(self.solver, clause.as_ptr() as _, clause.len() as _) });
    }

    pub fn solve<'a>(&'a mut self, assumps: &[Lit]) -> SatResult<'a> {
        if unsafe { solver_solve(self.solver, assumps.as_ptr() as _, assumps.len() as _) } {
            SatResult::Sat(Model {
                solver: self.solver,
                _pd: PhantomData,
            })
        } else {
            SatResult::Unsat(Conflict {
                solver: self.solver,
                _pd: PhantomData,
            })
        }
    }

    pub fn simplify(&mut self) -> bool {
        unsafe { solver_simplify(self.solver) }
    }

    pub fn release_var(&mut self, lit: Lit) {
        unsafe { solver_release_var(self.solver, lit.into()) }
    }

    pub fn set_polarity(&mut self, var: Var, pol: Option<bool>) {
        let pol = match pol {
            Some(true) => 0,
            Some(false) => 1,
            None => 2,
        };
        unsafe { solver_set_polarity(self.solver, var.into(), pol) }
    }

    pub fn set_random_seed(&mut self, seed: f64) {
        unsafe { solver_set_random_seed(self.solver, seed) }
    }

    pub fn set_rnd_init_act(&mut self, enable: bool) {
        unsafe { solver_set_rnd_init_act(self.solver, enable) }
    }

    /// # Safety
    /// unsafe get sat model
    pub unsafe fn get_model(&self) -> Model<'static> {
        Model {
            solver: self.solver,
            _pd: PhantomData,
        }
    }

    /// # Safety
    /// unsafe get unsat core
    pub unsafe fn get_conflict(&self) -> Conflict<'static> {
        Conflict {
            solver: self.solver,
            _pd: PhantomData,
        }
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { solver_free(self.solver) }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test() {
    use logic_form::Clause;
    let mut solver = Solver::new();
    let lit0: Lit = solver.new_var().into();
    let lit1: Lit = solver.new_var().into();
    let lit2: Lit = solver.new_var().into();
    solver.add_clause(&Clause::from([lit0, !lit2]));
    solver.add_clause(&Clause::from([lit1, !lit2]));
    solver.add_clause(&Clause::from([!lit0, !lit1, lit2]));
    match solver.solve(&[lit2]) {
        SatResult::Sat(model) => {
            dbg!(model.lit_value(lit0));
            dbg!(model.lit_value(lit1));
            dbg!(model.lit_value(lit2));
        }
        SatResult::Unsat(_) => todo!(),
    }
    solver.add_clause(&Clause::from([!lit0]));
    match solver.solve(&[lit2]) {
        SatResult::Sat(_) => panic!(),
        SatResult::Unsat(conflict) => {
            dbg!(conflict.has(!lit2));
        }
    }
}
