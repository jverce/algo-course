use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};

fn split_line(line: &str) -> Vec<&str> {
    return line.split(' ').collect();
}

fn convert_to_int(line: &str) -> Vec<i64> {
    let mut res: Vec<i64> = Vec::new();
    let split: Vec<&str> = split_line(line);
    for i in split.iter() {
        let as_int: i64 = i.parse::<i64>().unwrap();
        res.push(as_int);
    }
    return res;
}

pub fn read_lines(filename: &str) -> Vec<Vec<i64>> {
    let fd = File::open(filename).unwrap();
    let reader = BufReader::new(fd);
    let mut res: Vec<Vec<i64>> = Vec::new();
    for i in reader.lines() {
        let as_str: &str = &&i.unwrap();
        let as_int_vec = convert_to_int(as_str);
        res.push(as_int_vec);
    }
    return res;
}
