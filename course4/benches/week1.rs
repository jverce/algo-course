extern crate course4;

use course4::week1::solution::solve_for_file_bf;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("BF", |b| {
        b.iter(|| {
            solve_for_file_bf(black_box(
                "resources/week1/test_cases/input_random_25_128.txt",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
