mod clauses;
mod types;
mod assignment;
mod propagate;
mod solvers;
mod parser;
mod tester;
mod heuristics;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use crate::tester::test_basic_solver;

fn main() {
    let root_dir = Path::new("testcases");

    let total_time = Instant::now();

    if let Ok(folders) = fs::read_dir(root_dir) {
        for folder in folders.flatten() {
            if folder.path().is_dir() {
                let folder_name = folder.file_name().into_string().unwrap();
                let folder_time = Instant::now();
                let mut files_processed = 0;

                if let Ok(files) = fs::read_dir(folder.path()) {
                    for test_file in files.flatten() {
                        test_basic_solver(test_file.path(), true);
                        // test_solver_old::<SimpleClauseDB, SimpleBCP, CDCLSolver<SimpleClauseDB, SimpleBCP>, PathBuf>(test_file.path(), false);
                        files_processed += 1;
                    }
                }

                let folder_duration = folder_time.elapsed();
                println!("{}: Processed {} Files, Elapsed Time: {:?}", folder_name, files_processed, folder_duration);
            }
        }
    }

    let duration = total_time.elapsed();
    println!("Total Time Taken: {:?}", duration);
}
