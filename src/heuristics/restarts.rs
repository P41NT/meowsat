use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::ClauseID;

pub trait RestartPolicy<C: ClauseDB> {
    fn should_restart(&self) -> bool;
    fn on_conflict(&self, clause_db: &C, clause_id: ClauseID, lbd: u32);
    fn on_restart(&self);
}

pub mod no_restart;