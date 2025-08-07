use crate::errors::NetError;
use reqwest::Client;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::env;
use uuid::Uuid;

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

pub async fn verify_session(
    username: &str,
    shared_secret: &[u8],
    public_key: &[u8],
) -> Result<Uuid, NetError> {
    let server_hash = compute_server_hash(shared_secret, public_key);
    let client = Client::new();
    let url = env::var("MOJANG_SESSION_URL").unwrap_or_else(|_| {
        "https://sessionserver.mojang.com/session/minecraft/hasJoined".to_string()
    });
    let resp = client
        .get(url)
        .query(&[("username", username), ("serverId", &server_hash)])
        .send()
        .await
        .map_err(|e| NetError::Misc(format!("Session request failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(NetError::Misc(format!(
            "Session server returned status {}",
            resp.status()
        )));
    }

    #[derive(Deserialize)]
    struct SessionResponse {
        id: String,
    }

    let body: SessionResponse = resp
        .json()
        .await
        .map_err(|e| NetError::Misc(format!("Invalid session response: {e}")))?;
    let uuid = Uuid::parse_str(&body.id)
        .map_err(|e| NetError::Misc(format!("Invalid UUID from session: {e}")))?;
    Ok(uuid)
}
