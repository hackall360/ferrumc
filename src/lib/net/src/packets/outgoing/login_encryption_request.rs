use ferrumc_macros::packet;
use ferrumc_net_codec::encode::errors::NetEncodeError;
use ferrumc_net_codec::encode::{NetEncode, NetEncodeOpts};
use ferrumc_net_codec::net_types::byte_array::ByteArray;
use ferrumc_net_codec::net_types::var_int::VarInt;
use std::io::Write;
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[packet(packet_id = 0x01, state = "login")]
pub struct LoginEncryptionRequest<'a> {
    pub server_id: &'a str,
    pub public_key: ByteArray,
    pub verify_token: ByteArray,
}

impl<'a> NetEncode for LoginEncryptionRequest<'a> {
    fn encode<W: Write>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt::new(0x01).encode(writer, &NetEncodeOpts::None)?;
        self.server_id.encode(writer, &NetEncodeOpts::None)?;
        self.public_key.encode(writer, &NetEncodeOpts::None)?;
        self.verify_token.encode(writer, &NetEncodeOpts::None)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        _opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        VarInt::new(0x01)
            .encode_async(writer, &NetEncodeOpts::None)
            .await?;
        self.server_id
            .encode_async(writer, &NetEncodeOpts::None)
            .await?;
        self.public_key
            .encode_async(writer, &NetEncodeOpts::None)
            .await?;
        self.verify_token
            .encode_async(writer, &NetEncodeOpts::None)
            .await?;
        Ok(())
    }
}
