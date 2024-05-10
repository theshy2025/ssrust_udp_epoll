use std::{io::{Read, Write}, os::fd::{AsFd, BorrowedFd}};

use socket2::SockAddr;

use crate::{line::traits::network::LineTraitNetWork, log::Log};

use super::LineWorld;

impl LineTraitNetWork for LineWorld {

    fn socket_peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        match self.socket.peer_addr() {
            Ok(s) => Ok(s.as_socket().unwrap()),
            Err(e) => Err(e),
        }
    }

    fn peer_ip_port(&self) -> String {
        self.peer_ip_port.clone()
    }

    fn update_ip_port(&mut self,ip_port:String) {
        self.peer_ip_port = ip_port;
        self.log(format!("update peer_ip_port to {}",self.peer_ip_port));
    }
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize> {
        self.socket.read(buf)
    }

    fn socket_write(&mut self,buf:&[u8]) -> std::io::Result<usize> {
        self.socket.write(buf)
    }

    fn connect(&mut self) -> std::io::Result<()> {
        let addr:std::net::SocketAddr = self.peer_ip_port.parse().unwrap();
        let addr = SockAddr::from(addr);
        self.socket.connect(&addr)
    }
    
    fn socket_fd(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }
    
}
