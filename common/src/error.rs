use thiserror::Error;

#[derive(Debug, Error)]
pub enum MQError {
    #[error("Frame encode error: {0}")]
    FrameEncodeError(&'static str),
    #[error("Frame decode error")]
    FrameDecodeError,


}