mod clauses;
mod types;
mod assignment;
mod propagate;
mod solvers;
mod parser;
mod tester;

use std::fs;
use std::path::PathBuf;
use crate::tester::test_solver;
use crate::clauses::simple_clause_db::SimpleClauseDB;
use crate::propagate::simple_bcp::SimpleBCP;
use crate::solvers::dpll_solver::DPLLSolver;

fn main() {
    let test_dir = fs::read_dir("C:\\Users\\shawn\\RustroverProjects\\meowsat\\testcases\\uf50-218");

    for test_file in test_dir.unwrap() {
        let test_file = test_file.unwrap();
        let test_path = test_file.path();
        test_solver::<SimpleClauseDB, SimpleBCP, DPLLSolver<SimpleClauseDB, SimpleBCP>, PathBuf>(test_path);
    }

}
