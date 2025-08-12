use criterion::{Criterion, Throughput};
use ferrumc_world_gen::WorldGenerator;
use std::hint::black_box;

pub(crate) fn bench_chunk_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunk_loading");
    group.throughput(Throughput::Elements(1));
    group.bench_function("generate_chunk", |b| {
        b.iter(|| {
            black_box(WorldGenerator::new(0).generate_chunk(0, 0).unwrap());
        })
    });
    group.finish();
}
