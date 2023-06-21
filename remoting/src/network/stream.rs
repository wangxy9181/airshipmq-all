use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use bytes::BytesMut;
use futures::{ready, Stream, FutureExt, Sink};
use tokio::io::{AsyncRead, AsyncWrite};
use crate::error::RemotingError;
use crate::network::frame::{FrameCoder, read_frame};

pub struct RemotingStream<S, In, Out> {
    stream: S,
    // 读缓存
    read_buf: BytesMut,
    // 写缓存
    write_buf: BytesMut,
    // 已写字节数量
    written: usize,

    // 占位数据
    _in: PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<S, In, Out>  Stream for RemotingStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    In: Send + Unpin,
    Out: Unpin + Send +FrameCoder
{
    type Item = Result<Out, RemotingError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 开始读取时，read_buf 应该为 0
        assert_eq!(self.read_buf.len(), 0);

        // 分离出一个 rest（摆脱对 self 的引用）
        let mut rest = self.read_buf.split_off(0);

        // 读取 frame 到 read_buf
        let fut = read_frame(&mut self.stream, &mut rest);
        // 执行 fut
        ready!(Box::pin(fut).poll_unpin(cx))?;

        // 还原 read_buf
        self.read_buf.unsplit(rest);

        Poll::Ready(Some(Ok(Out::decode_frame(&mut self.read_buf)?)))
    }
}

impl<S, In, Out> Sink<In> for RemotingStream<S, In, Out>
where
    S: AsyncWrite + Send + Unpin,
    In: FrameCoder + Unpin + Send,
    Out: Unpin + Send
{
    type Error = RemotingError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: In) -> Result<(), Self::Error> {
        item.encode_frame(&mut self.write_buf)?;
        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let this = self.get_mut();
        let len = this.write_buf.len();
        while this.written != len {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, &this.write_buf[this.written..])?);
            this.written += n;
        }
        this.written = 0;
        this.write_buf.clear();

        // 调用 stream 的 flush，确保写入
        ready!(Pin::new(&mut this.stream).poll_flush(cx)?);

        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
}