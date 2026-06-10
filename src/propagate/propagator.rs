use std::rc::Weak;
use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::{Clause, ClauseID};

pub trait Propagator<Clauses: ClauseDB> {
    fn propagate(&mut self, assignment: &Assignment, clause_db: Clauses) -> Option<ClauseID>;
    // propagate returns clause_id in case of conflict
}