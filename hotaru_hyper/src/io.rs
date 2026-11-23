use core::pin::Pin;
use core::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, BufReader, BufWriter, ReadBuf, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

pub(crate) type TcpReader = BufReader<ReadHalf<TcpStream>>;
pub(crate) type TcpWriter = BufWriter<WriteHalf<TcpStream>>;

/// Recombines Hotaru's buffered TCP halves into the duplex I/O Hyper expects.
///
/// Keeping the existing reader preserves bytes buffered during protocol
/// detection when ownership moves from Hotaru to Hyper.
pub(crate) struct HyperDuplexIo {
    reader: TcpReader,
    writer: TcpWriter,
}

impl HyperDuplexIo {
    pub(crate) fn new(reader: TcpReader, writer: TcpWriter) -> Self {
        Self { reader, writer }
    }
}

impl AsyncRead for HyperDuplexIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.reader).poll_read(context, buffer)
    }
}

impl AsyncWrite for HyperDuplexIo {
    fn poll_write(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.writer).poll_write(context, buffer)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.writer).poll_flush(context)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.writer).poll_shutdown(context)
    }
}
