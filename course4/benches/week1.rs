extern crate course4;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use course4::week1::solution::{solve_for_file_bf, solve_for_file_fw};

fn criterion_benchmark(c: &mut Criterion) {
    let test_filename = "resources/week1/test_cases/input_random_25_128.txt";
    c.bench_function("BF", |b| {
        b.iter(|| solve_for_file_bf(black_box(test_filename)))
    });
    c.bench_function("FW", |b| {
        b.iter(|| solve_for_file_fw(black_box(test_filename)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
