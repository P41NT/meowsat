use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::propagate::propagator::Propagator;

pub trait Solver<Clauses: ClauseDB, Prop: Propagator<Clauses>> {
    fn new(clause_db: Clauses, prop: Prop, assignment: Assignment) -> Self;
    fn solve(&mut self) -> Option<Vec<bool>>;
}