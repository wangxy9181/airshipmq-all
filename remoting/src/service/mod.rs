use std::sync::Arc;
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
}

impl<Processor: RemotingProcess> Clone for RemotingService<Processor> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}