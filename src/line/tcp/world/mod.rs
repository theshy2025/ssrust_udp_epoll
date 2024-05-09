use socket2::Socket;

use crate::{line::base_line::BaseLine, log::log_dir::LogDir};

mod empty_trait_impl;
mod network;

pub struct LineWorld {
    pub basic:BaseLine,
    pub pair_id:u64,
    pub peer_ip_port:String,
    pub socket:Socket,
}

impl LineWorld {
    pub fn new(id:u64,pair_id:u64,socket:Socket) -> LineWorld {
        socket.set_nonblocking(true).unwrap();
        let buf_writer = LineWorld::create_log_buf_writer(id);
        let basic = BaseLine::new(id, buf_writer);
        LineWorld { basic, pair_id, peer_ip_port: String::new(), socket }
    }
}

