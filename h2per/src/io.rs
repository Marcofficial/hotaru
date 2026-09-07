use core::pin::Pin;
use core::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncWrite, BufReader, BufWriter, ReadBuf, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

pub(crate) type TcpReader = BufReader<ReadHalf<TcpStream>>;
pub(crate) type TcpWriter = BufWriter<WriteHalf<TcpStream>>;

/// Recombines Hotaru's buffered split halves into the duplex shape Hyper expects.
///
/// The reader remains the exact buffer used by Hotaru protocol detection, so bytes
/// observed by `fill_buf()` are still available when Hyper starts parsing.
pub(crate) struct DuplexIo {
    reader: TcpReader,
    writer: TcpWriter,
}

impl DuplexIo {
    pub(crate) fn new(reader: TcpReader, writer: TcpWriter) -> Self {
        Self { reader, writer }
    }
}

impl AsyncRead for DuplexIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.reader).poll_read(context, buffer)
    }
}

impl AsyncWrite for DuplexIo {
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
