use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemotingError {

    #[error("Frame oversize error")]
    FrameOversizeError,
    #[error("Frame encode error")]
    FrameEncodeError(#[from] prost::EncodeError),
    #[error("Frame decode error")]
    FrameDecodeError(#[from] prost::DecodeError),
    #[error("Frame compress error")]
    FrameCompressError(#[from] io::Error),

    #[error("解析 Frame header 失败: {0}")]
    FrameHeaderParseFail(String),
}