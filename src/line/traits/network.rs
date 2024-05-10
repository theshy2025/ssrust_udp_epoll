use std::os::fd::BorrowedFd;

use crate::{global, line::line_header::DataType};

use super::status::{LineTraitStatus, Status};

pub trait LineTraitNetWork : LineTraitStatus {
    fn socket_write(&mut self,_buf:&[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn socket_peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        todo!()
    }

    fn peer_ip_port(&self) -> String {
        todo!()
    }

    fn update_ip_port(&mut self,_ip_port:String){}

    fn socket_fd(&self) -> BorrowedFd<'_>;
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize>;

    fn on_network_data(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        let len = buf.len();
        self.log(format!("on network data from[{:?}]{} bytes",self.socket_peer_addr(),len));
        (0,len,DataType::Http)
    }

    fn socket_send(&mut self,buf:&[u8]) {
        let st = self.status();
        let len = buf.len();

       let hash = global::hash(buf);
        
        self.log(format!("try send {} bytes to [{:?}],hash:{:?}",len,self.socket_peer_addr(),hash));
        
        if st == Status::WriteClose || st == Status::ReadWriteBothClose || 
        st == Status::DeRegister || st == Status::Close {
            return;
        }
        
        match self.socket_write(buf) {
            Ok(n) => {
                self.log(format!("write {} bytes data to [{:?}]",n,self.socket_peer_addr()));
            },
            Err(e) => {
                self.err(format!("write data to network fail {}",e));
                panic!()
            },
        }
    }

    fn connect(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn start_connect(&mut self) {
        self.log(format!("start connecting to[{:?}]",self.peer_ip_port()));
        match self.connect() {
            Ok(_) => self.log(format!("connect done")),
            Err(e) => self.log(format!("{}",e)),
        }
    }
}