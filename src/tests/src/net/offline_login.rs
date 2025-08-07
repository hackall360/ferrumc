use ferrumc_config::server_config::{set_global_config, ServerConfig};
use uuid::Uuid;

#[test]
fn offline_login_uuid_matches() {
    set_global_config(ServerConfig {
        online_mode: false,
        ..Default::default()
    });
    let username = "player";
    let expected = Uuid::new_v3(
        &Uuid::NAMESPACE_DNS,
        format!("OfflinePlayer:{}", username).as_bytes(),
    );
    let computed = Uuid::new_v3(
        &Uuid::NAMESPACE_DNS,
        format!("OfflinePlayer:{}", username).as_bytes(),
    );
    assert_eq!(expected, computed);
}
