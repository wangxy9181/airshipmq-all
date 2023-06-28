use std::sync::Arc;
use tracing::{debug};
use crate::protocol::RemotingCommand;
pub use crate::service::process::RemotingProcess;

mod process;

/// RemotingService 表示服务端业务处理程序
pub struct RemotingService<Processor> {
    inner: Arc<RemotingServiceInner<Processor>>,
}

/// 存放实际数据的内部结构
struct RemotingServiceInner<Processor> {
    processor: Processor,
}

impl<Processor> RemotingServiceInner<Processor> {
    fn new(processor: Processor) -> Self {
        Self {
            processor
        }
    }
}

impl<Processor: RemotingProcess> RemotingService<Processor> {
    pub fn new(processor: Processor) -> Self {
        Self {
            inner: Arc::new(RemotingServiceInner::new(processor)),
        }
    }

    pub async fn execute(&self, remoting_command: RemotingCommand) -> RemotingCommand {
        let command_id = remoting_command.id;
        debug!("remoting command id: {}, remoting command: {:?}", command_id, remoting_command);
        let result = self.inner.processor.process(remoting_command).await;
        match result {
            Ok(command) => {
                debug!("remoting command 处理成功，处理结果: {:?}", command);
                command
            },
            Err(remoting_error) => {
                debug!("remoting command id: {} 处理失败: {:?}", command_id, remoting_error);
                RemotingCommand::from(remoting_error)
            }
        }
    }
}

impl<Processor: RemotingProcess> Clone for RemotingService<Processor> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}