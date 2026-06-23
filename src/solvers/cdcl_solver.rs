use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::{Clause, ClauseID};
use crate::propagate::propagator::Propagator;
use crate::solvers::solver::Solver;
use crate::types::{LBool, Literal};

pub struct CDCLSolver<Clauses: ClauseDB, Prop: Propagator<Clauses>> {
    assignment: Assignment,
    clause_db: Clauses,
    prop: Prop,
}

impl<Clauses: ClauseDB, Prop: Propagator<Clauses>> CDCLSolver<Clauses, Prop> {
    fn learn_clause(&mut self, clause_id: ClauseID) -> (Clause, usize) {
        let mut working_clause: Vec<bool> = vec![false; 2 * self.assignment.num_vars + 2];
        let mut last_lit_count = 0;

        let current_level = self.assignment.trail_lim.len();

        for lit in &self.clause_db.get_clause(clause_id).literals {
            working_clause[lit.0 as usize] = true;
            if self.assignment.level[lit.variable()] == current_level as i32 {
                last_lit_count += 1;
            }
        }

        let mut curr_trail_ind = self.assignment.trails.len() - 1;
        while last_lit_count > 1 {
            let curr_lit = self.assignment.trails[curr_trail_ind];
            let neg_lit = !curr_lit;
            let neg_lit_idx = neg_lit.0 as usize;

            if working_clause[neg_lit_idx] {
                let reason_clause = self.assignment.reason[curr_lit.variable()].unwrap();

                working_clause[neg_lit_idx] = false;
                last_lit_count -= 1;

                for &lit in &self.clause_db.get_clause(reason_clause).literals {
                    if lit == curr_lit {
                        continue;
                    }

                    if !working_clause[lit.0 as usize] {
                        working_clause[lit.0 as usize] = true;
                        if self.assignment.level[lit.variable()] == current_level as i32 {
                            last_lit_count += 1;
                        }
                    }
                }
            }
            curr_trail_ind -= 1;
        }

        let mut temp_clause_vec: Vec<Literal> = Vec::new();

        let mut max_level = 0;
        let mut second_max_level = 0;
        let mut asserting_lit = None;
        let mut backtrack_lit = None;

        for (lit_ind, &in_clause) in working_clause.iter().enumerate() {
            if in_clause {
                let lit = Literal(lit_ind as u32);
                let curr_level = self.assignment.level[Literal(lit_ind as u32).variable()];

                if curr_level > max_level {
                    second_max_level = max_level;
                    max_level = curr_level;
                    backtrack_lit = asserting_lit;
                    asserting_lit = Some(lit);
                } else if curr_level > second_max_level {
                    second_max_level = curr_level;
                    backtrack_lit = Some(lit);
                }
                else {
                    temp_clause_vec.push(lit);
                }
            }
        }

        temp_clause_vec.push(asserting_lit.unwrap());
        let num_lits = temp_clause_vec.len();
        temp_clause_vec.swap(0, num_lits - 1);

        if backtrack_lit.is_some() {
            temp_clause_vec.push(backtrack_lit.unwrap());
            let num_lits = temp_clause_vec.len();
            temp_clause_vec.swap(1, num_lits - 1);
        }

        (
            Clause {
                literals: temp_clause_vec,
            },
            second_max_level as usize,
        )
    }
}

impl<Clauses: ClauseDB, Prop: Propagator<Clauses>> Solver<Clauses, Prop> for CDCLSolver<Clauses, Prop> {
    fn new(clause_db: Clauses, prop: Prop, assignment: Assignment) -> Self {
        Self {
            assignment,
            clause_db,
            prop,
        }
    }

    fn solve(&mut self) -> Option<Vec<bool>> {
        loop {
            let conflict = self.prop.propagate(&mut self.assignment, &self.clause_db);
            if let Some(conflict) = conflict {
                // if there was a conflict in the propagate step, we check if any decision has been made.
                if self.assignment.trail_lim.is_empty() {
                    return None;
                }
                let (learned_clause, level_to_pop) = self.learn_clause(conflict);
                let asserting_lit = learned_clause.literals[0];
                let learned_clause_id = self.clause_db.add_clause(learned_clause);
                self.prop.add_clause(learned_clause_id, &self.clause_db);
                self.assignment.pop_to_level(level_to_pop);
                self.prop.reset_head(self.assignment.trails.len());
                self.assignment.enqueue(asserting_lit, Some(learned_clause_id));
            } else {
                // we find the next unassigned value in assignments, and assign it true / false
                // and check if it leads to conflicts.
                let mut unassigned = None;
                for i in 1..self.assignment.num_vars {
                    let lit = Literal::new(i as u32, true);
                    if self.assignment.literal(lit) == LBool::Undef {
                        unassigned = Some(i as u32);
                        break;
                    }
                }

                match unassigned {
                    Some(var) => {
                        let lit = Literal::new(var, true);
                        self.assignment.new_level();
                        self.assignment.enqueue(lit, None);
                    }
                    None => {
                        // if no conflict was found, we have found an assignment that
                        // satisfies all the clauses.
                        let mut res = Vec::with_capacity(self.assignment.num_vars);
                        for i in 1..self.assignment.num_vars {
                            let lit = Literal::new(i as u32, true);
                            res.push(self.assignment.literal(lit) == LBool::True);
                        }
                        return Some(res);
                    }
                }
            }
        }
    }
}
