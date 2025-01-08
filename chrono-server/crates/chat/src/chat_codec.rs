use crate::chat::{ChatReq, ChatRes};
use async_trait::async_trait;
use libp2p::futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use libp2p::request_response::Codec;
use libp2p::StreamProtocol;

pub const CHAT_PROTOCOL_NAME: StreamProtocol = StreamProtocol::new("/chrono/chat/1.0.0");

#[derive(Debug, Clone, Default)]
pub struct ChatCodeC {}

#[async_trait]
impl Codec for ChatCodeC {
    type Protocol = StreamProtocol;
    type Request = ChatReq;
    type Response = ChatRes;

    async fn read_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Request>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut vec = Vec::new();
        io.read_to_end(&mut vec).await?;
        Ok(serde_json::from_slice(vec.as_slice())?)
    }

    async fn read_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Response>
    where
        T: AsyncRead + Unpin + Send,
    {
        let mut vec = Vec::new();
        io.read_to_end(&mut vec).await?;
        Ok(serde_json::from_slice(vec.as_slice())?)
    }

    async fn write_request<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> std::io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data = serde_json::to_vec(&req)?;
        io.write_all(data.as_ref()).await?;
        Ok(())
    }

    async fn write_response<T>(
        &mut self,
        _protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> std::io::Result<()>
    where
        T: AsyncWrite + Unpin + Send,
    {
        let data = serde_json::to_vec(&res)?;
        io.write_all(data.as_ref()).await?;
        Ok(())
    }
}
