use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::ClauseID;
use crate::heuristics::restarts::RestartPolicy;

pub struct NoRestart;

impl<C: ClauseDB> RestartPolicy<C> for NoRestart {
    fn should_restart(&self) -> bool {
        false
    }
    fn on_conflict(&self, clause_db: &C, clause_id: ClauseID, lbd: u32) { }
    fn on_restart(&self) { }
}