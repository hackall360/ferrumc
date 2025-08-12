use ferrumc_macros::packet;
use ferrumc_net_codec::decode::errors::NetDecodeError;
use ferrumc_net_codec::decode::{NetDecode, NetDecodeOpts};
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Read;
use tokio::io::{AsyncRead, AsyncReadExt};

#[packet(packet_id = "key", state = "login")]
#[derive(Debug)]
pub struct LoginEncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl NetDecode for LoginEncryptionResponse {
    fn decode<R: Read>(reader: &mut R, _opts: &NetDecodeOpts) -> Result<Self, NetDecodeError> {
        let secret_len = VarInt::decode(reader, &NetDecodeOpts::None)?.0 as usize;
        let mut shared_secret = vec![0u8; secret_len];
        reader.read_exact(&mut shared_secret)?;
        let token_len = VarInt::decode(reader, &NetDecodeOpts::None)?.0 as usize;
        let mut verify_token = vec![0u8; token_len];
        reader.read_exact(&mut verify_token)?;
        Ok(Self {
            shared_secret,
            verify_token,
        })
    }

    async fn decode_async<R: AsyncRead + Unpin>(
        reader: &mut R,
        _opts: &NetDecodeOpts,
    ) -> Result<Self, NetDecodeError> {
        let secret_len = VarInt::decode_async(reader, &NetDecodeOpts::None).await?.0 as usize;
        let mut shared_secret = vec![0u8; secret_len];
        reader.read_exact(&mut shared_secret).await?;
        let token_len = VarInt::decode_async(reader, &NetDecodeOpts::None).await?.0 as usize;
        let mut verify_token = vec![0u8; token_len];
        reader.read_exact(&mut verify_token).await?;
        Ok(Self {
            shared_secret,
            verify_token,
        })
    }
}
