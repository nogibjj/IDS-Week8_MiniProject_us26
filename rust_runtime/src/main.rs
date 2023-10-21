use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sum_of_squares(n: u64) -> u64 {
    (1..=n).map(|x| x * x).sum()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Rust Sum of Squares", |b| {
        b.iter(|| sum_of_squares(black_box(10_000_000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);