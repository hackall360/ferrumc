use crate::conn_init::LoginResult;
use crate::connection::{EncryptedReader, StreamWriter};
use crate::errors::NetError;
use crate::packets::incoming::handshake::Handshake;
use ferrumc_state::GlobalState;
use tokio::io::AsyncRead;

/// Handles cross-server transfer requests during the handshake phase.
///
/// Sends a `transfer` packet to the client instructing it to connect to
/// the specified host and port. The current connection is closed
/// afterwards.
pub(super) async fn transfer<R: AsyncRead + Unpin>(
    hs_packet: Handshake,
    _conn_read: &mut EncryptedReader<R>,
    conn_write: &StreamWriter,
    _state: GlobalState,
) -> Result<(bool, LoginResult), NetError> {
    let transfer_packet = crate::packets::outgoing::transfer::TransferPacket::new(
        hs_packet.server_address,
        hs_packet.server_port,
    );
    conn_write.send_packet(transfer_packet)?;
    Ok((
        true,
        LoginResult {
            player_identity: None,
            compression: false,
        },
    ))
}
