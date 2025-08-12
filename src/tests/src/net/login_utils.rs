use reqwest::Client;
use sha1::{Digest, Sha1};
use uuid::Uuid;

pub fn compute_server_hash(shared_secret: &[u8], public_key: &[u8]) -> String {
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

pub async fn verify_session(
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
