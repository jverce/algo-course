extern crate course4;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs::File;
use std::fs::{read_dir, DirEntry};
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

use course4::common::types::Weight;
use course4::week1::solution::{solve_for_file_bf, solve_for_file_fw};

fn run_tests(files: impl Iterator<Item = Result<DirEntry>>, f: &dyn Fn(&str) -> Option<Weight>) {
    lazy_static! {
        static ref RE_INPUT_FILENAME: Regex =
            Regex::new(r"^input_random_(?P<id>.*)\.txt$").unwrap();
    }

    files
        // Extract the file names from the dir entries.
        .map(|f| f.as_ref().unwrap().file_name().into_string().unwrap())
        // Filter those file names that match the input file name format.
        .filter(|f| RE_INPUT_FILENAME.is_match(f))
        // Transform into tuples with the format: (input file name, output file name).
        .map(|f| {
            let input = String::from(f);
            let output =
                String::from(RE_INPUT_FILENAME.replace_all(&input, "output_random_$id.txt"));
            return (input, output);
        })
        // Append the containing directory to each file name.
        .map(|(input, output)| {
            let dir = String::from("resources/week1/test_cases");
            let full_input = format!("{}/{}", dir, input);
            let full_output = format!("{}/{}", dir, output);
            return (full_input, full_output);
        })
        // Run tests and assertions for each test file.
        .for_each(|(input, output)| {
            println!("Processing file {}", input);
            // Compute result for input file.
            let result = match f(&input) {
                Some(r) => (r as i64).to_string(),
                None => String::from("NULL"),
            };
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
fn solution_is_computed_correctly_bf() {
    let dir_name = "resources/week1/test_cases";
    let result = read_dir(dir_name);
    match result {
        Ok(files) => run_tests(files, &solve_for_file_bf),
        Err(_) => println!("Directory not found: {}", dir_name),
    }
    assert_eq!(2, 2);
}

#[test]
fn solution_is_computed_correctly_fw() {
    let dir_name = "resources/week1/test_cases";
    let result = read_dir(dir_name);
    match result {
        Ok(files) => run_tests(files, &solve_for_file_fw),
        Err(_) => println!("Directory not found: {}", dir_name),
    }
    assert_eq!(2, 2);
}
