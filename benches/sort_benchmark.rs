use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_structure_algorithms::sort::*;
use rand::{seq::SliceRandom, thread_rng};

fn bubble_sort_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut items: Vec<usize> = (1..=10000).collect();
    items.shuffle(&mut rng);

    let mut arr = black_box(items);

    c.bench_function("Bubble Sort", |b| b.iter(|| bubble(&mut arr)));
}

fn counting_sort_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut items: Vec<usize> = (1..=10000).collect();
    items.shuffle(&mut rng);

    let mut arr = black_box(items);

    c.bench_function("Counting Sort", |b| b.iter(|| counting(&mut arr)));
}

criterion_group!(benches, bubble_sort_benchmark, counting_sort_benchmark);
criterion_main!(benches);
