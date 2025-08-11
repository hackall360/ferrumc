use crate::encode::{NetEncode, NetEncodeOpts};
use crate::encode::errors::NetEncodeError;
use crate::net_types::network_position::NetworkPosition;
use std::io::Write;
use tokio::io::AsyncWrite;

/// Global position combining a dimension name and block position.
#[derive(Debug, Clone)]
pub struct GlobalPos<'a> {
    pub dimension_name: &'a str,
    pub position: NetworkPosition,
}

impl<'a> GlobalPos<'a> {
    pub fn new(dimension_name: &'a str, position: NetworkPosition) -> Self {
        Self { dimension_name, position }
    }
}

impl NetEncode for GlobalPos<'_> {
    fn encode<W: Write>(&self, writer: &mut W, opts: &NetEncodeOpts) -> Result<(), NetEncodeError> {
        self.dimension_name.encode(writer, opts)?;
        self.position.encode(writer, opts)?;
        Ok(())
    }

    async fn encode_async<W: AsyncWrite + Unpin>(
        &self,
        writer: &mut W,
        opts: &NetEncodeOpts,
    ) -> Result<(), NetEncodeError> {
        self.dimension_name.encode_async(writer, opts).await?;
        self.position.encode_async(writer, opts).await?;
        Ok(())
    }
}
