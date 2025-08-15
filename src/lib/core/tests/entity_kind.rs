use ferrumc_core::ai::{default_goals, EntityKind};

#[test]
fn all_entity_kinds_have_network_id_and_goals() {
    for &kind in EntityKind::ALL {
        assert!(kind.network_id() > 0, "{:?} missing network id", kind);
        assert!(
            !default_goals(kind).is_empty(),
            "{:?} missing default goals",
            kind
        );
    }
}
