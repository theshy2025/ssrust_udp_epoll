use std::{collections::HashMap, net::UdpSocket, time::Instant};

use crate::{line::{base_line::BaseLine, line_header::Step}, log::log_dir::LogDir};

mod empty_trait_impl;
mod status;
mod network;
mod dns;
mod pair;

pub struct LineUdp2MainLand {
    pub clock:Instant,
    pub pair_id:u64,
    pub basic:BaseLine,
    socket:UdpSocket,
    pub peer_ip_port:String,
    pub step:Step,

    pub last_packet_id:u64,
    pub client_hello_data:Vec<u8>,
    pub http_send_queue:HashMap<u64,(i64,Vec<u8>)>,
    pub http_recive_map:HashMap<u64,Vec<u8>>,
    pub ids_recive:Vec<u64>,
    
}

impl LineUdp2MainLand {
    pub fn new(id:u64,socket:UdpSocket,peer_ip_port:String) -> LineUdp2MainLand {
        socket.connect(&peer_ip_port).unwrap();
        let buf_writer = LineUdp2MainLand::create_log_buf_writer(id);
        let basic = BaseLine::new(id,buf_writer);
        
        LineUdp2MainLand {
            clock:Instant::now(), 
            pair_id:0,
            basic , 
            socket, 
            peer_ip_port,
            http_recive_map: HashMap::new(),
            step: Step::Raw,
            client_hello_data: Vec::new(),
            http_send_queue: HashMap::new(),
            last_packet_id: 0,
            ids_recive: Vec::new(),
        }
    }
}