extern crate course4;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use course4::week1::solution::solve_for_file;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("BF", |b| b.iter(|| solve_for_file(black_box("resources/week1/test_cases/input_random_25_128.txt"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);