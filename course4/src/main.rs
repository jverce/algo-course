mod common;

use std::println;
use common::utils::read_lines;

fn main() {
    let filename = "../res/week1/g1.txt";
    let file_content = read_lines(filename);
    let file_size = file_content.len();
    println!("{}", file_size);
    println!("{}", file_content[2][2]);
}
