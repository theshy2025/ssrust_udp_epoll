use std::collections::HashMap;

use socket2::Socket;

use crate::{line::base_line::BaseLine, log::log_dir::LogDir};

mod empty_trait_impl;
mod network;
mod tunnel_response;
mod pair;

pub struct LineWorld {
    pub basic:BaseLine,
    pub pair_id:u64,
    pub peer_ip_port:String,
    pub socket:Socket,
    pub last_normal_tunnel_response_packet_id:u64,
    pub tunnel_response_packets:HashMap<u64,Vec<u8>>,
}

impl LineWorld {
    pub fn new(id:u64,pair_id:u64,socket:Socket) -> LineWorld {
        socket.set_nonblocking(true).unwrap();
        let buf_writer = LineWorld::create_log_buf_writer(id);
        let basic = BaseLine::new(id, buf_writer);
        LineWorld { 
            basic, 
            pair_id, 
            peer_ip_port: String::new(), 
            socket,
            last_normal_tunnel_response_packet_id: 1,
            tunnel_response_packets: HashMap::new(), 
        }
    }
}

