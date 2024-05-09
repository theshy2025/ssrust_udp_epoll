use std::{net::SocketAddrV4, os::fd::BorrowedFd};

use socket2::SockAddr;

use crate::line::line_enum::DataType;

use super::status::{LineTraitStatus, Status};

pub trait LineTraitNetWork : LineTraitStatus {
    fn socket_write(&mut self,_buf:&[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn peer_ip_port(&self) -> String {String::new()}
    fn update_ip_port(&mut self,_ip_port:String){}

    fn socket_fd(&self) -> BorrowedFd<'_>;
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize>;

    fn on_network_data(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        let len = buf.len();
        self.log(format!("on network data from[{}]{} bytes",self.peer_ip_port(),len));
        (0,len,DataType::Http)
    }

    fn socket_send(&mut self,buf:&[u8]) {
        let st = self.status();
        let len = buf.len();
        let addr = self.peer_ip_port();
        self.log(format!("try socket_send {} bytes to [{}]",len,addr));
        
        
        if st == Status::WriteClose || st == Status::ReadWriteBothClose || 
        st == Status::DeRegister || st == Status::Close {
            return;
        }
        
        match self.socket_write(buf) {
            Ok(n) => {
                self.log(format!("write {} bytes data to [{}]",n,self.peer_ip_port()));
            },
            Err(e) => crate::log::err(format!("write data to network fail {}",e)),
        }
    }

    fn connect(&mut self,_address: &SockAddr) -> std::io::Result<()> {
        todo!()
    }

    fn start_connect(&mut self) {
        let ip_port = self.peer_ip_port();
        self.log(format!("start connecting to {} ",ip_port));
        let address:SocketAddrV4 = ip_port.parse().unwrap();
        match self.connect(&address.into()) {
            Ok(_) => self.log(format!("connect done")),
            Err(e) => self.log(format!("{},{}",address,e)),
        }
    }
}