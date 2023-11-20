use crate::*;
use logic_form::{Clause, Cnf, Var};

extern "C" {
    fn simp_solver_new() -> *mut c_void;
    fn simp_solver_free(s: *mut c_void);
    fn simp_solver_new_var(s: *mut c_void) -> c_int;
    fn simp_solver_num_var(s: *mut c_void) -> c_int;
    fn simp_solver_add_clause(s: *mut c_void, clause: *mut c_int, len: c_int) -> bool;
    fn simp_solver_set_frozen(s: *mut c_void, var: *mut c_int, frozen: bool);
    fn simp_solver_eliminate(s: *mut c_void, turn_off_elim: bool) -> bool;
    fn simp_solver_clauses(s: *mut c_void, len: *mut c_int) -> *mut c_void;
}

pub struct SimpSolver {
    solver: *mut c_void,
}

impl SimpSolver {
    pub fn new() -> Self {
        Self {
            solver: unsafe { simp_solver_new() },
        }
    }

    pub fn new_var(&mut self) -> Var {
        Var::new(unsafe { simp_solver_new_var(self.solver) } as usize)
    }

    pub fn num_var(&self) -> usize {
        unsafe { simp_solver_num_var(self.solver) as _ }
    }

    pub fn add_clause(&mut self, clause: &[Lit]) {
        assert!(unsafe {
            simp_solver_add_clause(self.solver, clause.as_ptr() as _, clause.len() as _)
        });
    }

    pub fn set_frozen(&mut self, var: Var, frozen: bool) {
        unsafe { simp_solver_set_frozen(self.solver, Into::<i32>::into(var) as _, frozen) }
    }

    pub fn eliminate(&mut self, turn_off_elim: bool) -> bool {
        unsafe { simp_solver_eliminate(self.solver, turn_off_elim) }
    }

    pub fn clauses(&self) -> Cnf {
        unsafe {
            let mut cnf = Cnf::new();
            let mut len = 0;
            let clauses: *mut usize = simp_solver_clauses(self.solver, &mut len as *mut _) as _;
            let clauses = Vec::from_raw_parts(clauses, len as _, len as _);
            for i in (0..clauses.len()).step_by(2) {
                let data = clauses[i] as *mut Lit;
                let len = clauses[i + 1];
                let cls = Vec::from_raw_parts(data, len, len);
                cnf.add_clause(Clause::from(cls));
            }
            cnf
        }
    }
}

impl Drop for SimpSolver {
    fn drop(&mut self) {
        unsafe { simp_solver_free(self.solver) }
    }
}

impl Default for SimpSolver {
    fn default() -> Self {
        Self::new()
    }
}
