use thiserror::Error;

#[derive(Debug, Error)]
pub enum MQError {
    #[error("Frame encode error")]
    FrameEncodeError,
    #[error("Frame decode error")]
    FrameDecodeError,


}