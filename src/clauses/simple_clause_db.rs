use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::{Clause, ClauseID};

struct SimpleClauseDB {
    clauses: Vec<Clause>,
}

impl SimpleClauseDB {
    pub fn new() -> SimpleClauseDB {
        SimpleClauseDB { clauses: Vec::new() }
    }
}

impl ClauseDB for SimpleClauseDB {
    fn add_clause(&mut self, clause: Clause) -> ClauseID {
        self.clauses.push(clause);
        ClauseID(self.clauses.len() - 1)
    }
    fn get_clause(&self, id: ClauseID) -> &Clause {
        &self.clauses[id.0]
    }
    fn is_satisfied(&self, assignment: &Assignment) -> bool {
        self.clauses.iter().all(|clause| clause.is_satisfied(assignment))
    }
    fn is_unsatisfied(&self, assignment: &Assignment) -> bool {
        self.clauses.iter().any(|clause| clause.is_unsatisfied(assignment))
    }
    fn num_clauses(&self) -> usize {
        self.clauses.len()
    }
}