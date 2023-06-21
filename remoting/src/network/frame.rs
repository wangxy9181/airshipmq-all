use std::io::{Read, Write};
use bytes::{Buf, BufMut, BytesMut};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt};
use tracing::error;
use crate::error::RemotingError;
use crate::protocol::RemotingCommand;

/// frame header 长度，1 字节压缩标识，4 字节长度
const FRAME_HEADER_LEN: usize = 5;
/// frame 最大长度位 4G
const MAX_FRAME_LEN: usize = 4 * 1024 * 1024 * 1024 - 1;
/// 压缩阈值
const COMPRESS_LIMIT: usize = 1436;

// trait 中添加默认方法
// 增加 Self 约束，为 protobuf 自动实现此 trait
pub trait FrameCoder
where
    Self: Message + Sized + Default
{
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), RemotingError> {
        let size = self.encoded_len();
        if size > MAX_FRAME_LEN {
            return Err(RemotingError::FrameOversizeError);
        }
        if size > COMPRESS_LIMIT {

            // 压缩数据
            // 获得压缩算法的输入
            let mut buf1 = Vec::with_capacity(size);
            self.encode(&mut buf1)?;

            // BytesMut 支持逻辑上的 split（之后还能 unsplit）
            // split_off 后
            // payload 获得 [at, capacity) 元素
            // self（即 buf）或者 [0, at) 元素
            let payload = buf.split_off(FRAME_HEADER_LEN);

            // 处理压缩，具体使用可以参考 flate2 文档
            let mut compression = GzEncoder::new(payload.writer(),
                                                 Compression::default());
            compression.write_all(&buf1[..])?;
            // 压缩完成后，拿回 payload
            let payload = compression.finish()?.into_inner();
            // 放入压缩标识
            buf.put_u8(1);
            // 写入长度
            buf.put_u32(payload.len() as _);
            // 合并 buf
            buf.unsplit(payload);
        } else {
            // 放入压缩标识
            buf.put_u8(0);
            // 放入数据长度
            buf.put_u32(size as _);
            // command 编码
            self.encode(buf)?;
        }

        Ok(())
    }

    fn decode_frame(buf: &mut BytesMut) -> Result<Self, RemotingError> {
        // 解析头部数据
        let (compressed, len) = decode_header(buf)?;
        // 数据压缩了
        if compressed == 1 {
            let mut decoder = GzDecoder::new(&buf[..len]);
            let mut buf1 = Vec::with_capacity(len * 2);
            decoder.read_to_end(&mut buf1)?;
            // 将 buf 向前推进 len，跳过已经读取的数据，因为上面读取的是 buf 的一个切片
            buf.advance(len);

            Ok(Self::decode(&buf1[..buf1.len()])?)
        } else {
            let command = Self::decode(&buf[..len])?;
            // 将 buf 向前推进 len，跳过已经读取的数据，因为上面读取的是 buf 的一个切片
            buf.advance(len);
            return Ok(command);
        }
    }
}

impl FrameCoder for RemotingCommand {}

fn decode_header(buf: &mut BytesMut) -> Result<(u8, usize), RemotingError> {
    if buf.len() < FRAME_HEADER_LEN {
        error!("buf 可读长度小于帧头部长度，解析帧头部失败");
        return Err(RemotingError::FrameHeaderParseFail("buf 可读长度小于帧头部长度，解析帧头部失败".to_string()))
    }
    // 读取压缩标识
    let compressed = buf.get_u8();
    // 读取长度标识
    let len = buf.get_u32() as usize;
    Ok((compressed, len))
}

pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), RemotingError>
where
    S: AsyncRead + Send + Unpin
{
    let compressed = stream.read_u8().await?;
    let len = stream.read_u32().await?;
    // 分配内存，保证能够读取一个 frame
    buf.reserve(FRAME_HEADER_LEN + (len as usize));
    // 写入 Frame header
    buf.put_u8(compressed);
    buf.put_u32(len);
    // 将 buf 向前推进 len 的长度，因为后面会初始化，所以没有关系
    unsafe {
        buf.advance_mut(len as _);
    }
    // 读取数据
    stream.read_exact(&mut buf[FRAME_HEADER_LEN..]).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use bytes::{Buf, BufMut, BytesMut};
    use crate::error::RemotingError;
    use crate::network::frame::FrameCoder;
    use crate::protocol::RemotingCommand;

    #[test]
    fn frame_codec_should_world() -> Result<(), RemotingError> {
        let broker_name = "broker_name";
        let broker_addr = "broker_addr";
        let cluster_name = "cluster_name";
        let ha_server_addr = "ha_server_addr";
        let broker_id = 0;
        let heartbeat_timeout_mills = 1000;
        let enable_acting_master = true;
        let remoting_command = RemotingCommand::new_broker_register_request(broker_name, broker_addr, cluster_name,
                                                                            ha_server_addr, broker_id, heartbeat_timeout_mills, enable_acting_master);
        let mut buf = BytesMut::new();
        // 编码
        remoting_command.encode_frame(&mut buf)?;
        // 解码
        let command = RemotingCommand::decode_frame(&mut buf)?;

        assert_eq!(command, remoting_command);

        Ok(())
    }

    #[test]
    fn bytes_mut_slice_should_woek() {
        let mut buf = BytesMut::new();
        buf.put_u8(1);
        buf.put_u8(2);
        buf.put_u8(3);
        buf.put_u8(4);
        buf.put_u8(5);

        assert_eq!(buf.len(), 5);

        let mut buf1 = &buf[..3];

        assert_eq!(buf1.len(), 3);
        assert_eq!(buf1.get_u8(), 1);
        assert_eq!(buf1.get_u8(), 2);
        assert_eq!(buf1.get_u8(), 3);
        // 对切片的读取不会影响到原 buf
        assert_eq!(buf.len(), 5);
        assert_eq!(buf.get_u8(), 1);
        assert_eq!(buf.get_u8(), 2);
        assert_eq!(buf.get_u8(), 3);
        assert_eq!(buf.get_u8(), 4);
        assert_eq!(buf.get_u8(), 5);
    }

}