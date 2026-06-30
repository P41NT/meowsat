use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::ClauseID;
use crate::types::Literal;
pub trait DecisionHeuristic<Clause: ClauseDB> {
    fn pick(&mut self, assignment: &Assignment) -> Option<Literal>;
    fn on_conflict(&mut self, clause_db: &Clause, clause_id: ClauseID, bumped_vars: &[u32]);
    fn on_unassign(&mut self, lit: Literal);
    fn on_restart(&mut self);
}

pub mod vsids;

