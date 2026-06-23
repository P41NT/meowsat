use crate::clauses::clause_type::ClauseID;
use crate::types::{LBool, Literal};

pub struct Assignment {
    pub num_vars: usize,
    assignments: Vec<LBool>,
    pub trails: Vec<Literal>,
    pub trail_lim: Vec<usize>,
    pub reason: Vec<Option<ClauseID>>,
    pub level: Vec<i32>,
}

impl Assignment {
    pub fn new(num_vars: usize) -> Self {
        let num_vars = num_vars + 1;
        Assignment {
            num_vars,
            assignments: vec![LBool::Undef; num_vars],
            trails: Vec::new(),
            trail_lim: Vec::new(),
            reason: vec![None; num_vars],
            level: vec![-1; num_vars],
        }
    }

    pub fn literal(&self, lit: Literal) -> LBool {
        let val = self.assignments[lit.variable()];
        if val == LBool::Undef || lit.sign() {
            val
        } else {
            -val
        }
    }

    pub fn enqueue(&mut self, lit: Literal, cause: Option<ClauseID>) {
        let var = lit.variable();
        self.assignments[var] = if lit.sign() {
            LBool::True
        } else {
            LBool::False
        };
        self.reason[var] = cause;
        self.trails.push(lit);
        self.level[var] = self.trail_lim.len() as i32;
    }

    pub fn pop_to_level(&mut self, level: usize) {
        if level >= self.trail_lim.len() {
            return;
        }
        let llim = self.trail_lim[level];
        while self.trails.len() > llim {
            let lit = self.trails.pop().unwrap();
            self.assignments[lit.variable()] = LBool::Undef;
            self.reason[lit.variable()] = None;
        }
        self.trail_lim.truncate(level);
    }

    pub fn new_level(&mut self) {
        self.trail_lim.push(self.trails.len());
    }
}
