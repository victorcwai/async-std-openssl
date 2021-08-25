//! Stream Wrappers for SSL to extend support for some consuming Library use cases.

use async_dup::{Arc, Mutex};
use async_std::io::{Read, Result, Write};
use async_std::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::SslStream;

// Ref: https://stackoverflow.com/questions/61643574/how-can-i-clone-an-opensslsslsslstream
// "SSL / TLS logic contains state. All the clones need to agree on and update that state.
// You will need to wrap it in an Arc<Mutex<_>> or equivalent and clone that."

/// A wrapper for an SslStream that allows cloning and sending between multiple tasks/threads.
/// In most cases you will not need this, but some consumers (such as tide listeners) rely
/// on this behaviour.
#[derive(Clone)]
pub struct SslStreamWrapper(Arc<Mutex<SslStream<TcpStream>>>);

impl SslStreamWrapper {
    /// Wrap an SslStream in a clonable wrapper.
    pub fn new(stream: SslStream<TcpStream>) -> Self {
        Self(Arc::new(Mutex::new(stream)))
    }
}

impl Read for SslStreamWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut &*self.0).poll_read(cx, buf)
    }
}

impl Write for SslStreamWrapper {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        Pin::new(&mut &*self.0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut &*self.0).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut &*self.0).poll_close(cx)
    }
}
