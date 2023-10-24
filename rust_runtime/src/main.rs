use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Instant;

fn sum_of_squares(n: u64) -> u64 {
    (1..=n).map(|x| x * x).sum()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Rust Sum of Squares", |b| {
        b.iter(|| {
            let start_time = Instant::now();
            let result = sum_of_squares(black_box(10_000_000));
            let end_time = Instant::now();

            let elapsed_time = end_time - start_time;
            println!("Start time: {:?}", start_time);
            println!("End time:   {:?}", end_time);
            println!("Elapsed time: {:?}", elapsed_time);
            black_box(result);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);