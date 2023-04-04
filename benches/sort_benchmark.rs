use criterion::{black_box, criterion_group, criterion_main, Criterion};
use data_structure_algorithms::sort::*;
use rand::prelude::*;

fn bubble_sort_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut items: Vec<usize> = (0..=10000).collect();
    items.shuffle(&mut rng);

    let mut arr = black_box(items);

    c.bench_function("Bubble Sort", |b| b.iter(|| bubble(&mut arr)));
}

fn counting_sort_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut items: Vec<usize> = (1..=10000).collect();
    items.shuffle(&mut rng);
    let mut arr = black_box(items);

    c.bench_function("Counting Sort", |b| b.iter(|| counting(&mut arr)));
}

fn merge_sort_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut items: Vec<usize> = (1..=10000).collect();
    items.shuffle(&mut rng);
    let mut arr = black_box(items);

    c.bench_function("Merge Sort", |b| b.iter(|| merge(&mut arr)));
}

fn selection_sort_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut items: Vec<usize> = (1..=10000).collect();
    items.shuffle(&mut rng);
    let mut arr = black_box(items);

    c.bench_function("Selection Sort", |b| b.iter(|| selection(&mut arr)));
}

fn insertion_sort_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut items: Vec<usize> = (1..=10000).collect();
    items.shuffle(&mut rng);
    let mut arr = black_box(items);

    c.bench_function("Insertion Sort", |b| b.iter(|| insertion(&mut arr)));
}

criterion_group!(
    benches,
    bubble_sort_benchmark,
    counting_sort_benchmark,
    merge_sort_benchmark,
    selection_sort_benchmark,
    insertion_sort_benchmark
);

criterion_main!(benches);
