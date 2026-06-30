use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::heuristics::decisions::DecisionHeuristic;
use crate::heuristics::restarts::RestartPolicy;
use crate::propagate::propagator::Propagator;
use crate::solvers::conflict_handler::{ConflictHandler, ConflictResult};
use crate::types::{LBool, Literal};

pub trait SolverConfig {
    type DB: ClauseDB;
    type Propagator:  Propagator<Self::DB>;
    type Heuristic:  DecisionHeuristic<Self::DB>;
    type Restarts:  RestartPolicy<Self::DB>;
}

pub struct CDCLSolver<Config: SolverConfig>
{
    pub assignment: Assignment,
    pub clause_db: Config::DB,
    pub propagator: Config::Propagator,
    pub decision_heuristic: Config::Heuristic,
    pub restart_policy: Config::Restarts,
    pub conflict_handler: ConflictHandler,
}

pub enum SolverResult {
    SAT(Vec<bool>),
    UNSAT
}

impl<Config: SolverConfig> CDCLSolver<Config>
{
    pub fn new(assignment: Assignment, clause_db: Config::DB, propagator: Config::Propagator, decision_heuristic: Config::Heuristic, restart_policy: Config::Restarts, conflict_handler: ConflictHandler) -> Self {
        Self {
            assignment,
            clause_db,
            propagator,
            decision_heuristic,
            restart_policy,
            conflict_handler
        }
    }
    fn backtrack(&mut self, level: usize) {
        let unassigned = self.assignment.pop_to_level(level);
        for lit in &unassigned {
            self.decision_heuristic.on_unassign(*lit);
        }
        self.propagator.reset_head(&self.assignment);
    }
    fn restart(&mut self) {
        self.backtrack(0);
        self.decision_heuristic.on_restart();
        self.restart_policy.on_restart();
    }
    pub fn solve(&mut self) -> SolverResult {
        loop {
            let conflict = self.propagator.propagate(&mut self.assignment, &mut self.clause_db);
            if let Some(conflict) = conflict {
                if self.assignment.trail_lim.is_empty() {
                    return SolverResult::UNSAT;
                }
                let ConflictResult {learned_clause, lbd, jump_level, bumped_vars} = self.conflict_handler.handle_conflict(&self.clause_db, conflict, &self.assignment);
                let asserting_lit = learned_clause.literals[0];
                self.backtrack(jump_level);
                let learned_id= self.clause_db.add_clause(learned_clause);
                self.propagator.add_clause(learned_id, &self.clause_db);
                self.assignment.enqueue(asserting_lit, Some(learned_id));
                self.decision_heuristic.on_conflict(&self.clause_db, learned_id, &bumped_vars);
                self.restart_policy.on_conflict(&self.clause_db, learned_id, lbd);

                if self.restart_policy.should_restart() {
                    self.restart();
                }
            } else {
                let next_lit = self.decision_heuristic.pick(&self.assignment);
                match next_lit {
                    Some(lit) => {
                        self.assignment.new_level();
                        self.assignment.enqueue(lit, None);
                    }
                    None => {
                        let res: Vec<bool> = (1..self.assignment.num_vars)
                            .map(|i| {
                                let lit = Literal::new(i as u32, true);
                                self.assignment.literal(lit) == LBool::True
                            }).collect();
                        return SolverResult::SAT(res);
                    }
                }
            }
        }
    }
}