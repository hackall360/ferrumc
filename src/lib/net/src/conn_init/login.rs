use crate::auth::verify_session;
use crate::compression::compress_packet;
use crate::conn_init::VarInt;
use crate::conn_init::{LoginResult, NetDecodeOpts};
use crate::connection::{EncryptedReader, StreamWriter};
use crate::errors::{NetError, PacketError};
use crate::packets::incoming::packet_skeleton::PacketSkeleton;
use crate::packets::outgoing::login_disconnect::LoginDisconnectPacket;
use crate::ConnState::*;
use ferrumc_config::server_config::get_global_config;
use ferrumc_core::identity::player_identity::PlayerIdentity;
use ferrumc_macros::lookup_packet;
use ferrumc_net_codec::decode::NetDecode;
use ferrumc_net_codec::encode::NetEncodeOpts;
use ferrumc_net_codec::net_types::byte_array::ByteArray;
use ferrumc_net_codec::net_types::length_prefixed_vec::LengthPrefixedVec;
use ferrumc_net_encryption::{decrypt_shared_secret, generate_rsa_keypair, generate_verify_token};
use ferrumc_state::GlobalState;
use rsa::pkcs1::EncodeRsaPublicKey;
use tokio::io::AsyncRead;
use tracing::error;
use uuid::Uuid;

/// Handles the **login sequence** for a newly connecting client.
///
/// This function follows the Minecraft 1.20.1 login handshake:
/// 1. Reads the initial login packet and authenticates the username/UUID.
/// 2. Optionally enables network compression.
/// 3. Sends login success and transitions directly into the Play state.
/// 4. Spawns the player in the world (initial chunks, teleport confirmation).
///
/// # Returns
/// `(false, LoginResult)` on success, where:
/// - `false` = keep connection open.
/// - `LoginResult` contains player identity and compression settings.
///
/// # Errors
/// Returns `NetError` for protocol violations, unexpected packets, or I/O errors.
pub(super) async fn login<R: AsyncRead + Unpin>(
    conn_read: &mut EncryptedReader<R>,
    conn_write: &StreamWriter,
    state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    let mut compressed = false;

    // =============================================================================================
    // 1 Receive initial Login Start packet
    let mut skel = PacketSkeleton::new(conn_read, compressed, Login).await?;

    let expected_id = lookup_packet!("login", "serverbound", "hello");

    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Login,
        }));
    }

    let login_start = crate::packets::incoming::login_start::LoginStartPacket::decode(
        &mut skel.data,
        &NetDecodeOpts::None,
    )?;

    let mut player_uuid = login_start.uuid;

    // =============================================================================================
    // 2 Handle online-mode encryption negotiation
    if get_global_config().online_mode {
        use crate::packets::incoming::login_encryption_response::LoginEncryptionResponse;
        use crate::packets::outgoing::login_encryption_request::LoginEncryptionRequest;

        let (private_key, public_key) = generate_rsa_keypair()?;
        let verify_token = generate_verify_token();
        let public_key_der = public_key.to_pkcs1_der().map_err(|e| {
            NetError::EncryptionError(
                ferrumc_net_encryption::errors::NetEncryptionError::RsaError(e.to_string()),
            )
        })?;

        let request = LoginEncryptionRequest {
            server_id: "",
            public_key: ByteArray::new(public_key_der.as_bytes().to_vec()),
            verify_token: ByteArray::new(verify_token.to_vec()),
        };
        conn_write.send_packet(request)?;

        let mut skel = PacketSkeleton::new(conn_read, compressed, Login).await?;
        if skel.id != 0x01 {
            return Err(NetError::Packet(PacketError::UnexpectedPacket {
                expected: 0x01,
                received: skel.id,
                state: Login,
            }));
        }
        let response = LoginEncryptionResponse::decode(&mut skel.data, &NetDecodeOpts::None)?;

        let shared_secret_vec = decrypt_shared_secret(&private_key, &response.shared_secret)?;
        let shared_secret: [u8; 16] = shared_secret_vec
            .try_into()
            .map_err(|_| NetError::Misc("Invalid shared secret".to_string()))?;
        let token = decrypt_shared_secret(&private_key, &response.verify_token)?;
        if token != verify_token {
            return Err(NetError::Misc("Invalid verify token".to_string()));
        }

        conn_write.enable_encryption(&shared_secret)?;
        conn_read.enable_encryption(&shared_secret)?;

        match verify_session(
            &login_start.username,
            &shared_secret,
            public_key_der.as_bytes(),
        )
        .await
        {
            Ok(uuid) => {
                player_uuid = uuid.as_u128();
            }
            Err(err) => {
                error!("Session verification failed: {:?}", err);
                let disconnect = LoginDisconnectPacket::new("Failed to verify session");
                conn_write.send_packet(disconnect)?;
                return Ok((
                    true,
                    LoginResult {
                        player_identity: None,
                        compression: false,
                    },
                ));
            }
        }
    }

    // =============================================================================================
    // 3 Negotiate compression if configured
    if get_global_config().network_compression_threshold > 0 {
        compressed = true;

        let compression_packet = crate::packets::outgoing::set_compression::SetCompressionPacket {
            threshold: VarInt::new(get_global_config().network_compression_threshold),
        };
        conn_write.send_packet(compression_packet)?;
        conn_write
            .compress
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    // =============================================================================================
    // 4 Send Login Success (UUID and username acknowledgement)
    let login_success = crate::packets::outgoing::login_success::LoginSuccessPacket {
        uuid: player_uuid,
        username: &login_start.username,
        properties: LengthPrefixedVec::default(),
    };

    conn_write.send_packet(login_success)?;

    // Build PlayerIdentity for server-side tracking
    let player_identity = PlayerIdentity {
        uuid: Uuid::from_u128(player_uuid),
        username: login_start.username.clone(),
        short_uuid: player_uuid as i32,
    };

    // =============================================================================================
    // 4 Send login_play packet to switch to Play state
    let login_play =
        crate::packets::outgoing::login_play::LoginPlayPacket::new(player_identity.short_uuid);
    conn_write.send_packet(login_play)?;

    // =============================================================================================
    // 5 Send initial player position sync (requires teleport confirmation)
    let teleport_id_i32: i32 = (rand::random::<u32>() & 0x3FFF_FFFF) as i32;
    let sync_player_pos =
        crate::packets::outgoing::synchronize_player_position::SynchronizePlayerPositionPacket {
            teleport_id: VarInt::new(teleport_id_i32),
            ..Default::default()
        };
    conn_write.send_packet(sync_player_pos)?;

    // =============================================================================================
    // 6 Await client's teleport acceptance
    let mut skel = PacketSkeleton::new(conn_read, compressed, Play).await?;
    let expected_id = lookup_packet!("play", "serverbound", "accept_teleportation");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Play,
        }));
    }

    let confirm_player_teleport =
        crate::packets::incoming::confirm_player_teleport::ConfirmPlayerTeleport::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    if confirm_player_teleport.teleport_id.0 != teleport_id_i32 {
        error!(
            "Teleport ID mismatch: expected {}, got {}",
            teleport_id_i32, confirm_player_teleport.teleport_id.0
        )
    }

    // =============================================================================================
    // 7 Receive first movement packet from player
    let mut skel = PacketSkeleton::new(conn_read, compressed, Play).await?;
    let expected_id = lookup_packet!("play", "serverbound", "move_player_pos_rot");
    if skel.id != expected_id {
        return Err(NetError::Packet(PacketError::UnexpectedPacket {
            expected: expected_id,
            received: skel.id,
            state: Play,
        }));
    }

    let _player_pos_and_rot =
        crate::packets::incoming::set_player_position_and_rotation::SetPlayerPositionAndRotationPacket::decode(
            &mut skel.data,
            &NetDecodeOpts::None,
        )?;

    // =============================================================================================
    // 8 Send initial game event (e.g., "change game mode")
    let game_event = crate::packets::outgoing::game_event::GameEventPacket::new(13, 0.0);
    conn_write.send_packet(game_event)?;

    // =============================================================================================
    // 9 Send center chunk packet (player spawn location)
    let center_chunk = crate::packets::outgoing::set_center_chunk::SetCenterChunk::new(0, 0);
    conn_write.send_packet(center_chunk)?;

    // =============================================================================================
    // 10 Load and send surrounding chunks within render distance
    let radius = get_global_config().chunk_render_distance as i32;

    let mut batch = state.thread_pool.batch();

    for x in -radius..=radius {
        for z in -radius..=radius {
            batch.execute({
                let state = state.clone();
                move || -> Result<Vec<u8>, NetError> {
                    let chunk = state.world.load_chunk(x, z, "overworld")?;
                    let chunk_data =
                        crate::packets::outgoing::chunk_and_light_data::ChunkAndLightData::from_chunk(
                            &chunk,
                        )?;
                    let compressed_packet = compress_packet(&chunk_data, compressed, &NetEncodeOpts::WithLength)?;
                    Ok(compressed_packet)
                }
            });
        }
    }

    let packets = batch.wait();

    for packet in packets {
        match packet {
            Ok(data) => {
                conn_write.send_raw_packet(data)?;
            }
            Err(err) => {
                error!("Failed to send chunk data: {:?}", err);
                return Err(NetError::Misc(format!(
                    "Failed to send chunk data: {:?}",
                    err
                )));
            }
        }
    }

    // =============================================================================================
    // ✅ Login sequence complete
    Ok((
        false,
        LoginResult {
            player_identity: Some(player_identity),
            compression: compressed,
        },
    ))
}
