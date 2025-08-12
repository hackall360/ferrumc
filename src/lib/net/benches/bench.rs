use criterion::{criterion_group, criterion_main, Criterion};

mod packets;
mod throughput;
fn bench_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding packets");

    packets::bench_packets(&mut group);

    group.finish();
}

fn bench_throughput(c: &mut Criterion) {
    throughput::bench_network_throughput(c);
}
criterion_main!(benches);
criterion_group!(benches, bench_encoding, bench_throughput);
