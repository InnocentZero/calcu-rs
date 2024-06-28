use calcu_rs::parse;
use chrono::NaiveDate;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use std::str::FromStr;

pub fn parse_sequence_benchmark(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();

    c.bench_function("parse_sequence 20", |b| {
        b.iter(|| {
            parse::parse_sequence(
                black_box(&start_date),
                black_box(&end_date),
                black_box(PathBuf::from_str("tests").as_mut().unwrap()),
            )
        })
    });
}

pub fn parse_sequence_benchmark_100(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd_opt(2024, 8, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 8, 30).unwrap();

    c.bench_function("parse_sequence 100", |b| {
        b.iter(|| {
            parse::parse_sequence(
                black_box(&start_date),
                black_box(&end_date),
                black_box(PathBuf::from_str("tests").as_mut().unwrap()),
            )
        })
    });
}

pub fn parse_sequence_benchmark_300(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 1, 30).unwrap();

    c.bench_function("parse_sequence 300", |b| {
        b.iter(|| {
            parse::parse_sequence(
                black_box(&start_date),
                black_box(&end_date),
                black_box(PathBuf::from_str("tests").as_mut().unwrap()),
            )
        })
    });
}

pub fn parse_sequence_benchmark_700(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd_opt(2024, 9, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 9, 30).unwrap();

    c.bench_function("parse_sequence 700", |b| {
        b.iter(|| {
            parse::parse_sequence(
                black_box(&start_date),
                black_box(&end_date),
                black_box(PathBuf::from_str("tests").as_mut().unwrap()),
            )
        })
    });
}

pub fn parse_sequence_benchmark_long_file(c: &mut Criterion) {
    let start_date = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2024, 7, 30).unwrap();

    c.bench_function("parse_sequence 3000", |b| {
        b.iter(|| {
            parse::parse_sequence(
                black_box(&start_date),
                black_box(&end_date),
                black_box(PathBuf::from_str("tests").as_mut().unwrap()),
            )
        })
    });
}

criterion_group!(
    benches,
    parse_sequence_benchmark,
    parse_sequence_benchmark_100,
    parse_sequence_benchmark_300,
    parse_sequence_benchmark_700,
    parse_sequence_benchmark_long_file
);
criterion_main!(benches);
