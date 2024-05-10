use std::os::fd::{AsFd, BorrowedFd};

use crate::{line::{line_header::{DataType, Step}, traits::{heart_beat::LineTraitHeartBeat, tunnel_response::LineTraitTunnelResponse, network::LineTraitNetWork, tunnel::LineTraitTunnel}}, log::Log};

use super::LineUdp2MainLand;


impl LineTraitNetWork for LineUdp2MainLand {
    fn socket_peer_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.socket.peer_addr()//self.peer_ip_port.clone()
    }

    fn peer_ip_port(&self) -> String {
        self.peer_ip_port.clone()
    }

    fn update_ip_port(&mut self,ip_port:String) {
        self.peer_ip_port = ip_port;
        self.log(format!("update_ip_port to {}",self.peer_ip_port));
    }
    
    fn socket_send(&mut self,buf:&[u8]) {
        let _addr = self.socket.peer_addr();
        //self.log(format!("try send {} bytes to[{:?}] ",buf.len(),addr));
        
        match self.socket.send(buf) {
            Ok(_n) => {}//self.log(format!("send {} bytes to[{:?}]done",n,addr)),
            Err(e) => self.log(format!("socket send fail {}",e)),
        }
    }
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize> {
        self.socket.recv(buf)
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        let len = buf.len();
        let data_type:DataType = DataType::from(buf[0]);
        self.log(format!("on network data from[{:?}]{} bytes,data_type:{:?},step:{:?},http_send_queue_len:{}",self.socket.peer_addr(),len,data_type,self.step,self.http_send_queue.len()));
        
        if data_type == DataType::HeartBeat {
            self.on_recv_heart_beat(&buf[1..]);
            self.send_heart_beat();
            return (0,0,DataType::Error);
        }

        if data_type == DataType::Ack {
            self.on_recive_ack(&buf[1..]);
            return (0,0,DataType::Error);
        }

        let id = self.get_packet_id(&buf[1..]);
        let packet_send_time = self.get_packet_send_time(&buf[1..]);
        self.send_ack(id,packet_send_time);

        match data_type {
            DataType::ClientHello => {
                if self.step == Step::Raw {
                    self.on_recv_client_hello(&mut buf[1..]);
                }
            },
            
            DataType::Http => { 
                return (0,buf.len(),DataType::Http)
            },

            _ => todo!()
        }
        (0,0,DataType::Error)
    }
    
    fn socket_fd(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }
    
}


impl LineUdp2MainLand {
    fn on_recv_client_hello(&mut self,buf:&mut [u8]) {
        let sni_len = u16::from_be_bytes([buf[24],buf[25]]);
        let stop = 26+sni_len as usize;
        let sni_buf = &mut buf[26..stop];
        
        crate::global::reverse(sni_buf);
        let ret = crate::global::decode_host_name(sni_buf);
        
        self.update_ip_port(ret);

        for i in stop..buf.len() {
            self.client_hello_data.push(crate::global::u8r(buf[i]));
        }

        self.step = Step::WaitingDnsCollect;

        self.log(format!("on_recv_client_hello {} bytes,sni_len:{},ip_port:{}",buf.len(),sni_len,self.peer_ip_port));
        
    }

    
    
}

