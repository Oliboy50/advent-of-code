use adventofcode_2024_rust::{day01, get_lines_from};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = get_lines_from("src/day01/input");
    c.bench_function("day01-part2", |b| {
        b.iter(|| day01::part2(black_box(input.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
