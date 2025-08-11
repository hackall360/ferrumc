use ferrumc_world::block_id::BlockId;

mod common;
use common::setup_world;

#[test]
#[ignore]
fn torch_has_power() {
    let world = setup_world();
    world
        .set_block_and_fetch(0, 1, 0, "overworld", BlockId(244))
        .unwrap();
    for _ in 0..2 {
        world.tick().unwrap();
    }
    assert_eq!(world.get_power_level(0, 1, 0, "overworld"), 15);
}

#[test]
#[ignore]
fn repeater_requires_input() {
    let world = setup_world();
    world
        .set_block_and_fetch(0, 1, 10, "overworld", BlockId(268))
        .unwrap();
    world
        .set_block_and_fetch(0, 1, 9, "overworld", BlockId(178))
        .unwrap();
    for _ in 0..2 {
        world.tick().unwrap();
    }
    assert_eq!(world.get_power_level(0, 1, 9, "overworld"), 0);
}
