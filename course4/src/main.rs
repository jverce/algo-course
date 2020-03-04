mod common;
mod week1;

use week1::solution::solve;

fn main() {
    let result = solve();
    match result {
        Some(r) => println!("{}", r),
        None => println!("NULL"),
    };
}
