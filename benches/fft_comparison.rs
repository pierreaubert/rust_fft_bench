use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fft_bench::benchmarks::*;

fn benchmark_rustfft_complex32(c: &mut Criterion) {
    let mut group = c.benchmark_group("rustfft_complex32");

    for size in [1024, 2048, 4096, 8192].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| rustfft_complex32(black_box(size)));
        });
    }

    group.finish();
}

fn benchmark_rustfft_complex64(c: &mut Criterion) {
    let mut group = c.benchmark_group("rustfft_complex64");

    for size in [1024, 2048, 4096, 8192].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| rustfft_complex64(black_box(size)));
        });
    }

    group.finish();
}

fn benchmark_scirs_complex32(c: &mut Criterion) {
    let mut group = c.benchmark_group("scirs_complex32");

    for size in [1024, 2048, 4096, 8192].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| scirs_complex32(black_box(size)));
        });
    }

    group.finish();
}

fn benchmark_scirs_complex64(c: &mut Criterion) {
    let mut group = c.benchmark_group("scirs_complex64");

    for size in [1024, 2048, 4096, 8192].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| scirs_complex64(black_box(size)));
        });
    }

    group.finish();
}

fn benchmark_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("fft_comparison_4096");
    let size = 4096;

    group.bench_function("rustfft_f32", |b| {
        b.iter(|| rustfft_complex32(black_box(size)));
    });

    group.bench_function("rustfft_f64", |b| {
        b.iter(|| rustfft_complex64(black_box(size)));
    });

    group.bench_function("scirs_f32", |b| {
        b.iter(|| scirs_complex32(black_box(size)));
    });

    group.bench_function("scirs_f64", |b| {
        b.iter(|| scirs_complex64(black_box(size)));
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_rustfft_complex32,
    benchmark_rustfft_complex64,
    benchmark_scirs_complex32,
    benchmark_scirs_complex64,
    benchmark_comparison
);
criterion_main!(benches);
