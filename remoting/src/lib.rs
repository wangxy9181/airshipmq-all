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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use async_trait::async_trait;
    use tokio::net::{TcpListener, TcpStream};
    use crate::error::RemotingError;
    use crate::network::RemotingStream;
    use crate::protocol::{Command, RemotingCommand};
    use crate::service::RemotingProcess;

    #[tokio::test]
    async fn server_should_work() -> Result<()> {
        let addr = "0.0.0.0:8080";
        let config = ServerConfig::new(addr);
        start_server(DummyProcess, &config).await?;

        let mut stream = TcpStream::connect(addr).await?;
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


        Ok(())
    }

    struct DummyProcess;

    #[async_trait]
    impl RemotingProcess for DummyProcess {
        type Error = RemotingError;

        async fn process(&self, cmd: RemotingCommand) -> std::result::Result<RemotingCommand, Self::Error> {
            if cmd.command.is_none() {
                return Err(RemotingError::NoCommandError);
            }
            let command = cmd.command.unwrap();
            match command {
                Command::BrokerRegisterRequest(request) => {
                    let response = RemotingCommand::new_success_response();
                    Ok(response)
                }
                _=> Ok(RemotingError::NoCommandError.into()),
            }
        }
    }
}

