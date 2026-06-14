use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::{Clause, ClauseID};
use crate::propagate::propagator::{Propagator, Watcher};
use crate::types::{LBool, Literal};

pub struct SimpleBCP {
    watches: Vec<Vec<Watcher>>,
    clause_watches: Vec<(Literal, Literal)>,
    pub assign_head: usize
}

impl SimpleBCP {
    fn lit_idx(lit: Literal) -> usize {
         (lit.variable() << 1) | (lit.sign() as usize)
    }
    fn get_watches(&mut self, lit: Literal) -> &mut [Watcher] {
        let lit_idx = (lit.variable() << 1) | (lit.sign() as usize);
        self.watches[lit_idx].as_mut_slice()
    }
}

impl<Clauses: ClauseDB> Propagator<Clauses> for SimpleBCP {
    fn new(clause_db: &Clauses, assignment: &mut Assignment) -> Self {
        let mut temp = Self {
            watches: vec![vec![]; 2 * assignment.num_vars + 2],
            clause_watches: vec![(Literal(0), Literal(0)); clause_db.num_clauses()],
            assign_head: 0
        };
        for clause_id in 0..clause_db.num_clauses() {
            let clause_id_strong = ClauseID(clause_id);
            let curr_clause: &Clause  = &clause_db.get_clause(clause_id_strong);
            if curr_clause.len() == 1 {
                assignment.enqueue(curr_clause.literals[0], None);
            }
            else {
                let lit1 = curr_clause.literals[0];
                let lit2 = curr_clause.literals[1];

                temp.clause_watches[clause_id] = (lit1, lit2);

                temp.watches[Self::lit_idx(lit1)].push(Watcher{ clause_id: clause_id_strong, other_lit: lit2 });
                temp.watches[Self::lit_idx(lit2)].push(Watcher{ clause_id: clause_id_strong, other_lit: lit1 });
            }
        }
        temp
    }

    fn propagate(&mut self, assignment: &mut Assignment, clause_db: &Clauses) -> Option<ClauseID> {

        while self.assign_head < assignment.trails.len() {
            let curr_lit = assignment.trails[self.assign_head];
            self.assign_head += 1;

            let opp_lit = !curr_lit;
            let opp_idx = Self::lit_idx(opp_lit);

            // swaps out the watches array, this means we will have to push multiple times
            // in the hot path to maintain the original array which is not ideal.
            // But, since this is the naive implementation, this is okay.
            // This will be a more optimized version in the actual propagator using unsafe{} or
            // some other technique for performance reasons

            let opp_watches = std::mem::take(&mut self.watches[opp_idx]);

            let mut watch_idx = 0;
            let mut conflict: Option<ClauseID> = None;

            while watch_idx < opp_watches.len() {
                // for each clause, get the other watched literal (according to 2WL) and clause_id
                let Watcher {clause_id, other_lit: cached_other } = opp_watches[watch_idx];
                watch_idx += 1;

                // if the other literal in 2WL is true, then we keep the watch and continue as the
                // clause is satisfied already
                if assignment.literal(cached_other) == LBool::True {
                    self.watches[opp_idx].push(Watcher {clause_id, other_lit: cached_other });
                    continue;
                }

                // else we search through the entire clause to find a replacement for the opposite
                // literal to watch
                let (w1, w2) = self.clause_watches[clause_id.0];
                let true_other_lit = if w1 == opp_lit { w2 } else { w1 };

                if assignment.literal(true_other_lit) == LBool::True {
                    self.watches[opp_idx].push(Watcher {clause_id, other_lit: true_other_lit });
                    continue;
                }

                let curr_clause = clause_db.get_clause(clause_id);
                let mut found_replacement = false;

                for &lit in curr_clause.literals.iter() {
                    if lit == opp_lit || lit == true_other_lit || assignment.literal(lit) == LBool::False {
                        // the replacement must not be the opp_lit or other_lit, and it should not
                        // be false, either undef or true
                        continue
                    }
                    else {
                        // the following block is for when new_lit is the replacement for the opp_lit
                        let new_lit = lit;
                        let new_idx = Self::lit_idx(new_lit);

                        self.clause_watches[clause_id.0] = (new_lit, true_other_lit);

                        // we push the identical stuff from opp_lit to this new lit's index in watches
                        self.watches[new_idx].push( Watcher { clause_id, other_lit: true_other_lit } );
                        found_replacement = true;
                        break;
                    }
                }

                // if we have not found a replacement, this means it is either a conflict or a unit
                // clause with only other_lit in it.
                if !found_replacement {
                    // we keep the current clause as we have not found a replacement
                    self.watches[opp_idx].push(Watcher {clause_id, other_lit: true_other_lit });
                    // if other_lit is false, then this is a conflict of course.
                    if assignment.literal(true_other_lit) == LBool::False {
                        // we set conflict to this clause_id which will be returned later
                        conflict = Some(clause_id);
                        break;
                    }
                    else {
                        // else this is a unit clause, so we enqueue it into the same assignment
                        // structure
                        assignment.enqueue(true_other_lit, Some(clause_id));
                    }
                }
            }

            // if the scan was cut short because of a conflict, we add all the remaining watches
            while watch_idx < opp_watches.len() {
                self.watches[opp_idx].push(opp_watches[watch_idx]);
                watch_idx += 1;
            }

            // if a conflict was found, we return it
            if conflict.is_some() {
                return conflict;
            }
        }
        None
    }

    fn reset_head(&mut self, level: usize) {
        self.assign_head = level;
    }
}