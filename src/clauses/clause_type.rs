use crate::assignment::Assignment;
use crate::types::{LBool, Literal};

// strong typed ClauseID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClauseID (pub usize);

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

    pub fn is_satisfied(&self, assignment: &Assignment) -> bool {
        self.literals.iter().any(|lit| assignment.literal(*lit) == LBool::True)
    }

    pub fn is_unsatisfied(&self, assignment: &Assignment) -> bool {
        self.literals.iter().all(|lit| assignment.literal(*lit) == LBool::False)
    }
}