use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub mod chunkbench;

criterion_main!(chunkbench::chunk_benches);
