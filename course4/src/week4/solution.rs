use itertools::Itertools;

pub fn solve_for_file(filename: &str) -> bool {
    return false;
}

pub fn solve() {
    let filenames = (1..=6).map(|i| format!("resources/week4/2sat{}.txt", i));
    let result = filenames
        .map(|f| solve_for_file(&f))
        .map(|r| r.into())
        .map(|r: i32| r.to_string())
        .join("");
    println!("{}", result);
}
