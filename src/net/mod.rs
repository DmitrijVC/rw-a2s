pub mod client;
pub mod server;

use std::net::{UdpSocket, ToSocketAddrs};
use std::io;


// ToDo find different method if possible
pub trait ToUdpSocket{
    fn conn<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()>;
    fn send_packet(&self, buf: &[u8]) -> io::Result<usize>;
    fn receive_packet(&self, buf: &mut [u8]) -> io::Result<usize>;
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
}
