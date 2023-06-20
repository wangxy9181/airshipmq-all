use bytes::{BufMut, BytesMut};
use prost::Message;
use tracing::error;
use crate::error::RemotingError;

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
    Self: Message + Sized
{
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), RemotingError> {
        let size = self.encoded_len();
        if size > MAX_FRAME_LEN {
            return Err(RemotingError::FrameOversizeError);
        }
        if size > COMPRESS_LIMIT {
            // 放入压缩标识
            buf.put_u8(1);
            // 压缩数据
            // 获得压缩算法的输入
            let mut buf1 = Vec::with_capacity(size);
            self.encode(&mut buf1)?;

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
        todo!()
    }
}