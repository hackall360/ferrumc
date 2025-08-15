use bevy_ecs::prelude::Component;
use typename::TypeName;

use crate::transform::position::Position;

mod passive;
mod hostile;
mod neutral;

#[derive(TypeName, Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityKind {
    Allay,
    AreaEffectCloud,
    ArmorStand,
    Arrow,
    Axolotl,
    Bat,
    Bee,
    Blaze,
    BlockDisplay,
    Boat,
    Camel,
    Cat,
    CaveSpider,
    ChestBoat,
    ChestMinecart,
    Chicken,
    Cod,
    CommandBlockMinecart,
    Cow,
    Creeper,
    Dolphin,
    Donkey,
    DragonFireball,
    Drowned,
    Egg,
    ElderGuardian,
    EndCrystal,
    EnderDragon,
    EnderPearl,
    Enderman,
    Endermite,
    Evoker,
    EvokerFangs,
    ExperienceBottle,
    ExperienceOrb,
    EyeOfEnder,
    FallingBlock,
    FireworkRocket,
    Fox,
    Frog,
    FurnaceMinecart,
    Ghast,
    Giant,
    GlowItemFrame,
    GlowSquid,
    Goat,
    Guardian,
    Hoglin,
    HopperMinecart,
    Horse,
    Husk,
    Illusioner,
    Interaction,
    IronGolem,
    Item,
    ItemDisplay,
    ItemFrame,
    Fireball,
    LeashKnot,
    LightningBolt,
    Llama,
    LlamaSpit,
    MagmaCube,
    Marker,
    Minecart,
    Mooshroom,
    Mule,
    Ocelot,
    Painting,
    Panda,
    Parrot,
    Phantom,
    Pig,
    Piglin,
    PiglinBrute,
    Pillager,
    PolarBear,
    Potion,
    Pufferfish,
    Rabbit,
    Ravager,
    Salmon,
    Sheep,
    Shulker,
    ShulkerBullet,
    Silverfish,
    Skeleton,
    SkeletonHorse,
    Slime,
    SmallFireball,
    Sniffer,
    SnowGolem,
    Snowball,
    SpawnerMinecart,
    SpectralArrow,
    Spider,
    Squid,
    Stray,
    Strider,
    Tadpole,
    TextDisplay,
    Tnt,
    TntMinecart,
    TraderLlama,
    Trident,
    TropicalFish,
    Turtle,
    Vex,
    Villager,
    Vindicator,
    WanderingTrader,
    Warden,
    Witch,
    Wither,
    WitherSkeleton,
    WitherSkull,
    Wolf,
    Zoglin,
    Zombie,
    ZombieHorse,
    ZombieVillager,
    ZombifiedPiglin,
    Player,
    FishingBobber,
}

#[derive(TypeName, Component, Debug, Clone)]
pub enum AIGoal {
    Idle,
    Wander,
    Graze,
    Flee { from: Position },
    Target { target: Position },
    Attack { target: Position },
    Trade,
    Defend { target: Position },
}

#[derive(TypeName, Component, Debug, Clone, Copy)]
pub struct Mob {
    pub kind: EntityKind,
}

#[derive(TypeName, Component, Debug, Default, Clone, Copy)]
pub struct PendingSpawn;

pub fn default_goals(kind: EntityKind) -> Vec<AIGoal> {
    if let Some(goals) = passive::goals(kind) {
        return goals;
    }
    if let Some(goals) = hostile::goals(kind) {
        return goals;
    }
    if let Some(goals) = neutral::goals(kind) {
        return goals;
    }
    vec![AIGoal::Idle]
}

const ALLAY_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:allay");
const AREA_EFFECT_CLOUD_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:area_effect_cloud");
const ARMOR_STAND_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:armor_stand");
const ARROW_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:arrow");
const AXOLOTL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:axolotl");
const BAT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:bat");
const BEE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:bee");
const BLAZE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:blaze");
const BLOCK_DISPLAY_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:block_display");
const BOAT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:boat");
const CAMEL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:camel");
const CAT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:cat");
const CAVE_SPIDER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:cave_spider");
const CHEST_BOAT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:chest_boat");
const CHEST_MINECART_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:chest_minecart");
const CHICKEN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:chicken");
const COD_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:cod");
const COMMAND_BLOCK_MINECART_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:command_block_minecart");
const COW_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:cow");
const CREEPER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:creeper");
const DOLPHIN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:dolphin");
const DONKEY_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:donkey");
const DRAGON_FIREBALL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:dragon_fireball");
const DROWNED_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:drowned");
const EGG_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:egg");
const ELDER_GUARDIAN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:elder_guardian");
const END_CRYSTAL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:end_crystal");
const ENDER_DRAGON_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:ender_dragon");
const ENDER_PEARL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:ender_pearl");
const ENDERMAN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:enderman");
const ENDERMITE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:endermite");
const EVOKER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:evoker");
const EVOKER_FANGS_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:evoker_fangs");
const EXPERIENCE_BOTTLE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:experience_bottle");
const EXPERIENCE_ORB_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:experience_orb");
const EYE_OF_ENDER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:eye_of_ender");
const FALLING_BLOCK_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:falling_block");
const FIREWORK_ROCKET_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:firework_rocket");
const FOX_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:fox");
const FROG_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:frog");
const FURNACE_MINECART_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:furnace_minecart");
const GHAST_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:ghast");
const GIANT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:giant");
const GLOW_ITEM_FRAME_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:glow_item_frame");
const GLOW_SQUID_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:glow_squid");
const GOAT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:goat");
const GUARDIAN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:guardian");
const HOGLIN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:hoglin");
const HOPPER_MINECART_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:hopper_minecart");
const HORSE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:horse");
const HUSK_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:husk");
const ILLUSIONER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:illusioner");
const INTERACTION_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:interaction");
const IRON_GOLEM_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:iron_golem");
const ITEM_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:item");
const ITEM_DISPLAY_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:item_display");
const ITEM_FRAME_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:item_frame");
const FIREBALL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:fireball");
const LEASH_KNOT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:leash_knot");
const LIGHTNING_BOLT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:lightning_bolt");
const LLAMA_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:llama");
const LLAMA_SPIT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:llama_spit");
const MAGMA_CUBE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:magma_cube");
const MARKER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:marker");
const MINECART_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:minecart");
const MOOSHROOM_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:mooshroom");
const MULE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:mule");
const OCELOT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:ocelot");
const PAINTING_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:painting");
const PANDA_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:panda");
const PARROT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:parrot");
const PHANTOM_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:phantom");
const PIG_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:pig");
const PIGLIN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:piglin");
const PIGLIN_BRUTE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:piglin_brute");
const PILLAGER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:pillager");
const POLAR_BEAR_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:polar_bear");
const POTION_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:potion");
const PUFFERFISH_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:pufferfish");
const RABBIT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:rabbit");
const RAVAGER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:ravager");
const SALMON_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:salmon");
const SHEEP_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:sheep");
const SHULKER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:shulker");
const SHULKER_BULLET_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:shulker_bullet");
const SILVERFISH_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:silverfish");
const SKELETON_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:skeleton");
const SKELETON_HORSE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:skeleton_horse");
const SLIME_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:slime");
const SMALL_FIREBALL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:small_fireball");
const SNIFFER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:sniffer");
const SNOW_GOLEM_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:snow_golem");
const SNOWBALL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:snowball");
const SPAWNER_MINECART_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:spawner_minecart");
const SPECTRAL_ARROW_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:spectral_arrow");
const SPIDER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:spider");
const SQUID_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:squid");
const STRAY_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:stray");
const STRIDER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:strider");
const TADPOLE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:tadpole");
const TEXT_DISPLAY_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:text_display");
const TNT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:tnt");
const TNT_MINECART_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:tnt_minecart");
const TRADER_LLAMA_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:trader_llama");
const TRIDENT_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:trident");
const TROPICAL_FISH_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:tropical_fish");
const TURTLE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:turtle");
const VEX_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:vex");
const VILLAGER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:villager");
const VINDICATOR_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:vindicator");
const WANDERING_TRADER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:wandering_trader");
const WARDEN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:warden");
const WITCH_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:witch");
const WITHER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:wither");
const WITHER_SKELETON_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:wither_skeleton");
const WITHER_SKULL_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:wither_skull");
const WOLF_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:wolf");
const ZOGLIN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:zoglin");
const ZOMBIE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:zombie");
const ZOMBIE_HORSE_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:zombie_horse");
const ZOMBIE_VILLAGER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:zombie_villager");
const ZOMBIFIED_PIGLIN_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:zombified_piglin");
const PLAYER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:player");
const FISHING_BOBBER_ID: u64 = get_registry_entry!("minecraft:entity_type.entries.minecraft:fishing_bobber");

impl EntityKind {
    pub const ALL: &'static [EntityKind] = &[
        EntityKind::Allay,
        EntityKind::AreaEffectCloud,
        EntityKind::ArmorStand,
        EntityKind::Arrow,
        EntityKind::Axolotl,
        EntityKind::Bat,
        EntityKind::Bee,
        EntityKind::Blaze,
        EntityKind::BlockDisplay,
        EntityKind::Boat,
        EntityKind::Camel,
        EntityKind::Cat,
        EntityKind::CaveSpider,
        EntityKind::ChestBoat,
        EntityKind::ChestMinecart,
        EntityKind::Chicken,
        EntityKind::Cod,
        EntityKind::CommandBlockMinecart,
        EntityKind::Cow,
        EntityKind::Creeper,
        EntityKind::Dolphin,
        EntityKind::Donkey,
        EntityKind::DragonFireball,
        EntityKind::Drowned,
        EntityKind::Egg,
        EntityKind::ElderGuardian,
        EntityKind::EndCrystal,
        EntityKind::EnderDragon,
        EntityKind::EnderPearl,
        EntityKind::Enderman,
        EntityKind::Endermite,
        EntityKind::Evoker,
        EntityKind::EvokerFangs,
        EntityKind::ExperienceBottle,
        EntityKind::ExperienceOrb,
        EntityKind::EyeOfEnder,
        EntityKind::FallingBlock,
        EntityKind::FireworkRocket,
        EntityKind::Fox,
        EntityKind::Frog,
        EntityKind::FurnaceMinecart,
        EntityKind::Ghast,
        EntityKind::Giant,
        EntityKind::GlowItemFrame,
        EntityKind::GlowSquid,
        EntityKind::Goat,
        EntityKind::Guardian,
        EntityKind::Hoglin,
        EntityKind::HopperMinecart,
        EntityKind::Horse,
        EntityKind::Husk,
        EntityKind::Illusioner,
        EntityKind::Interaction,
        EntityKind::IronGolem,
        EntityKind::Item,
        EntityKind::ItemDisplay,
        EntityKind::ItemFrame,
        EntityKind::Fireball,
        EntityKind::LeashKnot,
        EntityKind::LightningBolt,
        EntityKind::Llama,
        EntityKind::LlamaSpit,
        EntityKind::MagmaCube,
        EntityKind::Marker,
        EntityKind::Minecart,
        EntityKind::Mooshroom,
        EntityKind::Mule,
        EntityKind::Ocelot,
        EntityKind::Painting,
        EntityKind::Panda,
        EntityKind::Parrot,
        EntityKind::Phantom,
        EntityKind::Pig,
        EntityKind::Piglin,
        EntityKind::PiglinBrute,
        EntityKind::Pillager,
        EntityKind::PolarBear,
        EntityKind::Potion,
        EntityKind::Pufferfish,
        EntityKind::Rabbit,
        EntityKind::Ravager,
        EntityKind::Salmon,
        EntityKind::Sheep,
        EntityKind::Shulker,
        EntityKind::ShulkerBullet,
        EntityKind::Silverfish,
        EntityKind::Skeleton,
        EntityKind::SkeletonHorse,
        EntityKind::Slime,
        EntityKind::SmallFireball,
        EntityKind::Sniffer,
        EntityKind::SnowGolem,
        EntityKind::Snowball,
        EntityKind::SpawnerMinecart,
        EntityKind::SpectralArrow,
        EntityKind::Spider,
        EntityKind::Squid,
        EntityKind::Stray,
        EntityKind::Strider,
        EntityKind::Tadpole,
        EntityKind::TextDisplay,
        EntityKind::Tnt,
        EntityKind::TntMinecart,
        EntityKind::TraderLlama,
        EntityKind::Trident,
        EntityKind::TropicalFish,
        EntityKind::Turtle,
        EntityKind::Vex,
        EntityKind::Villager,
        EntityKind::Vindicator,
        EntityKind::WanderingTrader,
        EntityKind::Warden,
        EntityKind::Witch,
        EntityKind::Wither,
        EntityKind::WitherSkeleton,
        EntityKind::WitherSkull,
        EntityKind::Wolf,
        EntityKind::Zoglin,
        EntityKind::Zombie,
        EntityKind::ZombieHorse,
        EntityKind::ZombieVillager,
        EntityKind::ZombifiedPiglin,
        EntityKind::Player,
        EntityKind::FishingBobber,
    ];

    pub fn network_id(self) -> i32 {
        match self {
            EntityKind::Allay => ALLAY_ID as i32,
            EntityKind::AreaEffectCloud => AREA_EFFECT_CLOUD_ID as i32,
            EntityKind::ArmorStand => ARMOR_STAND_ID as i32,
            EntityKind::Arrow => ARROW_ID as i32,
            EntityKind::Axolotl => AXOLOTL_ID as i32,
            EntityKind::Bat => BAT_ID as i32,
            EntityKind::Bee => BEE_ID as i32,
            EntityKind::Blaze => BLAZE_ID as i32,
            EntityKind::BlockDisplay => BLOCK_DISPLAY_ID as i32,
            EntityKind::Boat => BOAT_ID as i32,
            EntityKind::Camel => CAMEL_ID as i32,
            EntityKind::Cat => CAT_ID as i32,
            EntityKind::CaveSpider => CAVE_SPIDER_ID as i32,
            EntityKind::ChestBoat => CHEST_BOAT_ID as i32,
            EntityKind::ChestMinecart => CHEST_MINECART_ID as i32,
            EntityKind::Chicken => CHICKEN_ID as i32,
            EntityKind::Cod => COD_ID as i32,
            EntityKind::CommandBlockMinecart => COMMAND_BLOCK_MINECART_ID as i32,
            EntityKind::Cow => COW_ID as i32,
            EntityKind::Creeper => CREEPER_ID as i32,
            EntityKind::Dolphin => DOLPHIN_ID as i32,
            EntityKind::Donkey => DONKEY_ID as i32,
            EntityKind::DragonFireball => DRAGON_FIREBALL_ID as i32,
            EntityKind::Drowned => DROWNED_ID as i32,
            EntityKind::Egg => EGG_ID as i32,
            EntityKind::ElderGuardian => ELDER_GUARDIAN_ID as i32,
            EntityKind::EndCrystal => END_CRYSTAL_ID as i32,
            EntityKind::EnderDragon => ENDER_DRAGON_ID as i32,
            EntityKind::EnderPearl => ENDER_PEARL_ID as i32,
            EntityKind::Enderman => ENDERMAN_ID as i32,
            EntityKind::Endermite => ENDERMITE_ID as i32,
            EntityKind::Evoker => EVOKER_ID as i32,
            EntityKind::EvokerFangs => EVOKER_FANGS_ID as i32,
            EntityKind::ExperienceBottle => EXPERIENCE_BOTTLE_ID as i32,
            EntityKind::ExperienceOrb => EXPERIENCE_ORB_ID as i32,
            EntityKind::EyeOfEnder => EYE_OF_ENDER_ID as i32,
            EntityKind::FallingBlock => FALLING_BLOCK_ID as i32,
            EntityKind::FireworkRocket => FIREWORK_ROCKET_ID as i32,
            EntityKind::Fox => FOX_ID as i32,
            EntityKind::Frog => FROG_ID as i32,
            EntityKind::FurnaceMinecart => FURNACE_MINECART_ID as i32,
            EntityKind::Ghast => GHAST_ID as i32,
            EntityKind::Giant => GIANT_ID as i32,
            EntityKind::GlowItemFrame => GLOW_ITEM_FRAME_ID as i32,
            EntityKind::GlowSquid => GLOW_SQUID_ID as i32,
            EntityKind::Goat => GOAT_ID as i32,
            EntityKind::Guardian => GUARDIAN_ID as i32,
            EntityKind::Hoglin => HOGLIN_ID as i32,
            EntityKind::HopperMinecart => HOPPER_MINECART_ID as i32,
            EntityKind::Horse => HORSE_ID as i32,
            EntityKind::Husk => HUSK_ID as i32,
            EntityKind::Illusioner => ILLUSIONER_ID as i32,
            EntityKind::Interaction => INTERACTION_ID as i32,
            EntityKind::IronGolem => IRON_GOLEM_ID as i32,
            EntityKind::Item => ITEM_ID as i32,
            EntityKind::ItemDisplay => ITEM_DISPLAY_ID as i32,
            EntityKind::ItemFrame => ITEM_FRAME_ID as i32,
            EntityKind::Fireball => FIREBALL_ID as i32,
            EntityKind::LeashKnot => LEASH_KNOT_ID as i32,
            EntityKind::LightningBolt => LIGHTNING_BOLT_ID as i32,
            EntityKind::Llama => LLAMA_ID as i32,
            EntityKind::LlamaSpit => LLAMA_SPIT_ID as i32,
            EntityKind::MagmaCube => MAGMA_CUBE_ID as i32,
            EntityKind::Marker => MARKER_ID as i32,
            EntityKind::Minecart => MINECART_ID as i32,
            EntityKind::Mooshroom => MOOSHROOM_ID as i32,
            EntityKind::Mule => MULE_ID as i32,
            EntityKind::Ocelot => OCELOT_ID as i32,
            EntityKind::Painting => PAINTING_ID as i32,
            EntityKind::Panda => PANDA_ID as i32,
            EntityKind::Parrot => PARROT_ID as i32,
            EntityKind::Phantom => PHANTOM_ID as i32,
            EntityKind::Pig => PIG_ID as i32,
            EntityKind::Piglin => PIGLIN_ID as i32,
            EntityKind::PiglinBrute => PIGLIN_BRUTE_ID as i32,
            EntityKind::Pillager => PILLAGER_ID as i32,
            EntityKind::PolarBear => POLAR_BEAR_ID as i32,
            EntityKind::Potion => POTION_ID as i32,
            EntityKind::Pufferfish => PUFFERFISH_ID as i32,
            EntityKind::Rabbit => RABBIT_ID as i32,
            EntityKind::Ravager => RAVAGER_ID as i32,
            EntityKind::Salmon => SALMON_ID as i32,
            EntityKind::Sheep => SHEEP_ID as i32,
            EntityKind::Shulker => SHULKER_ID as i32,
            EntityKind::ShulkerBullet => SHULKER_BULLET_ID as i32,
            EntityKind::Silverfish => SILVERFISH_ID as i32,
            EntityKind::Skeleton => SKELETON_ID as i32,
            EntityKind::SkeletonHorse => SKELETON_HORSE_ID as i32,
            EntityKind::Slime => SLIME_ID as i32,
            EntityKind::SmallFireball => SMALL_FIREBALL_ID as i32,
            EntityKind::Sniffer => SNIFFER_ID as i32,
            EntityKind::SnowGolem => SNOW_GOLEM_ID as i32,
            EntityKind::Snowball => SNOWBALL_ID as i32,
            EntityKind::SpawnerMinecart => SPAWNER_MINECART_ID as i32,
            EntityKind::SpectralArrow => SPECTRAL_ARROW_ID as i32,
            EntityKind::Spider => SPIDER_ID as i32,
            EntityKind::Squid => SQUID_ID as i32,
            EntityKind::Stray => STRAY_ID as i32,
            EntityKind::Strider => STRIDER_ID as i32,
            EntityKind::Tadpole => TADPOLE_ID as i32,
            EntityKind::TextDisplay => TEXT_DISPLAY_ID as i32,
            EntityKind::Tnt => TNT_ID as i32,
            EntityKind::TntMinecart => TNT_MINECART_ID as i32,
            EntityKind::TraderLlama => TRADER_LLAMA_ID as i32,
            EntityKind::Trident => TRIDENT_ID as i32,
            EntityKind::TropicalFish => TROPICAL_FISH_ID as i32,
            EntityKind::Turtle => TURTLE_ID as i32,
            EntityKind::Vex => VEX_ID as i32,
            EntityKind::Villager => VILLAGER_ID as i32,
            EntityKind::Vindicator => VINDICATOR_ID as i32,
            EntityKind::WanderingTrader => WANDERING_TRADER_ID as i32,
            EntityKind::Warden => WARDEN_ID as i32,
            EntityKind::Witch => WITCH_ID as i32,
            EntityKind::Wither => WITHER_ID as i32,
            EntityKind::WitherSkeleton => WITHER_SKELETON_ID as i32,
            EntityKind::WitherSkull => WITHER_SKULL_ID as i32,
            EntityKind::Wolf => WOLF_ID as i32,
            EntityKind::Zoglin => ZOGLIN_ID as i32,
            EntityKind::Zombie => ZOMBIE_ID as i32,
            EntityKind::ZombieHorse => ZOMBIE_HORSE_ID as i32,
            EntityKind::ZombieVillager => ZOMBIE_VILLAGER_ID as i32,
            EntityKind::ZombifiedPiglin => ZOMBIFIED_PIGLIN_ID as i32,
            EntityKind::Player => PLAYER_ID as i32,
            EntityKind::FishingBobber => FISHING_BOBBER_ID as i32,
        }
    }
}
