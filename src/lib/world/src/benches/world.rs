mod cache;
mod chunk_loading;
mod edit_bench;

use criterion::{criterion_group, criterion_main};
fn world_benches(c: &mut criterion::Criterion) {
    edit_bench::bench_edits(c);
    cache::bench_cache(c);
    chunk_loading::bench_chunk_loading(c);
}
criterion_group!(world_bench, world_benches);
criterion_main!(world_bench);
