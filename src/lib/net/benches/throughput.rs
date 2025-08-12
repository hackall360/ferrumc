use criterion::{Criterion, Throughput};
use ferrumc_net::packets::outgoing::chunk_and_light_data::ChunkAndLightData;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_world_gen::WorldGenerator;
use std::hint::black_box;

pub fn bench_network_throughput(c: &mut Criterion) {
    let chunk = WorldGenerator::new(0).generate_chunk(0, 0).unwrap();
    let packet = ChunkAndLightData::from_chunk(&chunk).unwrap();
    let mut buffer = Vec::new();
    packet
        .encode(&mut buffer, &NetEncodeOpts::WithLength)
        .unwrap();
    let size = buffer.len() as u64;

    let mut group = c.benchmark_group("network_throughput");
    group.throughput(Throughput::Bytes(size));
    group.bench_function("encode_chunk_packet", |b| {
        b.iter(|| {
            let mut buf = Vec::with_capacity(size as usize);
            packet.encode(&mut buf, &NetEncodeOpts::WithLength).unwrap();
            black_box(buf);
        });
    });
    group.finish();
}
