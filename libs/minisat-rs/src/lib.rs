mod core;
pub use core::*;
mod simp;
pub use simp::*;

use logic_form::Lit;
use std::{
    ffi::{c_int, c_void},
    fmt::{self, Debug},
    marker::PhantomData,
};

extern "C" {
    fn solver_model_value(s: *mut c_void, lit: c_int) -> c_int;
    fn solver_conflict_has(s: *mut c_void, lit: c_int) -> bool;
}

pub struct Model<'a> {
    solver: *mut c_void,
    _pd: PhantomData<&'a ()>,
}

impl Model<'_> {
    pub fn lit_value(&self, lit: Lit) -> bool {
        let res = unsafe { solver_model_value(self.solver, lit.into()) };
        assert!(res == 0 || res == 1);
        res == 0
    }
}

pub struct Conflict<'a> {
    solver: *mut c_void,
    _pd: PhantomData<&'a ()>,
}

impl Conflict<'_> {
    pub fn has(&self, lit: Lit) -> bool {
        unsafe { solver_conflict_has(self.solver, lit.into()) }
    }
}

pub enum SatResult<'a> {
    Sat(Model<'a>),
    Unsat(Conflict<'a>),
}

impl Debug for SatResult<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sat(_) => "Sat".fmt(f),
            Self::Unsat(_) => "Unsat".fmt(f),
        }
    }
}
