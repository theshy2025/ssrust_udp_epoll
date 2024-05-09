use std::os::fd::{AsFd, BorrowedFd};

use crate::{global, line::{line_enum::{DataType, OrderResult, Step}, traits::{heart_beat::LineTraitHeartBeat, network::LineTraitNetWork, tunnel::LineTraitTunnel}}, log::Log};

use super::LineUdp2MainLand;


impl LineTraitNetWork for LineUdp2MainLand {
    fn peer_ip_port(&self) -> String {
        self.peer_ip_port.clone()
    }

    fn update_ip_port(&mut self,ip_port:String) {
        self.peer_ip_port = ip_port;
        self.log(format!("update_ip_port to {}",self.peer_ip_port));
    }
    
    fn socket_send(&mut self,buf:&[u8]) {
        let addr = self.socket.peer_addr().unwrap().to_string();
        self.log(format!("udp send {} bytes to {} ",buf.len(),addr));
        
        self.socket.send(buf).expect(&self.id().to_string());
    }
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize> {
        self.socket.recv(buf)
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        let len = buf.len();
        let data_type:DataType = DataType::from(buf[0]);
        let addr = self.socket.peer_addr().unwrap().to_string();
        self.log(format!("on network data from[{}]{} bytes,data_type:{:?},step:{:?},http_send_queue_len:{}",addr,len,data_type,self.step,self.http_send_queue.len()));
        match data_type {
            DataType::HeartBeat => {
                self.on_recv_heart_beat(&buf[1..]);
                self.send_heart_beat();
            },
            
            DataType::ClientHello => self.on_recv_client_hello(&mut buf[1..]),
            
            DataType::Http => return self.on_http_packet(&buf[1..]),

            DataType::Ack => self.on_ack(&buf[1..]),

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
        assert!(self.client_hello_data.is_empty());
        let packet_id = global::u64_from_slice(&buf[0..8]);
        let _t = global::i64_from_slice(&buf[8..16]);
        let ret = self.ack_recive_packet(packet_id,0);
        
        if ret != OrderResult::Normal {
            return;
        }

        let sni_len = u16::from_be_bytes([buf[16],buf[17]]);
        let stop = 18+sni_len as usize;
        let sni_buf = &mut buf[18..stop];
        
        crate::global::reverse(sni_buf);
        let ret = crate::global::decode_host_name(sni_buf);
        
        self.update_ip_port(ret);

        for i in stop..buf.len() {
            self.client_hello_data.push(crate::global::u8r(buf[i]));
        }

        self.step = Step::WaitingDnsCollect;

        self.log(format!("on_recv_client_hello {} bytes,packet_id:{},sni_len:{},ip_port:{}",buf.len(),packet_id,sni_len,self.peer_ip_port));
        
    }

    
    
}

