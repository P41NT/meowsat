use std::cmp::Ordering;
use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::ClauseID;
use crate::heuristics::decisions::DecisionHeuristic;
use crate::types::{LBool, Literal};

struct HeapEntry {
    variable: usize,
    score: f64,
}

impl PartialEq<Self> for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for HeapEntry { }

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct VSIDS {
    heap: std::collections::BinaryHeap<HeapEntry>,
    conflict_counter: u32,
    conflict_interval: u32,
    bump: f64,
    decay: f64,
    scores: Vec<f64>,
    phases: Vec<bool>
}

impl VSIDS {
    pub fn new(num_vars: usize, bump: f64, decay: f64) -> VSIDS {
        let mut heap = std::collections::BinaryHeap::new();
        let scores = vec![0.0; num_vars + 1];
        let phases = vec![false; num_vars + 1];
        for var in 0..num_vars {
            heap.push(HeapEntry { variable: var, score: scores[var] });
        }
        Self { heap, conflict_counter: 0, conflict_interval: 256, bump, decay, scores, phases }
    }
}

impl<C: ClauseDB> DecisionHeuristic<C> for VSIDS {
    fn pick(&mut self, assignment: &Assignment) -> Option<Literal> {
        loop {
            let picked = self.heap.pop()?;
            if picked.score - self.scores[picked.variable] > 1e-8 {
                continue;
            }
            if assignment.literal(Literal::new(picked.variable as u32, true)) != LBool::Undef {
                continue;
            }
            let polarity = self.phases[picked.variable];
            return Some(Literal::new(picked.variable as u32, polarity));
        }
    }
    fn on_conflict(&mut self, clause_db: &C, clause_id: ClauseID, bumped_vars: &[u32]) {
        let learned_clause = clause_db.get_clause(clause_id);
        for &lit in &learned_clause.literals {
            let v = lit.variable();
            self.scores[v] += self.bump;
            self.heap.push(HeapEntry { variable: v, score: self.scores[v] });
        }
        self.conflict_counter += 1;
        if self.conflict_counter % self.conflict_interval == 0 {
            for score in &mut self.scores {
                *score /= 2.0;
            }
        }
    }

    fn on_unassign(&mut self, lit: Literal) {
        let var = lit.variable();
        self.phases[var] = lit.sign();
    }

    fn on_restart(&mut self) {}
}