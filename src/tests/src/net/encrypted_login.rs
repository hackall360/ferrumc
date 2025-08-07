use ferrumc_net_encryption::{
    decrypt_shared_secret, generate_rsa_keypair, generate_verify_token, Aes128Cfb8Decryptor,
    Aes128Cfb8Encryptor,
};
use reqwest::Client;
use rsa::{
    pkcs1::EncodeRsaPublicKey, pkcs1v15::Pkcs1v15Encrypt, rand_core::OsRng, rand_core::RngCore,
};
use sha1::{Digest, Sha1};
use uuid::Uuid;
use wiremock::{matchers::path, Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn encrypted_login_handshake_and_encryption() {
    // Start mock session server
    let server = MockServer::start().await;
    let expected_uuid = Uuid::new_v4();
    Mock::given(path("/session/minecraft/hasJoined"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": expected_uuid.to_string()
        })))
        .mount(&server)
        .await;
    std::env::set_var(
        "MOJANG_SESSION_URL",
        format!("{}/session/minecraft/hasJoined", server.uri()),
    );

    // Server generates RSA key pair and verify token
    let (private_key, public_key) = generate_rsa_keypair().unwrap();
    let verify_token = generate_verify_token();

    // Client creates shared secret and encrypts values with public key
    let mut rng = OsRng;
    let mut shared_secret = [0u8; 16];
    rng.fill_bytes(&mut shared_secret);
    let encrypted_secret = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &shared_secret)
        .unwrap();
    let encrypted_token = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &verify_token)
        .unwrap();

    // Server decrypts and validates
    let decrypted_secret = decrypt_shared_secret(&private_key, &encrypted_secret).unwrap();
    assert_eq!(decrypted_secret, shared_secret);
    let decrypted_token = decrypt_shared_secret(&private_key, &encrypted_token).unwrap();
    assert_eq!(decrypted_token, verify_token);

    // Verify session using mocked server
    let public_key_der = public_key.to_pkcs1_der().unwrap();
    let uuid = verify_session("player", &shared_secret, public_key_der.as_bytes())
        .await
        .unwrap();
    assert_eq!(uuid, expected_uuid);

    // Confirm packets are encrypted/decrypted correctly
    let encryptor = Aes128Cfb8Encryptor::new(shared_secret, shared_secret);
    let decryptor = Aes128Cfb8Decryptor::new(shared_secret, shared_secret);
    let payload = b"hello world";
    let encrypted = encryptor.encrypt(payload).unwrap();
    assert_ne!(encrypted, payload);
    let decrypted = decryptor.decrypt(&encrypted).unwrap();
    assert_eq!(decrypted, payload);
}

fn compute_server_hash(shared_secret: &[u8], public_key: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(shared_secret);
    hasher.update(public_key);
    let mut digest = hasher.finalize().to_vec();
    let negative = digest[0] & 0x80 != 0;
    if negative {
        let mut carry = true;
        for byte in digest.iter_mut().rev() {
            *byte = !*byte;
            if carry {
                let (new, overflow) = byte.overflowing_add(1);
                *byte = new;
                carry = overflow;
            }
        }
    }
    let mut hex = digest
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    while hex.starts_with('0') && hex.len() > 1 {
        hex.remove(0);
    }
    if negative {
        format!("-{}", hex)
    } else {
        hex
    }
}

async fn verify_session(
    username: &str,
    shared_secret: &[u8],
    public_key: &[u8],
) -> Result<Uuid, reqwest::Error> {
    let server_hash = compute_server_hash(shared_secret, public_key);
    let url = std::env::var("MOJANG_SESSION_URL").expect("MOJANG_SESSION_URL not set");
    let client = Client::new();
    let resp = client
        .get(url)
        .query(&[("username", username), ("serverId", &server_hash)])
        .send()
        .await?;
    let body: serde_json::Value = resp.json().await?;
    Ok(Uuid::parse_str(body["id"].as_str().unwrap()).unwrap())
}
