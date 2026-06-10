use crate::assignment::Assignment;
use crate::clauses::clause_type::Clause;
use crate::clauses::clause_type::ClauseID;

pub trait ClauseDB {
    fn add_clause(&mut self, literals: Clause) -> ClauseID;
    fn is_satisfied(&self, assignment: &Assignment) -> bool;
    fn is_unsatisfied(&self, assignment: &Assignment) -> bool;
}