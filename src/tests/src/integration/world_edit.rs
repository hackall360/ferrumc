use ferrumc_world::block_id::BlockId;
use ferrumc_world::chunk_format::Chunk;

#[test]
fn world_edit_changes_block() {
    let mut chunk = Chunk::new(0, 0, "overworld".to_string());
    let original = chunk.get_block(0, 0, 0).unwrap();
    assert_eq!(original, BlockId(0));
    chunk.set_block(0, 0, 0, BlockId(1)).unwrap();
    let updated = chunk.get_block(0, 0, 0).unwrap();
    assert_eq!(updated, BlockId(1));
}
