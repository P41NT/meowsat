use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::clauses::clause_db::ClauseDB;
use crate::clauses::clause_type::Clause;
use crate::types::Literal;

// takes in filename as input, returns a clause_db based on the generic and the num_vars
pub fn dimacs_parse_from_file<Clauses: ClauseDB, P: AsRef<std::path::Path>>(path: P) -> (Clauses, usize) {
    let mut clause_db = Clauses::new();

    let f = match File::open(&path) {
        Err(why) => {
            let filename = path.as_ref()
                .file_name()
                .map(|os| os.to_string_lossy().into_owned())
                .unwrap_or_else(|| "".to_string());
            panic!("couldn't open {}: {}", filename, why)
        },
        Ok(f) => f,
    };

    let reader = BufReader::new(f);

    // comments_done denotes if the comments are exhausted in dimacs cnf format
    // when we reach a p in the cnf file, we start reading the actual literals and clauses
    let mut comments_done = false;
    let mut num_vars = 0;
    let mut num_clauses = 0;

    let mut curr_clauses = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        // if line is empty we skip
        if line.is_empty() {
            continue
        }

        // tokenize the input
        let mut tokens = line.split_whitespace().collect::<Vec<_>>();

        if comments_done {
            tokens.pop(); // the last element is just a 0 for termination, not required
            let clause_vec: Vec<Literal> = tokens.iter().map(|token| {
                let raw_lit: i32 = token.parse().unwrap();
                // dimacs cnf format uses negative numbers for false literals, we use that to create
                // the literal in our format (which uses the lsb to denote falseness)
                Literal::new(raw_lit.abs() as u32, raw_lit > 0)
            }).collect();
            clause_db.add_clause(Clause::new(clause_vec));

            curr_clauses += 1;
            if curr_clauses == num_clauses {
                break;
            }
        }
        else {
            match tokens[0].chars().nth(0) {
                // if we reach p, we start reading in the actual literals and clauses
                // therefore, we set the comments_done flag to true
                Some('p') => {
                    comments_done = true;
                    num_vars = tokens[2].parse().unwrap();
                    num_clauses = tokens[3].parse().unwrap();
                }
                _ => {}
            }
        }
    }

    (clause_db, num_vars)
}
