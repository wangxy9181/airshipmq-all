use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing::{debug, error, info};
use crate::network::RemotingStream;
use crate::service::RemotingService;

pub use crate::service::RemotingProcess;
pub use crate::error::RemotingError;
pub use crate::protocol::RemotingCommand;

mod protocol;
mod network;
mod error;
mod service;

/// 服务端配置
#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    addr: String
}

impl ServerConfig {
    pub fn new(addr: impl Into<String>) -> Self {
        Self {
            addr: addr.into(),
        }
    }
}

pub async fn start_server_with_config(processor: impl RemotingProcess, config: &ServerConfig) -> Result<(), RemotingError> {
    let addr = config.addr.clone();
    start_server(processor, addr).await?;
    Ok(())
}

async fn start_server(processor: impl RemotingProcess + Sized, addr: String) -> Result<(), RemotingError> {
    let remoting_service = RemotingService::new(processor);
    let listener = TcpListener::bind(&addr).await?;
    info!("Start listening at {}", addr);
    loop {
        // 接收新连接
        let (stream, client_addr) = listener.accept().await?;
        info!("Client {} connected", client_addr);
        let cloned_remoting_service = remoting_service.clone();
        // 读写分离
        let (read_half, write_half) = stream.into_split();
        // channel 用于读写任务之间的通信
        let (sender, recv) = mpsc::unbounded_channel();
        // 新建任务处理新的连接的读取
        tokio::spawn(async move {
            let cloned_sender = sender.clone();
            // 创建 RemotingStream，方便读取和解码数据
            let mut remoting_stream: RemotingStream<_, RemotingCommand, RemotingCommand> =
                RemotingStream::new(read_half);
            while let Some(data) = remoting_stream.next().await {
                match data {
                    Ok(command) => {
                        let cloned_sender = cloned_sender.clone();
                        let cloned = cloned_remoting_service.clone();
                        tokio::spawn(async move {
                            let response_command = cloned.execute(command).await;
                            send_response_to_channel(&cloned_sender, response_command);
                        });
                    }
                    Err(err) => {
                        let response_command = RemotingCommand::from(err);
                        send_response_to_channel(&cloned_sender, response_command);
                    }
                }
            }
            info!("Client {} disconnected", client_addr);
        });
        // 新建任务处理新的连接的写入
        tokio::spawn(async move {
            // 创建 RemotingStream，方便发送和编码数据
            let mut write_remoting_stream: RemotingStream<_, RemotingCommand, RemotingCommand> = RemotingStream::new(write_half);
            let mut receiver_stream = UnboundedReceiverStream::new(recv);
            while let Some(command) = receiver_stream.next().await {
                let command_id = command.id;
                debug!("待发送 remoting command: {:?}", command);
                let write_result = write_remoting_stream.send(command).await;
                error!("remoting command id: {} 数据发送失败：{:?}", command_id, write_result);
            }
        });
    }
}

fn send_response_to_channel(sender: &UnboundedSender<RemotingCommand>, command: RemotingCommand) {
    let result = sender.send(command);
    if let Err(send_error) = result {
        let command = send_error.0;
        let command_id = command.id;
        info!("发送 remoting command 到 channel 失败，尝试重新发送，remoting command: {:?}", command);
        let result = sender.send(command);
        match result {
            Ok(_) => {
                info!("重新发送 remoting command 成功, remoting command id: {}", command_id)
            }
            Err(_) => {
                error!("重新发送 remoting command 失败, remoting command id: {}", command_id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use async_trait::async_trait;
    use futures::{SinkExt, StreamExt};
    use tokio::net::{TcpStream};
    use crate::error::RemotingError;
    use crate::network::RemotingStream;
    use crate::protocol::{Command, RemotingCommand};
    use crate::service::RemotingProcess;

    #[tokio::test]
    async fn server_should_work() -> Result<()> {
        let addr = "localhost:8080";
        let config = ServerConfig::new(addr);
        tokio::spawn(async move {
            start_server_with_config(DummyProcess, &config).await.unwrap();
        });

        let stream = TcpStream::connect(addr).await?;
        let mut stream: RemotingStream<TcpStream, RemotingCommand, RemotingCommand> = RemotingStream::new(stream);
        // 创建 broker_register 命令
        let broker_name = "broker_name";
        let broker_addr = "broker_addr";
        let cluster_name = "cluster_name";
        let ha_server_addr = "ha_server_addr";
        let broker_id = 0;
        let heartbeat_timeout_mills = 1000;
        let enable_acting_master = true;
        let remoting_command = RemotingCommand::new_broker_register_request(
            broker_name, broker_addr, cluster_name, ha_server_addr, broker_id,
            heartbeat_timeout_mills, enable_acting_master);
        // 发送命令
        stream.send(remoting_command).await?;
        // 接收返回处理
        let response_command = stream.next().await
            .map_or(Ok(Default::default()), |result| result)
            .map_or(Default::default(), |command| command);

        println!("接收到命令: {:?}", response_command);
        assert_eq!(response_command, RemotingCommand::new_success_response());

        Ok(())
    }

    struct DummyProcess;

    #[allow(unreachable_patterns)]
    #[async_trait]
    impl RemotingProcess for DummyProcess {

        async fn process(&self, cmd: RemotingCommand) -> std::result::Result<RemotingCommand, RemotingError> {
            if cmd.command.is_none() {
                return Err(RemotingError::NoCommandError);
            }
            let command = cmd.command.unwrap();
            match command {
                Command::BrokerRegisterRequest(_) => {
                    let response = RemotingCommand::new_success_response();
                    Ok(response)
                }
                _=> Ok(RemotingError::NoCommandError.into()),
            }
        }
    }
}

