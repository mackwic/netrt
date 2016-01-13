/*
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

pub use std::net::{SocketAddr, ToSocketAddrs};
use std::net::UdpSocket;
use std::time::Duration;
use std::io::Result;

/// The Socket structure mirrors `std::net::UdpSocket` and implement some
/// useful traits
#[derive(Debug)]
pub struct Socket {
    _socket: UdpSocket,
    blocking: bool
}

impl Socket {
    /// Creates a UDP socket from the given address.
    ///
    /// The address type can be any implementor of ToSocketAddr trait. See its
    /// documentation for concrete examples.
    pub fn bind<A: ToSocketAddrs>(address : A) -> Result<Socket> {
        UdpSocket::bind(address).map(|sock| Socket { _socket: sock, blocking: true })
    }

    /// Receives data from the socket. On success, returns the number of bytes
    /// read and the address from whence the data came.
    pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)> {
        self._socket.recv_from(buf)
    }

    /// Sends data on the socket to the given address. On success, returns the
    /// number of bytes written.
    /// Address type can be any implementor of ToSocketAddrs trait. See its
    /// documentation for concrete examples.
    pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> Result<usize> {
        self._socket.send_to(buf, addr)
    }

    /// Returns the socket address that this socket was created from.
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self._socket.local_addr()
    }

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned `Socket` is a reference to the same socket that this object
    /// references. Both handles will read and write the same port, and options set
    /// on one socket will be propagated to the other.
    pub fn try_clone(&self) -> Result<Socket> {
        self._socket.try_clone().map(|sock| Socket { _socket: sock, blocking: true })
    }


    /// Sets the read timeout to the timeout specified.
    ///
    /// If the value specified is None, then read calls will block indefinitely. It is an error to pass the zero Duration to this method.
    ///
    /// # Note
    /// Platforms may return a different error code whenever a read times out as a result of setting this option. For example Unix typically returns an error of the kind WouldBlock, but Windows may return TimedOut.
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()> {
        self._socket.set_read_timeout(dur)
    }

    /// Sets the write timeout to the timeout specified.
    ///
    /// If the value specified is `None`, then `write` calls will block
    /// indefinitely. It is an error to pass the zero `Duration` to this
    /// method.
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> Result<()> {
        self._socket.set_write_timeout(dur)
    }


    /// Returns the read timeout of this socket.
    ///
    /// If the timeout is `None`, then `read` calls will block indefinitely.
    pub fn read_timeout(&self) -> Result<Option<Duration>> {
        self._socket.read_timeout()
    }

    /// Returns the write timeout of this socket.
    ///
    /// If the timeout is `None`, then `write` calls will block indefinitely.
    pub fn write_timeout(&self) -> Result<Option<Duration>> {
        self._socket.write_timeout()
    }

    /// Simple getter on the blocking state of the underlying socket.
    pub fn blocking(&self) -> bool {
        self.blocking
    }

    /// `set_blocking(false)` set the socket in non-blicking mode, which is
    /// the read and write timeout the smallest possible depending of the platform
    pub fn set_blocking(&mut self, blocking: bool) -> Result<()> {

        // TODO problems of atomicity here. If read_timeout succeed but
        // write_timeout fails, do we need to rollback read_timeout ?

        if blocking {
            let duration = Duration::new(0, 1);
            self
                .set_read_timeout(Some(duration))
                .and_then(|()| self.set_write_timeout(Some(duration)))
                .and_then(|()| { self.blocking = true ; Ok(()) })
        } else {
            self
                .set_read_timeout(None)
                .and_then(|()| self.set_write_timeout(None))
                .and_then(|()| {self.blocking = false ; Ok(()) })
        }
    }
}
