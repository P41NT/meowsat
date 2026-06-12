use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::propagate::propagator::Propagator;
use crate::solvers::solver::Solver;

struct DPLLSolver<Clauses: ClauseDB, Prop: Propagator<Clauses>> {
    assignments: Assignment,
    clause_db: Clauses,
    prop: Prop,
}

impl<Clauses: ClauseDB, Prop: Propagator<Clauses>> Solver<Clauses, Prop> for DPLLSolver<Clauses, Prop> {
    fn new(clause_db: Clauses, prop: Prop, assignment: Assignment) -> Self {
        todo!("Implement constructor for DPLL solver")
    }

    fn solve(&mut self) -> Option<Vec<bool>> {
        todo!("Implement solve function for DPLL solver")
    }
}