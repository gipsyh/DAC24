use crate::{solver::Ic3Solver, Ic3};
use minisat::SatResult;

impl Ic3 {
    pub fn verify(&mut self) -> bool {
        let invariant = self
            .frames
            .iter()
            .position(|frame| frame.is_empty())
            .unwrap();
        let mut solver = Ic3Solver::new(self.share.clone(), invariant);
        let mut num = 0;
        for i in invariant..self.frames.len() {
            for cube in self.frames[i].iter() {
                num += 1;
                solver.add_clause(&!cube);
            }
        }
        if let SatResult::Sat(_) = solver.solve(&self.share.bad) {
            return false;
        }
        for i in invariant..self.frames.len() {
            for cube in self.frames[i].iter() {
                if let SatResult::Sat(_) = solver.solve(&self.share.model.cube_next(cube)) {
                    return false;
                }
            }
        }
        println!("inductive invariant verified with {num} lemmas!");
        true
    }
}
