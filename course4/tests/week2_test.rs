extern crate course4;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::fs::{read_dir, DirEntry};
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

use course4::week2::solution::solve_for_file;
use course4::week2::types::TspResult;

const TEST_CASES_DIR: &str = "resources/week2/test_cases";

fn run_tests(files: impl Iterator<Item = Result<DirEntry>>, f: &dyn Fn(&str) -> TspResult) {
    lazy_static! {
        static ref RE_INPUT_FILENAME: Regex = Regex::new(r"^input_(?P<id>.*)\.txt$").unwrap();
    }

    files
        // Extract the file names from the dir entries.
        .map(|f| f.as_ref().unwrap().file_name().into_string().unwrap())
        // Filter those file names that match the input file name format.
        .filter(|f| RE_INPUT_FILENAME.is_match(f))
        // Transform into tuples with the format: (input file name, output file name).
        .map(|f| {
            let input = String::from(f);
            let output = String::from(RE_INPUT_FILENAME.replace_all(&input, "output_$id.txt"));
            return (input, output);
        })
        // Append the containing directory to each file name.
        .map(|(input, output)| {
            let full_input = format!("{}/{}", TEST_CASES_DIR, input);
            let full_output = format!("{}/{}", TEST_CASES_DIR, output);
            return (full_input, full_output);
        })
        // Run tests and assertions for each test file.
        .for_each(|(input, output)| {
            println!("Processing file {}", input);
            // Compute result for input file.
            let result = f(&input).to_string();
            // Read correct answer from output file.
            let ofd = File::open(output).unwrap();
            let reader = BufReader::new(ofd);
            let expected_result = reader.lines().nth(0).unwrap().unwrap();

            // Assert that both results match.
            assert_eq!(
                expected_result, result,
                "Incorrect result for test case {}",
                input
            );
        });
}

#[test]
fn solution_is_computed_correctly_tsp() {
    let result = read_dir(TEST_CASES_DIR);
    match result {
        Ok(files) => run_tests(files, &solve_for_file),
        Err(_) => println!("Directory not found: {}", TEST_CASES_DIR),
    }
}
