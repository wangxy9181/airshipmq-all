use async_trait::async_trait;
use remoting::{RemotingCommand, RemotingError, RemotingProcess};

pub struct NamesrvProcessor;

impl NamesrvProcessor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RemotingProcess for NamesrvProcessor {

    async fn process(&self, cmd: RemotingCommand) -> Result<RemotingCommand, RemotingError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use chrono::Local;

    #[tokio::test]
    async fn process_should_work() -> Result<()> {
        // 创建
        // 创建 Namesrv
        let namesrv = NamesrvProcessor::new();
        //
        Ok(())
    }
}