use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::propagate::propagator::Propagator;
use crate::solvers::solver::Solver;
use crate::types::{LBool, Literal};

pub struct DPLLSolver<Clauses: ClauseDB, Prop: Propagator<Clauses>> {
    assignment: Assignment,
    clause_db: Clauses,
    prop: Prop,
}

impl<Clauses: ClauseDB, Prop: Propagator<Clauses>> Solver<Clauses, Prop> for DPLLSolver<Clauses, Prop> {
    fn new(clause_db: Clauses, prop: Prop, assignment: Assignment) -> Self {
        Self {
            assignment,
            clause_db,
            prop,
        }
    }

    fn solve(&mut self) -> Option<Vec<bool>> {
        // decision stack that stores the decisions that are made at each level
        // propagation decisions are not stored here.
        let mut dec_stack: Vec<(Literal, bool)> = Vec::new();
        loop {
            let conflict = self.prop.propagate(&mut self.assignment, &self.clause_db);
            if conflict.is_some() {
                // if there was a conflict in the propagate step, we check if any decision has been made.
                if dec_stack.is_empty() {
                    // if there was no decision to be made, it is definitely UNSAT
                    return None;
                }

                loop {
                    if dec_stack.is_empty() {
                        return None;
                    }

                    // we loop through, and find the last decision that was made which led to this
                    // conflict, and we change it.
                    let (last_lit, flipped) = dec_stack.pop().unwrap();
                    self.assignment.pop_to_level(dec_stack.len());
                    self.prop.reset_head(self.assignment.trails.len());
                    // if we haven't tried both true and false cases, we try that first
                    if !flipped {                
                        self.assignment.new_level();
                        self.assignment.enqueue(!last_lit, None);
                        dec_stack.push((!last_lit, true));   
                        break;
                    }
                }
            }
            else {
                // we find the next unassigned value in assignments, and assign it true / false
                // and check if it leads to conflicts.
                let mut unassigned = None;
                for i in 0..self.assignment.num_vars {
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
                        dec_stack.push((lit, false));
                    }
                    None => {
                        // if no conflict was found, we have found an assignment that
                        // satisfies all the clauses.
                        let mut res = Vec::with_capacity(self.assignment.num_vars);
                        for i in 0..self.assignment.num_vars {
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