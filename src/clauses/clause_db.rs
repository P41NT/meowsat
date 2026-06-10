use crate::types::{LBool, Literal};
use crate::assignment::Assignment;

#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn is_satisfied(&self, assignment: Assignment) -> bool {
        self.literals.iter().any(|lit| assignment.literal(*lit) == LBool::True)
    }

    pub fn is_not_satisfied(&self, assignment: Assignment) -> bool {
        self.literals.iter().all(|lit| assignment.literal(*lit) == LBool::False)
    }
}

pub trait ClauseDB {
    fn add_clause(&mut self, literals: Clause);
    fn is_satisfied(&self, assignment: Assignment) -> bool;
    fn is_unsatisfied(&self, assignment: Assignment) -> bool;
}