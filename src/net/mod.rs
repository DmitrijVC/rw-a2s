pub mod client;
pub mod server;

use std::net::{UdpSocket, ToSocketAddrs};
use std::io;
use std::time::Duration;


// ToDo find different method if possible
pub trait ToUdpSocket{
    fn conn<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()>;
    fn send_packet(&self, buf: &[u8]) -> io::Result<usize>;
    fn receive_packet(&self, buf: &mut [u8]) -> io::Result<usize>;
    fn write_timeout(&self, dur: Option<Duration>) -> io::Result<()>;
    fn read_timeout(&self, dur: Option<Duration>) -> io::Result<()>;
}

impl ToUdpSocket for UdpSocket {
    fn conn<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        self.connect(addr)
    }

    fn send_packet(&self, buf: &[u8]) -> io::Result<usize> {
        self.send(buf)
    }

    fn receive_packet(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv(buf)
    }

    fn write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.set_write_timeout(dur)
    }

    fn read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.set_read_timeout(dur)
    }
}

impl ToUdpSocket for &UdpSocket {
    fn conn<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        self.connect(addr)
    }

    fn send_packet(&self, buf: &[u8]) -> io::Result<usize> {
        self.send(buf)
    }

    fn receive_packet(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv(buf)
    }

    fn write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.set_write_timeout(dur)
    }

    fn read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.set_read_timeout(dur)
    }
}
