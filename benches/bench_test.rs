use criterion::{criterion_group, criterion_main, Criterion};
use demo_oracles::average_btcusd::cache_mode; // Adjust the import path based on your project structure

fn cache_mode_benchmark(c: &mut Criterion) {
    c.bench_function("cache_mode", |b| {
        b.iter(|| {
            // Replace this block with the actual code you want to benchmark
            let duration_seconds = 9;
            let _ = cache_mode(duration_seconds);
        });
    });
}

criterion_group!(benches, cache_mode_benchmark);
criterion_main!(benches);
// Add more benchmark functions as needed
// criterion_group!(another_benchmark_group, another_benchmark_function);
