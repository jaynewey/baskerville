use baskerville::{infer_csv_with_options, CsvInput, InferOptions};
use criterion::{criterion_group, criterion_main, Criterion};

#[inline]
fn csv_headers(path: &str) {
    infer_csv_with_options(
        CsvInput::Path(path),
        &mut InferOptions {
            has_headers: true,
            ..InferOptions::default()
        },
    )
    .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("5000_default", |b| {
        b.iter(|| csv_headers("./benches/5000_default.csv"))
    });

    c.bench_function("20000_default", |b| {
        b.iter(|| csv_headers("./benches/20000_default.csv"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
