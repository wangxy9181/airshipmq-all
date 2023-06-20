use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemotingError {

    #[error("Frame oversize error")]
    FrameOversizeError,
    #[error("Frame encode error")]
    FrameEncodeError(#[from] prost::EncodeError),
    #[error("Frame decode error")]
    FrameDecodeError(#[from] prost::DecodeError),


}