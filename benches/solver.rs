use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use minesweeper_ng_gen::*;

fn load_field(index: u32) -> Option<DefinedField> {
    DefinedField::from_file(&format!(
        "generated/testing/benchmarking/{}.minesweeper",
        index
    ))
    .ok()
}

fn solver_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("solver");
    group.measurement_time(std::time::Duration::from_secs(10));

    for i in 1..=100 {
        if let Some(field) = load_field(i) {
            let field_id = format!("field_{:03}", i);
            group.bench_function(&field_id, |b| b.iter(|| is_solvable(black_box(&field))));
        }
    }

    group.finish();
}

fn evil_field_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("solver_evil");
    group.measurement_time(std::time::Duration::from_secs(10));

    if let Ok(field) = DefinedField::from_file("generated/testing/evil_ng_field.minesweeper") {
        group.bench_function("evil_ng_field", |b| {
            b.iter(|| is_solvable(black_box(&field)))
        });
    }

    if let Ok(field) = DefinedField::from_file("generated/testing/hard.minesweeper") {
        group.bench_function("hard_field", |b| {
            b.iter(|| is_solvable(black_box(&field)))
        });
    }

    group.finish();
}

fn ng_generation_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("ng_generation");
    group.measurement_time(std::time::Duration::from_secs(10));

    group.bench_function("beginner 9x9 10mines", |b| {
        b.iter(|| NoGuessField::new(black_box(9), black_box(9), black_box(Mines::Count(10))))
    });

    group.bench_function("intermediate 16x16 40mines", |b| {
        b.iter(|| NoGuessField::new(black_box(16), black_box(16), black_box(Mines::Count(40))))
    });

    group.bench_function("expert 30x16 99mines", |b| {
        b.iter(|| NoGuessField::new(black_box(30), black_box(16), black_box(Mines::Count(99))))
    });

    group.finish();
}

criterion_group!(benches, solver_benchmarks, evil_field_benchmark, ng_generation_benchmarks);
criterion_main!(benches);
