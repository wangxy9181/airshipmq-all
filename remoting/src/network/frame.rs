use bytes::{Buf, BufMut, BytesMut};
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::error::RemotingError;
use crate::pb::RemotingCommand;

/// Frame 头部长度为 5 字节
pub const FRAME_HEADER_LEN: usize = 5;
/// Frame 最大的数据量：因为数据长度使用 4 字节表示，所以最大长度为 2^32 -1，能表示 4G
pub const FRAME_MAX_CONTENT_LEN: usize = 1024 * 1024 * 1024 * 4 - 1;

pub trait FrameCoder
where
    Self: Message + Sized + Default
{
    /// 将 message 编码成 frame
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), RemotingError> {
        let len = self.encoded_len();
        if len > FRAME_MAX_CONTENT_LEN {
            return Err(RemotingError::FrameError("data is too big"));
        }
        // 开始写入头部
        // 写入一字节，作为冗余
        buf.put_u8(0);
        // 写入长度
        buf.put_u32(len as _);
        // 写入数据
        self.encode(buf)?;
        Ok(())
    }

    /// 将 buf 中的数据解码成 frame
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, RemotingError> {
        // 读取头部，第一个字节为冗余
        let _ = buf.get_u8();
        let len = buf.get_u32();
        let remoting_command = Self::decode(&buf[..len as _])?;
        // 跳过已经读取过的数据：因为上面读取时用的是切片，不会影响到 buf，所以这里要进行处理
        buf.advance(len as _);
        Ok(remoting_command)
    }
}

impl FrameCoder for RemotingCommand {}

pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), RemotingError>
where
    S: AsyncRead + Unpin,
{
    let b = stream.read_u8().await?;
    // 读取长度
    let len = stream.read_u32().await?;
    buf.reserve(len as usize + FRAME_HEADER_LEN);

    buf.put_u8(b);
    buf.put_u32(len);

    unsafe {
        buf.advance_mut(len as _);
    }
    // 读取指定长度的数据
    stream.read_exact(&mut buf[FRAME_HEADER_LEN..]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use crate::pb::RemotingCommand;
    use super::*;

    #[tokio::test]
    async fn encode_and_decode_should_work() -> Result<()> {
        // 创建一个 command
        let broker_name = "broker_name";
        let broker_addr = "broker_addr";
        let broker_id = 1i64;
        let cmd1 = RemotingCommand::new_broker_register_request(broker_name, broker_addr, broker_id);
        // 编码
        let mut buf = BytesMut::new();
        cmd1.encode_frame(&mut buf)?;
        // 解码
        let cmd2 = RemotingCommand::decode_frame(&mut buf)?;

        assert_eq!(cmd1, cmd2);

        Ok(())
    }

}