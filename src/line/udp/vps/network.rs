use std::os::fd::{AsFd, BorrowedFd};

use crate::{config, line::{line_enum::DataType, traits::{heart_beat::LineTraitHeartBeat, network::LineTraitNetWork, status::{LineTraitStatus, Status}, tunnel::LineTraitTunnel}}, log::{self, Log}};

use super::LineUdp2Vps;

impl LineTraitNetWork for LineUdp2Vps {
    fn peer_ip_port(&self) -> String {
        self.peer_ip_port.clone()
    }

    fn update_ip_port(&mut self,ip_port:String) {
        self.peer_ip_port = ip_port;
    }
    
    fn socket_send(&mut self,buf:&[u8]) {
        self.log(format!("udp send {} bytes to [{}]",buf.len(),self.peer_ip_port()));
        self.socket.send_to(buf,self.peer_ip_port()).unwrap();
    }
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize> {
        self.socket.recv(buf)
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        let len = buf.len();
        let data_type = DataType::from(buf[0]);
        self.log(format!("on network data from [{}]{} bytes,data_type:{:?},step:{:?},http_send_queue_len:{}",self.peer_ip_port(),len,data_type,self.step,self.http_send_queue.len()));
        
        match data_type {
            DataType::Port => self.on_port(&buf[1..]),
            
            DataType::HeartBeat => {
                self.on_recv_heart_beat(&buf[1..]);
                if self.status() == Status::Raw {
                    self.set_status(Status::Establish);
                }
            },

            DataType::Http => {
                return self.on_http_packet(&buf[1..])
            },

            DataType::Ack => self.on_ack(&buf[1..]),

            _ => {
                log::err(format!("[{}]error data_type",self.id()))
            }
        }
        (0,0,DataType::Error)
    }
    
    fn socket_fd(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }
    
}

impl LineUdp2Vps {
    fn on_port(&mut self,buf:&[u8]) {
        let port = u16::from_be_bytes([buf[0],buf[1]]);
        let ip = config::loader::get("vps_ip").unwrap();
        self.log(format!("server tell me port:{}",port));
        self.update_ip_port(format!("{}:{}",ip,port));
        self.send_heart_beat();
    }
}

