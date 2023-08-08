mod processor;

#[cfg(test)]
mod tests {
    use tokio::net::TcpStream;
    use tokio_stream::StreamExt;
    use crate::network::RemotingStream;
    use crate::service::processor::RemotingProcessor;

    #[tokio::test]
    async fn remoting_service_should_work() {

        // 创建一个 RemotingProcessor：业务处理逻辑
        // let processor = DummyProcessor::new();
        // // 创建一个 RemotingService
        // let service = RemotingService::new(processor);
        // // 启动 service
        // service.execute().await?;

        // 创建一个
    }
}