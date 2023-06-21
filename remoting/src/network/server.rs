use tokio::net::TcpListener;
use tracing::info;
use crate::error::RemotingError;

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    ip: String,
    port: i32,
}

/// 启动服务端应用程序
pub async fn start_server(config: ServerConfig) -> Result<(), RemotingError> {
    let listener = TcpListener::bind(get_ip_address(&config)).await?;
    tokio::spawn(async move {
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("接收到新的连接，连接地址: {}", addr);
            // 新建任务处理新的连接
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