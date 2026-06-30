use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::{Clause, ClauseID};
use crate::types::Literal;

pub struct ConflictHandler {
    seen: Vec<bool>,
    learnt: Vec<Literal>,
    bump_queue: Vec<u32>,
    level_seen: Vec<bool>,
}

pub struct ConflictResult {
    pub learned_clause: Clause,
    pub lbd: u32,
    pub jump_level: usize,
    pub bumped_vars: Vec<u32>
}

impl ConflictHandler {
    pub fn new(num_vars: usize) -> Self {
        Self {
            seen:       vec![false; num_vars + 1],
            learnt:     Vec::new(),
            bump_queue: Vec::new(),
            level_seen: vec![false; num_vars + 1],
        }
    }

    fn compute_lbd(&mut self, lits: &[Literal], assignment: &Assignment) -> u32 {
        let mut count = 0u32;
        let mut touched: Vec<usize> = Vec::new();

        for &lit in lits {
            let level = assignment.level[lit.variable()] as usize;
            if !self.level_seen[level] {
                self.level_seen[level] = true;
                touched.push(level);
                count += 1;
            }
        }

        for level in touched {
            self.level_seen[level] = false;
        }

        count
    }

    pub fn handle_conflict<C: ClauseDB>(&mut self, clause_db: &C, conflict_id: ClauseID, assignment: &Assignment) -> ConflictResult {
        let mut working_clause: Vec<bool> = vec![false; 2 * assignment.num_vars + 2];
        let mut last_lit_count = 0;

        self.bump_queue.clear();

        let current_level = assignment.trail_lim.len();

        for lit in &clause_db.get_clause(conflict_id).literals {
            working_clause[lit.0 as usize] = true;
            let var = lit.variable();

            if !self.seen[var] {
                self.seen[var] = true;
                self.bump_queue.push(var as u32);
            }

            if assignment.level[var] == current_level as i32 {
                last_lit_count += 1;
            }
        }

        let mut curr_trail_ind = assignment.trails.len() - 1;
        while last_lit_count > 1 {
            let curr_lit = assignment.trails[curr_trail_ind];
            let neg_lit = !curr_lit;
            let neg_lit_idx = neg_lit.0 as usize;

            if working_clause[neg_lit_idx] {
                let reason_clause = assignment.reason[curr_lit.variable()].unwrap();

                working_clause[neg_lit_idx] = false;
                last_lit_count -= 1;

                for &lit in &clause_db.get_clause(reason_clause).literals {
                    if lit == curr_lit {
                        continue;
                    }

                    if !working_clause[lit.0 as usize] {
                        working_clause[lit.0 as usize] = true;
                        if assignment.level[lit.variable()] == current_level as i32 {
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
                let curr_level = assignment.level[Literal(lit_ind as u32).variable()];

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

        for &var in &self.bump_queue {
            self.seen[var as usize] = false;
        }

        let lbd = self.compute_lbd(&temp_clause_vec, assignment);

        ConflictResult {
            learned_clause: Clause {literals: temp_clause_vec},
            lbd,
            jump_level: second_max_level as usize,
            bumped_vars: self.bump_queue.clone()
        }
    }
}