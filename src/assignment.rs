use std::rc::Weak;
use crate::clauses::clause_db::Clause;
use crate::types::{LBool, Literal};

pub struct Assignment {
    assignments: Vec<LBool>,
    trails: Vec<Literal>,
    trail_lim: Vec<usize>,
    cause: Vec<Weak<Clause>> // required for CDCL I think
}

impl Assignment {
    pub fn new(num_vars: usize) -> Self {
        todo!("Implement constructor for Assignment struct")
    }

    pub fn literal(&self, lit: Literal) -> LBool {
        todo!("Implement literal look up, returns if given literal is true or false or undef")
    }

    pub fn enqueue(&mut self, lit: Literal) {
        todo!("Implement enqueue for assignments")
    }

    pub fn pop_to_level(&mut self, level: usize) {
        todo!("Implement pop to level for assignments")
    }
}