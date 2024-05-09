use std::{net::{TcpStream, UdpSocket}, os::fd::AsFd};

use socket2::{Domain, Socket, Type};

use crate::{config::DNS_AGENT, line::{tcp::{pc::LinePc, world::LineWorld}, udp::dns::LineDns}};

use super::Gate;

impl Gate {
    pub fn new_dns_line(&mut self,socket:UdpSocket) {
        self.register_read_event(socket.as_fd(), DNS_AGENT);
        let line = LineDns::new(socket);
        self.lines.insert(DNS_AGENT, Box::new(line));
    }

    pub fn new_pc_line(&mut self,pair_id:u64,socket:TcpStream) -> u64 {
        let id: u64 = crate::global::next_id();
        self.register_read_event(socket.as_fd(), id);
        let line = LinePc::new(id,pair_id,socket);
        self.lines.insert(id, Box::new(line));
        id
    }

    pub fn new_world_line(&mut self,pair_id:u64) -> u64 {
        let id: u64 = crate::global::next_id();
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        self.register_write_event(socket.as_fd(), id);
        let line = LineWorld::new(id,pair_id,socket);
        self.lines.insert(id, Box::new(line));
        id
    }
}

/*

use std::os::fd::AsFd;

use socket2::{Domain, SockAddr, Socket, Type};

use crate::{config::DNS_AGENT, line::{ tcp::{mainland::LineTcp2MainLand, pc::LinePc, vps::LineTcp2Vps, world::LineWorld}, traits::{heart_beat::LineHeartBeat, network::LineNetWork}, udp::{dns::LineDns, mainland::LineUdp2MainLand, vps::LineUdp2Vps}}};

use super::Gate;

impl Gate {
    

    pub fn new_tcp_2_vps_line(&mut self,id:u64) {
        let socket = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
        self.register_write_event(socket.as_fd(), id);
        let line = LineTcp2Vps::new(id,socket);
        self.lines.insert(id, Box::new(line));

        //let line = self.lines.get_mut(&id).unwrap();
        //line.start_connect();
    }

    

    

    

    pub fn new_tcp_mainland_line(&mut self,socket:Socket) -> u64 {
        let id: u64 = crate::global::next_id();
        self.register_read_event(socket.as_fd(), id);
        let mut line = LineTcp2MainLand::new(id,socket);
        line.send_heart_beat();
        self.lines.insert(id, Box::new(line));
        id
    }

    
    
    
}
*/