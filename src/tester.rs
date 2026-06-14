use crate::assignment::Assignment;
use crate::clauses::clause_db::ClauseDB;
use crate::solvers::solver::Solver;
use crate::parser::dimacs_parse_from_file;
use crate::propagate::propagator::Propagator;

use std::time::Instant;

pub fn test_solver<Clauses, Prop, Solve, P>(test_file: P)
where
    Clauses: ClauseDB,
    Prop: Propagator<Clauses>,
    Solve: Solver<Clauses, Prop>,
    P: AsRef<std::path::Path>
{

    let filename = test_file.as_ref().file_name()
        .map(|os| os.to_string_lossy().into_owned())
        .unwrap_or_else(|| "".to_string());

    let (clause_db, num_vars) = dimacs_parse_from_file::<Clauses, P>(test_file);
    let mut assignments = Assignment::new(num_vars);

    let propagator = Prop::new(&clause_db, &mut assignments);
    let mut solver = Solve::new(clause_db, propagator, assignments);

    let start = Instant::now();
    let result = solver.solve();
    let duration = start.elapsed();

    let verdict = match result {
        Some(_) => "SAT",
        None => "UNSAT",
    };

    println!("{}\t\t{:?}\t\t{}", filename, duration, verdict);
}
