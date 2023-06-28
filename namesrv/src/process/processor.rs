use async_trait::async_trait;
use remoting::{RemotingCommand, RemotingError, RemotingProcess};

pub struct NamesrvProcessor;

#[async_trait]
impl RemotingProcess for NamesrvProcessor {

    async fn process(&self, cmd: RemotingCommand) -> Result<RemotingCommand, RemotingError> {
        todo!()
    }
}