use aig::{AigCube, AigEdge, TernaryValue};
use logic_form::{Cube, Var};
use std::assert_matches::assert_matches;

use crate::Ic3;

impl Ic3 {
    #[allow(dead_code)]
    pub fn generalize_by_ternary_simulation(
        &self,
        model: minisat::Model,
        assumptions: &Cube,
    ) -> Cube {
        let aig = &self.share.aig;
        let assumptions = AigCube::from_cube(assumptions);

        let mut primary_inputs = Vec::new();
        let mut latch_inputs = Vec::new();
        for input in &aig.inputs {
            primary_inputs.push(model.lit_value(Var::from(*input).into()).into());
        }
        for latch in &aig.latchs {
            latch_inputs.push(model.lit_value(Var::from(latch.input).into()).into());
        }
        let mut simulation = aig.ternary_simulate(&primary_inputs, &latch_inputs);
        for logic in assumptions.iter().chain(aig.constraints.iter()) {
            assert_matches!(
                simulation[logic.node_id()].not_if(logic.compl()),
                TernaryValue::True
            );
        }
        for (i, li) in latch_inputs.iter_mut().enumerate().take(aig.latchs.len()) {
            assert_matches!(*li, TernaryValue::True | TernaryValue::False);
            let origin = *li;
            *li = TernaryValue::X;
            simulation =
                aig.update_ternary_simulate(simulation, aig.latchs[i].input, TernaryValue::X);
            for logic in assumptions.iter().chain(aig.constraints.iter()) {
                match simulation[logic.node_id()].not_if(logic.compl()) {
                    TernaryValue::True => (),
                    TernaryValue::False => panic!(),
                    TernaryValue::X => {
                        *li = origin;
                        simulation =
                            aig.update_ternary_simulate(simulation, aig.latchs[i].input, origin);
                        break;
                    }
                }
            }
        }
        let mut cube = AigCube::new();
        for (i, value) in latch_inputs.iter().enumerate().take(aig.latchs.len()) {
            match value {
                TernaryValue::True => {
                    cube.push(AigEdge::new(aig.latchs[i].input, false));
                }
                TernaryValue::False => {
                    cube.push(AigEdge::new(aig.latchs[i].input, true));
                }
                TernaryValue::X => (),
            }
        }
        cube.to_cube()
    }
}
