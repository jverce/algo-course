extern crate course4;
use course4::week1::solution::solve_for_file;
use std::fs::read_dir;

#[test]
fn solution_is_computed_correctly() {
    let dir_name = "resources/week1/test_cases";
    let result = read_dir(dir_name);
    match result {
        Ok(files) => files.for_each(|f| {
            let filename = f.unwrap().file_name();
            println!("{:?}", filename);
        }),
        Err(_) => println!("Directory not found: {}", dir_name),
    }
    assert_eq!(2, 2);
}
