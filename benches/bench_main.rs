use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod benchmarks;

pub fn p1(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        b.iter(|| {
            benchmarks::part1_me::exec(black_box(include_str!("../input.txt").lines().collect()))
        })
    });
}
pub fn p2(c: &mut Criterion) {
    c.bench_function("part2", |b| {
        b.iter(|| {
            benchmarks::part2_me::exec(black_box(include_str!("../input.txt").lines().collect()))
        })
    });
}
pub fn third_party(c: &mut Criterion) {
    c.bench_function("third_party", |b| {
        b.iter(|| {
            benchmarks::third_party::exec(black_box(include_str!("../input.txt").lines().collect()))
        })
    });
}

pub fn timv_1(c: &mut Criterion) {
    c.bench_function("timv_1", |b| {
        b.iter(|| benchmarks::timv_1::exec())
    });
}

pub fn timv_2(c: &mut Criterion) {
    c.bench_function("timv_2", |b| {
        b.iter(|| {
            benchmarks::timv_2::exec();
        })
    });
}

criterion_group! {
   name = benches;
   config =Criterion::default();
   targets =  p1, p2, third_party,timv_1,timv_2
}
criterion_main!(benches);
