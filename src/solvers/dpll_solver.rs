use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::propagate::propagator::Propagator;
use crate::solvers::solver::Solver;
use crate::types::{LBool, Literal};

struct DPLLSolver<Clauses: ClauseDB, Prop: Propagator<Clauses>> {
    assignments: Assignment,
    clause_db: Clauses,
    prop: Prop,
}

impl<Clauses: ClauseDB, Prop: Propagator<Clauses>> Solver<Clauses, Prop> for DPLLSolver<Clauses, Prop> {
    fn new(clause_db: Clauses, prop: Prop, assignment: Assignment) -> Self {
        Self {
            assignments: assignment,
            clause_db,
            prop,
        }
    }

    fn solve(&mut self) -> Option<Vec<bool>> {
        let mut dec_stack: Vec<(Literal, bool)> = Vec::new();
        loop {
            let conflict = self.prop.propagate(&mut self.assignments, &self.clause_db);
            if conflict.is_some() {
                if dec_stack.is_empty() {
                    return None;
                }

                loop {
                    if dec_stack.is_empty() {
                        return None;
                    }

                    let (last_lit, flipped) = dec_stack.pop().unwrap();
                    self.assignments.pop_to_level(dec_stack.len());
                    if !flipped {                
                        self.assignments.new_level();
                        self.assignments.enqueue(!last_lit, None);
                        dec_stack.push((!last_lit, true));   
                        break;
                    }
                }
            }
            else {
                let mut unassigned = None;
                for i in 0..self.assignments.num_vars {
                    let lit = Literal::new(i as u32, true);
                    if self.assignments.literal(lit) == LBool::Undef {
                        unassigned = Some(i as u32);
                        break;
                    }
                }

                match unassigned {
                    Some(var) => {
                        let lit = Literal::new(var, true);
                        self.assignments.new_level();
                        self.assignments.enqueue(lit, None);
                        dec_stack.push((lit, false));
                    }
                    None => {
                        let mut res = Vec::with_capacity(self.assignments.num_vars);
                        for i in 0..self.assignments.num_vars {
                            let lit = Literal::new(i as u32, true);
                            res.push(self.assignments.literal(lit) == LBool::True);
                        }
                        return Some(res);
                    }
                }
            }
        }
    }
}