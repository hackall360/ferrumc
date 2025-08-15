use crate::ai::EntityKind;
use crate::health::Health;
use crate::movement::Speed;

/// Default attribute set for a given entity type.
#[derive(Debug, Clone)]
pub struct Attributes {
    pub health: Health,
    pub speed: Speed,
}

/// Returns the default [`Attributes`] for the provided [`EntityKind`].
///
/// Only a handful of entities are explicitly mapped. Any unmapped entity
/// will receive a reasonable default set of attributes.
pub fn attributes_for(kind: EntityKind) -> Attributes {
    match kind {
        EntityKind::Player => Attributes {
            health: Health::with_attributes(20.0, 0.0, 0.0),
            speed: Speed(0.1),
        },
        EntityKind::Cow => Attributes {
            health: Health::with_attributes(20.0, 0.0, 0.0),
            speed: Speed(0.2),
        },
        EntityKind::Zombie => Attributes {
            health: Health::with_attributes(20.0, 2.0, 0.0),
            speed: Speed(0.23),
        },
        EntityKind::Skeleton => Attributes {
            health: Health::with_attributes(20.0, 2.0, 0.0),
            speed: Speed(0.25),
        },
        _ => Attributes {
            health: Health::with_attributes(20.0, 0.0, 0.0),
            speed: Speed::default(),
        },
    }
}
