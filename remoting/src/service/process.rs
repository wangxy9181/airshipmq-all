use async_trait::async_trait;
use crate::protocol::RemotingCommand;

#[async_trait]
pub trait RemotingProcess: Send + Sync + 'static {

    type Error;

    async fn process(&self, cmd: RemotingCommand) -> Result<RemotingCommand, Self::Error>;
}