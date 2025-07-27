use criterion::{Criterion, black_box, criterion_group, criterion_main};
use lib_with_tests::{example_arrays, fast_example_arrays};

fn benchmark_example_arrays(c: &mut Criterion) {
    c.bench_function("example_arrays", |b| {
        b.iter(|| {
            // black_box prevents the compiler from optimizing away the function call
            black_box(example_arrays())
        })
    });
}

fn benchmark_fast_example_arrays(c: &mut Criterion) {
    c.bench_function("fast_example_arrays", |b| {
        b.iter(|| black_box(fast_example_arrays()))
    });
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_functions");

    group.bench_function("example_arrays", |b| b.iter(|| black_box(example_arrays())));

    group.bench_function("fast_example_arrays", |b| {
        b.iter(|| black_box(fast_example_arrays()))
    });

    group.finish();
}

// Register the benchmark functions
criterion_group!(
    benches,
    benchmark_example_arrays,
    benchmark_fast_example_arrays,
    benchmark_comparison
);
criterion_main!(benches);
