use std::{io::{self, Read, Write}, os::fd::{AsFd, BorrowedFd}};

use crate::{global, line::{line_header::DataType, traits::network::LineTraitNetWork}, log::{self, Log}};

use super::LinePc;

#[derive(Debug)]
pub enum Step {
    Raw,
    HelloDone,
    SniDone,
    ClientHelloDone,
}

impl LineTraitNetWork for LinePc {
    fn socket_peer_addr(&self) -> io::Result<std::net::SocketAddr> {
        self.socket.peer_addr()
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        let len = buf.len();
        self.log(format!("on network data from[{:?}]{} bytes step:{:?}",self.socket.peer_addr(),len,self.step));
        match self.step {
            Step::Raw => self.s5_hello(),
            Step::HelloDone => self.s5_sni(buf),
            Step::SniDone => self.s5_client_hello(buf),
            Step::ClientHelloDone => (0,buf.len(),DataType::Http),
        }
    }
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize> {
        self.socket.read(buf)
    }
    
    fn socket_write(&mut self,buf:&[u8]) -> std::io::Result<usize> {
        self.socket.write(buf)
    }

    fn socket_fd(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }

}

impl LinePc {
    fn s5_hello(&mut self) -> (usize,usize,DataType) {
        self.log(format!("s5_hello"));
        self.socket.write(&[5,0]).unwrap();
        
        self.step = Step::HelloDone;

        (0,0,DataType::Error)
    }

    fn s5_sni(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        assert_eq!(buf[0],5);

        let host = global::decode_host_name(buf);
        
        log::def(format!("[{}]{}",self.id(),host));
        self.log(format!("{}",host));

        self.socket.write(&[5,0,0,1,0,0,0,0,0,0]).unwrap();

        self.step = Step::SniDone;
        
        crate::global::reverse(buf);

        buf[0] = 0;
        (0,buf.len(),DataType::Sni)
    }

    fn s5_client_hello(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        self.log(format!("s5_client_hello"));
        crate::global::reverse(buf);

        self.step = Step::ClientHelloDone;

        (0,buf.len(),DataType::ClientHello)
    }
}