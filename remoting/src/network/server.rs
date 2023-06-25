use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use crate::error::RemotingError;
use crate::network::process::RemotingProcess;
use crate::network::stream::RemotingStream;

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    ip: String,
    port: i32,
}

pub struct RemotingServer<Processor>
{
    processor: Processor,

}

/// 启动服务端应用程序
pub fn start_server(config: ServerConfig, processor: Arc<impl RemotingProcess>)
    -> Result<(), RemotingError>
{
    tokio::spawn(async move {
        let listener = TcpListener::bind(get_ip_address(&config)).await?;
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("接收到新的连接，连接地址: {}", addr);
            // 连接处理
            tokio::spawn(async move {

            });
        }
        Ok::<_, RemotingError>(())
    });
    Ok(())
}

fn get_ip_address(config: &ServerConfig) -> String {
    let mut address = String::new();
    address.push_str(config.ip.as_str());
    address.push_str(config.port.to_string().as_str());
    address
}