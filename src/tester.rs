use crate::solvers::cdcl_solver::{CDCLSolver, SolverConfig};
use std::time::Instant;
use crate::assignment::Assignment;
use crate::parser::dimacs_parse_from_file;
use crate::clauses::clause_type;
use crate::clauses::simple_clause_db::SimpleClauseDB;
use crate::heuristics::decisions::DecisionHeuristic;
use crate::heuristics::decisions::vsids::VSIDS;
use crate::heuristics::restarts::no_restart::NoRestart;
use crate::propagate::propagator::Propagator;
use crate::propagate::simple_bcp::SimpleBCP;
use crate::solvers::cdcl_solver::SolverResult::{SAT, UNSAT};
use crate::solvers::conflict_handler::ConflictHandler;

pub fn test_basic_solver<P: AsRef<std::path::Path>>(test_file: P, print_res: bool) {

    struct TestSolverConfig;
    impl SolverConfig for TestSolverConfig {
        type DB = SimpleClauseDB;
        type Propagator = SimpleBCP;
        type Heuristic = VSIDS;
        type Restarts = NoRestart;
    }

    let filename = test_file.as_ref().file_name()
        .map(|os| os.to_string_lossy().into_owned())
        .unwrap_or_else(|| "".to_string());

    let (clause_db, num_vars) = dimacs_parse_from_file::<SimpleClauseDB, P>(test_file);
    let mut assignments = Assignment::new(num_vars);
    let propagator = SimpleBCP::new(&clause_db, &mut assignments);
    let decision_heuristic = VSIDS::new(num_vars, 20f64, 5f64);
    let restart_policy = NoRestart;
    let conflict_handler = ConflictHandler::new(num_vars);

    let mut solver: CDCLSolver<TestSolverConfig> = CDCLSolver::new(assignments, clause_db, propagator, decision_heuristic, restart_policy, conflict_handler);

    let start = Instant::now();
    let result = solver.solve();
    let duration = start.elapsed();

    let verdict = match result {
        SAT(_) => "SAT",
        UNSAT => "UNSAT",
    };

    if print_res {
        println!("{}\t\t{:?}\t\t{}", filename, duration, verdict);
    }
}

// pub fn test_solver<C: SolverConfig, P: AsRef<std::path::Path>>(test_file: P, print_res: bool) {
//     let filename = test_file.as_ref().file_name()
//         .map(|os| os.to_string_lossy().into_owned())
//         .unwrap_or_else(|| "".to_string());
//
//     let (clause_db, num_vars) = dimacs_parse_from_file::<C::DB, P>(test_file);
//     let mut assignments = Assignment::new(num_vars);
//
//     let propagator = C::Propagator::new(&clause_db, &mut assignments);
//     let decision_heuristic = C::
//     let mut solver = CDCLSolver::
// }

// pub fn test_solver_old<Clauses, Prop, Solve, P>(test_file: P, print_res: bool)
// where
//     Clauses: ClauseDB,
//     Prop: Propagator<Clauses>,
//     Solve: SolverOld<Clauses, Prop>,
//     P: AsRef<std::path::Path>
// {
//
//     let filename = test_file.as_ref().file_name()
//         .map(|os| os.to_string_lossy().into_owned())
//         .unwrap_or_else(|| "".to_string());
//
//     let (clause_db, num_vars) = dimacs_parse_from_file::<Clauses, P>(test_file);
//     let mut assignments = Assignment::new(num_vars);
//
//     let propagator = Prop::new(&clause_db, &mut assignments);
//     let mut solver = Solve::new(clause_db, propagator, assignments);
//
//     let start = Instant::now();
//     let result = solver.solve();
//     let duration = start.elapsed();
//
//     let verdict = match result {
//         Some(_) => "SAT",
//         None => "UNSAT",
//     };
//
//     if print_res {
//         println!("{}\t\t{:?}\t\t{}", filename, duration, verdict);
//     }
// }
