use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use bytes::{BufMut, BytesMut};
use futures::{ready, Sink, Stream, FutureExt};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use crate::error::RemotingError;
use crate::network::frame::{FrameCoder, read_frame};

pub struct RemotingStream<Message, Stream> {
    stream: Stream,
    write_buf: BytesMut,
    read_buf: BytesMut,
    written: usize,
    _message: PhantomData<Message>,
}

impl<Message, Stream> RemotingStream<Message, Stream>
where
    Message: FrameCoder,
    Stream: AsyncRead + AsyncWrite + Send + Sync
{
    pub fn new(stream: Stream) -> Self {
        Self {
            stream,
            write_buf: BytesMut::new(),
            read_buf: BytesMut::new(),
            written: 0usize,
            _message: PhantomData::default(),
        }
    }
}

impl<Message, Stream> Sink<Message> for RemotingStream<Message, Stream>
where
    Message: FrameCoder + Unpin,
    Stream: AsyncWrite + Unpin
{
    type Error = RemotingError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        item.encode_frame(&mut self.write_buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.get_mut();
        let len = this.write_buf.len();
        while this.written < len {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, &this.write_buf[this.written..]))?;
            this.written += n;
        }
        // 清除 buf
        this.write_buf.clear();
        this.written = 0;

        Poll::Ready(Ok(()))
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // 确保写入
        let pin = self.as_mut();
        ready!(self.as_mut().poll_flush(cx)?);
        // 关闭 stream
        ready!(Pin::new(&mut self.stream).poll_shutdown(cx)?);

        Poll::Ready(Ok(()))
    }
}

impl<Message, S> Stream for RemotingStream<Message, S>
where
    Message: FrameCoder + Unpin,
    S: AsyncRead + Unpin
{
    type Item = Result<Message, RemotingError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let mut buf = this.read_buf.split_off(0);
        let future = read_frame(&mut this.stream, &mut buf);
        ready!(Box::pin(future).poll_unpin(cx))?;

        this.read_buf.unsplit(buf);
        // 清除数据
        Poll::Ready(Some(Ok(Message::decode_frame(&mut this.read_buf)?)))
    }
}

#[cfg(test)]
mod tests {

    use anyhow::Result;
    use futures::{SinkExt, StreamExt};
    use tokio::net::{TcpListener, TcpStream};
    use crate::pb::{RemotingCommand};
    use crate::pb::remoting_command::{Data};
    use super::*;

    #[tokio::test]
    async fn remoting_stream_should_work() -> Result<()> {
        let addr = "0.0.0.0:8090";
        // 在指定地址上面创建一个 dummy server
        start_dummy_server(addr).await?;
        // 连接指定地址
        let stream = TcpStream::connect(addr).await?;
        // 根据 TcpStream 创建一个 RemotingStream
        let mut stream = RemotingStream::new(stream);

        let mut counter = 0;

        loop {
            if counter > 500 {
                break;
            }

            // 构建 RemotingCommand
            let broker_id: i64 = 0;
            let cmd = RemotingCommand::new_broker_register_request("broker_name", "broker_addr", broker_id);
            let command_id = cmd.id;
            // 发送数据
            stream.send(cmd).await?;
            // 接受响应
            let option = stream.next().await.unwrap();
            let cmd = option?;

            assert_eq!(cmd, RemotingCommand::broker_register_success(command_id));
            counter += 1;
        }

        Ok(())
    }

    async fn start_dummy_server(addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;

        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let mut remoting_stream: RemotingStream<RemotingCommand, _> = RemotingStream::new(stream);

                tokio::spawn(async move {
                    while let Some(Ok(command)) = remoting_stream.next().await {
                        let data = command.data;
                        if let Some(data) = data {
                            let command_id = command.id;
                            let response = match data {
                                Data::BrokerRegisterRequest(request) => {
                                    RemotingCommand::broker_register_success(command_id)
                                }
                                _ => RemotingCommand::command_not_support(command_id),
                            };
                            remoting_stream.send(response).await?;
                        }
                    }
                    Ok::<_, RemotingError>(())
                });
            }
        });

        Ok(())
    }
}