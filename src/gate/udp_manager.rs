use std::{net::UdpSocket, os::fd::AsFd};

use crate::{config::{self, UDP_GATE_ID}, line::{line_header::DataType, tcp::pc::LinePc, udp::{mainland::LineUdp2MainLand, vps::LineUdp2Vps}}, log::{log_dir::LogDir, Log}};

use super::Gate;

impl Gate {
    pub fn start_udp_gate(&mut self) {
        LineUdp2MainLand::create_dir();
        
        let port = config::loader::get("udp_port").unwrap();
        let socket = UdpSocket::bind(format!("0.0.0.0:{}",port)).unwrap();
        self.register_read_event(socket.as_fd(), UDP_GATE_ID);
        self.log(format!("udp gate socket listen on {:?}",socket.local_addr()));
        self.udp_gate = Some(socket);
    }

    pub fn create_udp_2_vps_lines(&mut self,n:u8) { 
        LinePc::create_dir();
        LineUdp2Vps::create_dir();
        for _ in 0..n {
            self.new_udp_2_vps_line();
        }
    }

    pub fn new_udp_2_vps_line(&mut self) {
        let id = crate::global::next_id();
        let socket = UdpSocket::bind(format!("0.0.0.0:0")).unwrap();
        self.register_read_event(socket.as_fd(), id);
        let line = LineUdp2Vps::new(id,socket);
        self.lines.insert(id, Box::new(line));

        let line = self.lines.get_mut(&id).unwrap();
        line.socket_send(&[0]);
    }

    pub fn accept_udp_connect(&mut self) {
        match &self.udp_gate {
            Some(g) => {
                let mut buf = [0u8;1024];
                let (_n,addr) = g.recv_from(&mut buf).unwrap();
                let source = format!("{}:{}",addr.ip(),addr.port());
                //self.log(format!("recv {} bytes from {}",n,source));
                self.new_udp_mainland_line(source);
            },
            None => todo!(),
        }
    }

    pub fn new_udp_mainland_line(&mut self,peer_ip_port:String) {
        let id = crate::global::next_id();
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let port = socket.local_addr().unwrap().port();
        self.register_read_event(socket.as_fd(), id);

        let mut buf = Vec::new();
        buf.push(DataType::Port.u8());
        buf.extend(port.to_be_bytes());
        self.udp_gate.as_mut().unwrap().send_to(&buf, &peer_ip_port).unwrap();

        let line = LineUdp2MainLand::new(id,socket,peer_ip_port);
        self.lines.insert(id, Box::new(line));
        
    }

    pub fn find_idle_udp_2_vps_line(&self,count:bool) -> (u64,usize) {
        let mut id = 0;
        let mut idle = 0usize;
        for (_,line) in self.lines.iter() {
            match line.as_any().downcast_ref::<LineUdp2Vps>() {
                Some(uv) => {
                    if uv.is_ready() {
                        if count {
                            idle = idle + 1;
                            id = line.id();
                        } else {
                            return (line.id(),0);
                        }
                    }
                },
                None => {},
            }
        }

        (id,idle)
    }

}