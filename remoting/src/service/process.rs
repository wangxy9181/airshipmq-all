use async_trait::async_trait;
use crate::error::RemotingError;
use crate::protocol::RemotingCommand;

#[async_trait]
pub trait RemotingProcess: Send + Sync + 'static {

    async fn process(&self, cmd: RemotingCommand) -> Result<RemotingCommand, RemotingError>;
}