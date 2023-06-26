use std::sync::Arc;
use futures::StreamExt;
use tokio::net::{TcpListener};
use tokio::sync::mpsc;
use tokio_stream::wrappers::{UnboundedReceiverStream};
use tracing::{debug, error, info};
use crate::error::RemotingError;
use crate::network::process::RemotingProcess;
use crate::network::stream::RemotingStream;
use crate::protocol::RemotingCommand;

/// 服务端配置
#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    ip: String,
    port: i32,
}

/// RemotingServer 表示服务端程序
pub struct RemotingServer<Processor: RemotingProcess>
{
    processor: Arc<Processor>,
    config: ServerConfig,
}

impl<Processor: RemotingProcess> RemotingServer<Processor> {
    /// 创建 RemotingServer
    pub fn new(config: ServerConfig, processor: Processor) -> Self {
        Self {
            processor: Arc::new(processor),
            config,
        }
    }

    /// 启动 RemotingServer
    pub fn start(self) -> Result<(), RemotingError> {
        tokio::spawn(async move {
            let ip_address = get_ip_address(&self.config);
            let listener = TcpListener::bind(&ip_address).await?;
            info!("Start listening at {}", ip_address);
            loop {
                let (mut stream, addr) = listener.accept().await?;
                info!("Client {} connected!", addr);
                // 通道，用于数据同步
                let (sender, recv) = mpsc::unbounded_channel();
                sender.send(RemotingCommand::default());
                // 拆分成 读写 stream
                let (read_stream, write_stream) = stream.split();
                let processor = self.processor.clone();
                // 读
                tokio::spawn(async move {
                    let mut remoting_stream: RemotingStream<_, RemotingCommand, RemotingCommand> = RemotingStream::new(read_stream);
                    while let Some(result) = remoting_stream.next().await {
                        let deal_result = match result {
                            Ok(command_request) => {
                                debug!("收到来自 {} 的请求：{:?}", addr, command_request);
                                let command_response = processor.process(command_request).await?;
                                debug!("来自 {} 的请求处理完成，处理结果为: {:?}", addr, command_response);
                                sender.send(command_response)
                            },
                            Err(err) => {
                                let command_response: RemotingCommand = err.into();
                                error!("Read command error: {:?}, from : {}", err,addr);
                                sender.send(command_response)
                            }
                        };
                        if let Err(err) = deal_result {
                            info!("Command response channel 发送失败，重新发送开始...");
                            let result = sender.send(err.0);
                            info!("Command response channel 发送失败，重新发送结束，发送结果: {:?}", result);
                        }
                    }
                    info!("Client {} disconnected!", addr);
                    Ok(())
                });
                // 写
                tokio::spawn(async move {
                    let recv_stream = UnboundedReceiverStream::new(recv);
                });
            }
            Ok::<_, RemotingError>(())
        });

        Ok(())
    }
}

/// 启动服务端应用程序
pub fn start_server(config: ServerConfig, processor: impl RemotingProcess)
    -> Result<(), RemotingError>
{

    Ok(())
}

fn get_ip_address(config: &ServerConfig) -> String {
    let mut address = String::new();
    address.push_str(config.ip.as_str());
    address.push_str(config.port.to_string().as_str());
    address
}