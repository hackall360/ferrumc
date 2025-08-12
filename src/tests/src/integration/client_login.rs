use crate::net::login_utils::verify_session;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_net_encryption::{
    decrypt_shared_secret, generate_rsa_keypair, generate_verify_token, Aes128Cfb8Decryptor,
    Aes128Cfb8Encryptor,
};
use rsa::{
    pkcs1::EncodeRsaPublicKey, pkcs1v15::Pkcs1v15Encrypt, rand_core::OsRng, rand_core::RngCore,
};
use uuid::Uuid;
use wiremock::{matchers::path, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn client_login_flow() {
    // Offline mode UUID computation
    let username = "player";
    let expected = Uuid::new_v3(
        &Uuid::NAMESPACE_DNS,
        format!("OfflinePlayer:{}", username).as_bytes(),
    );
    let identity = PlayerIdentity::new(username.to_string(), expected.as_u128());
    assert_eq!(identity.uuid, expected);

    // Online mode session verification
    let server = MockServer::start().await;
    Mock::given(path("/session/minecraft/hasJoined"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": Uuid::new_v4().to_string()
        })))
        .mount(&server)
        .await;
    std::env::set_var(
        "MOJANG_SESSION_URL",
        format!("{}/session/minecraft/hasJoined", server.uri()),
    );

    let (private_key, public_key) = generate_rsa_keypair().unwrap();
    let verify_token = generate_verify_token();
    let mut rng = OsRng;
    let mut shared_secret = [0u8; 16];
    rng.fill_bytes(&mut shared_secret);
    let encrypted_secret = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &shared_secret)
        .unwrap();
    let encrypted_token = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &verify_token)
        .unwrap();

    let decrypted_secret = decrypt_shared_secret(&private_key, &encrypted_secret).unwrap();
    assert_eq!(decrypted_secret, shared_secret);
    let decrypted_token = decrypt_shared_secret(&private_key, &encrypted_token).unwrap();
    assert_eq!(decrypted_token, verify_token);

    let public_key_der = public_key.to_pkcs1_der().unwrap();
    let uuid = verify_session(username, &shared_secret, public_key_der.as_bytes())
        .await
        .unwrap();
    assert_ne!(uuid, Uuid::nil());

    let encryptor = Aes128Cfb8Encryptor::new(shared_secret, shared_secret);
    let decryptor = Aes128Cfb8Decryptor::new(shared_secret, shared_secret);
    let payload = b"hello world";
    let encrypted = encryptor.encrypt(payload).unwrap();
    let decrypted = decryptor.decrypt(&encrypted).unwrap();
    assert_eq!(decrypted, payload);
}
