use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[path = "../src/main.rs"] // Here
mod main;

use main::FishPopulation;

fn test_days(n: u64, population: &mut FishPopulation) -> usize {
    for _ in 1..n {
        population.one_day();
    }
    population.count()
}

fn criterion_benchmark(c: &mut Criterion) {
    let population = FishPopulation::from_str("3,4,3,1,2");
    c.bench_function("test 256 days", |b| {
        b.iter(|| test_days(black_box(256), &mut population.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
