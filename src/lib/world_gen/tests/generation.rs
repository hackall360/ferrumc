use ferrumc_world_gen::WorldGenerator;

#[test]
#[ignore]
fn snapshot_chunk() {
    let generator = WorldGenerator::new(0);
    let chunk = generator.generate_chunk(0, 0).unwrap();
    insta::assert_snapshot!(format!("{:?}", chunk));
}
