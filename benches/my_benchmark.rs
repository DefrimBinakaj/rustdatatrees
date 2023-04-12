use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
#[path = "../src/main.rs"]
mod main;

fn bench_rbtree(c: &mut Criterion) {
    
}

fn bench_avltree(c: &mut Criterion) {
    
}

criterion_group!(benches, bench_rbtree, bench_avltree);
criterion_main!(benches);