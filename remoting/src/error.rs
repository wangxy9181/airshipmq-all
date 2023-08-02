use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemotingError {
    #[error("Frame error: {0}")]
    FrameError(&'static str),
    #[error("Frame encode error")]
    FrameEncodeError(#[from] prost::EncodeError),
    #[error("Frame decode error")]
    FrameDecodeError(#[from] prost::DecodeError),

    #[error("Failed because io error")]
    IoErroe(#[from] std::io::Error),
}