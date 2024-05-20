use criterion::{criterion_group, criterion_main};
use criterion::{AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput};
use rand::prelude::*;
use rand_distr::LogNormal;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::time::Duration;

use compensated_summation::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("compensated summation");

    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let values: Vec<f64> = rng
        .sample_iter(LogNormal::new(0.0, 40.0).unwrap())
        .take(1_000_000)
        .collect();

    for n in [1, 10, 100, 10_000, 1_000_000] {
        group.throughput(Throughput::Elements(n as u64));

        group.bench_with_input(
            BenchmarkId::new("naive", n),
            &values[0..n],
            |b, slice: &[f64]| b.iter(|| slice.iter().sum::<f64>()),
        );

        group.bench_with_input(
            BenchmarkId::new("Kahan-Babuska", n),
            &values[0..n],
            |b, slice: &[f64]| b.iter(|| slice.iter().sum::<KahanBabuska<f64>>().total()),
        );

        group.bench_with_input(
            BenchmarkId::new("kahan_babuska_sum", n),
            &values[0..n],
            |b, slice: &[f64]| b.iter(|| dev::kahan_babuska_sum(slice.iter().cloned())),
        );

        group.bench_with_input(
            BenchmarkId::new("Kahan-Babuska-Neumaier", n),
            &values[0..n],
            |b, slice: &[f64]| b.iter(|| slice.iter().sum::<KahanBabuskaNeumaier<f64>>().total()),
        );

        group.bench_with_input(
            BenchmarkId::new("kahan_babuska_neumaier_sum", n),
            &values[0..n],
            |b, slice: &[f64]| b.iter(|| dev::kahan_babuska_neumaier_sum(slice.iter().cloned())),
        );
        group.bench_with_input(
            BenchmarkId::new("kahan_babuska_neumaier_abs_sum", n),
            &values[0..n],
            |b, slice: &[f64]| {
                b.iter(|| dev::kahan_babuska_neumaier_abs_sum(slice.iter().cloned()))
            },
        );
        group.bench_with_input(
            BenchmarkId::new("kahan_babuska_neumaier_abs_two_sum", n),
            &values[0..n],
            |b, slice: &[f64]| {
                b.iter(|| dev::kahan_babuska_neumaier_abs_two_sum(slice.iter().cloned()))
            },
        );
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs_f64(0.25))
        .measurement_time(Duration::from_secs_f64(0.5));
    targets = criterion_benchmark
}
criterion_main!(benches);
