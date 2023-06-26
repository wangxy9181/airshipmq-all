use std::io;
use thiserror::Error;
use crate::protocol::RemotingCommand;

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

impl From<RemotingError> for RemotingCommand {
    fn from(error: RemotingError) -> Self {
        match error {
            RemotingError::FrameOversizeError => RemotingCommand::new_error_command("Frame oversize error"),
            RemotingError::FrameEncodeError(_) => RemotingCommand::new_error_command("Frame encode error"),
            RemotingError::FrameDecodeError(_) => RemotingCommand::new_error_command("Frame decode error"),
            RemotingError::FrameCompressError(_) => RemotingCommand::new_error_command("Frame compress error"),
            RemotingError::FrameHeaderParseFail(s) => RemotingCommand::new_error_command(format!("解析 Frame header 失败: {}", s)),
        }
    }
}