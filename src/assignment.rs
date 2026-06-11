use std::rc::Weak;
use crate::clauses::clause_type::{Clause, ClauseID};
use crate::types::{LBool, Literal};

pub struct Assignment {
    assignments: Vec<LBool>,
    trails: Vec<Literal>,
    trail_lim: Vec<usize>,
    reason: Vec<Option<ClauseID>>
}

impl Assignment {
    pub fn new(num_vars: usize) -> Self {
        Assignment {
            assignments: vec![LBool::Undef;num_vars],
            trails: Vec::new(),
            trail_lim: Vec::new(),
            reason: vec![None;num_vars]
        }
    }

    pub fn literal(&self, lit: Literal) -> LBool {
        let val = self.assignments[lit.variable()];
        if val == LBool::Undef || lit.sign(){
            val
        }else{
            -val
        }
    }

    pub fn enqueue(&mut self, lit: Literal, cause: Option<ClauseID>) {
        let var = lit.variable();
        self.assignment[var] = if lit.sign(){
            LBool::True
        } else{
            LBool::False
        };
        self.reason[var] = cause;
        self.trails.push(lit);
    }

    pub fn pop_to_level(&mut self, level: usize) {
        if level >= self.trail_lim.len(){
            return;
        }
        let llim = self.trail_lim[level];
        while self.trails.len() > llim{
            let lit = self.trails.pop().unwrap();
            self.assignments[lit.variable()] = LBool::Undef;
            self.reason[var] = None;
        }
        self.trail_lim.truncate(level);
    }

    pub fn new_level(&mut self){
        self.trail_lim.push(self.trails.len());
    }
}