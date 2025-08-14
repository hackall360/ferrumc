use bevy_ecs::prelude::Component;
use typename::TypeName;

macro_rules! entities {
    ($($name:ident),* $(,)?) => {
        $(#[derive(TypeName, Component, Debug, Clone)]
        pub struct $name;)*
    };
}

// Passive mobs
entities!(
    Allay,
    Axolotl,
    Bat,
    Camel,
    Cat,
    Chicken,
    Cod,
    Cow,
    Dolphin,
    Donkey,
    Fox,
    Frog,
    GlowSquid,
    Horse,
    Mooshroom,
    Mule,
    Ocelot,
    Panda,
    Parrot,
    Pig,
    Rabbit,
    Salmon,
    Sheep,
    Sniffer,
    Squid,
    Strider,
    Tadpole,
    TropicalFish,
    Turtle,
    Villager,
    WanderingTrader,
    TraderLlama,
    Goat,
    Pufferfish,
);

// Neutral mobs
entities!(
    Bee,
    Enderman,
    IronGolem,
    Llama,
    PolarBear,
    Piglin,
    ZombifiedPiglin,
    Wolf,
    SnowGolem,
);

// Hostile mobs
entities!(
    Blaze,
    CaveSpider,
    Creeper,
    Drowned,
    Endermite,
    Evoker,
    Ghast,
    Guardian,
    Hoglin,
    Husk,
    Illusioner,
    MagmaCube,
    Phantom,
    PiglinBrute,
    Pillager,
    Ravager,
    Shulker,
    Silverfish,
    Skeleton,
    SkeletonHorse,
    Slime,
    Spider,
    Stray,
    Vex,
    Vindicator,
    Witch,
    WitherSkeleton,
    Zoglin,
    Zombie,
    ZombieHorse,
    ZombieVillager,
    Giant,
);

// Boss mobs
entities!(
    ElderGuardian,
    EnderDragon,
    Warden,
    Wither,
);

// Utility entities
entities!(
    AreaEffectCloud,
    ArmorStand,
    BlockDisplay,
    EndCrystal,
    Interaction,
    Item,
    ItemDisplay,
    ItemFrame,
    GlowItemFrame,
    LeashKnot,
    LightningBolt,
    Marker,
    Painting,
    TextDisplay,
    Tnt,
    FallingBlock,
    ExperienceOrb,
    EyeOfEnder,
    Player,
);

// Projectiles
entities!(
    Arrow,
    DragonFireball,
    Egg,
    EnderPearl,
    EvokerFangs,
    ExperienceBottle,
    Fireball,
    FireworkRocket,
    FishingBobber,
    LlamaSpit,
    Potion,
    SmallFireball,
    Snowball,
    SpectralArrow,
    Trident,
    WitherSkull,
    ShulkerBullet,
);

// Vehicles
entities!(
    Boat,
    ChestBoat,
    Minecart,
    ChestMinecart,
    CommandBlockMinecart,
    FurnaceMinecart,
    HopperMinecart,
    SpawnerMinecart,
    TntMinecart,
);

pub mod spawn_rules;
