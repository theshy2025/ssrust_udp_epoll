use std::{collections::HashMap, net::UdpSocket};

use crate::{config, line::{base_line::BaseLine, line_enum::Step}, log::log_dir::LogDir};

mod empty_trait_impl;
mod status;
mod network;
mod pair;

pub struct LineUdp2Vps {
    pub pair_id:u64,
    pub basic:BaseLine,
    socket:UdpSocket,
    pub peer_ip_port:String,
    pub step:Step,

    pub last_send_heart_beat:i64,
    pub last_recv_network_data:i64,
    
    pub last_packet_id:u64,
    pub http_send_queue:HashMap<u64,(i64,Vec<u8>)>,
    pub http_recive_map:HashMap<u64,Vec<u8>>,
    pub ids_recive:Vec<u64>,
    
}

impl LineUdp2Vps {
    pub fn new(id:u64,socket:UdpSocket) -> LineUdp2Vps {
        let buf_writer = LineUdp2Vps::create_log_buf_writer(id);
        let basic = BaseLine::new(id,buf_writer);
        let peer_ip_port = format!("{}:{}",config::loader::get("vps_ip").unwrap(),config::loader::get("vps_udp_port").unwrap());
        LineUdp2Vps { 
            basic , 
            socket, 
            peer_ip_port, 
            step: Step::Raw,
            http_recive_map: HashMap::new(),
            pair_id: 0,
            http_send_queue: HashMap::new(),
            last_send_heart_beat: 0,
            last_recv_network_data: 0,
            last_packet_id: 0,
            ids_recive: Vec::new(), 
        }
    }
}
/* 

    pub missing_report:HashMap<u16,i64>,
*/