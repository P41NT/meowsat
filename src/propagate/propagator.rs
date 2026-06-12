use std::rc::Weak;
use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::{Clause, ClauseID};
use crate::types::Literal;

#[derive(Debug, Clone, Copy)]
pub struct Watcher {
    pub clause_id: ClauseID,
    pub other_lit: Literal
}

pub trait Propagator<Clauses: ClauseDB> {
    fn new(clause_db: &Clauses, assignment: &mut Assignment) -> Self;
    fn propagate(&mut self, assignment: &mut Assignment, clause_db: Clauses) -> Option<ClauseID>;
    // propagate returns clause_id in case of conflict
}