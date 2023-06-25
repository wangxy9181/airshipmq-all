use async_trait::async_trait;
use crate::protocol::RemotingCommand;

#[async_trait]
pub trait RemotingProcess {

    type Error;

    async fn process(cmd: RemotingCommand) -> Result<RemotingCommand, Self::Error>;
}