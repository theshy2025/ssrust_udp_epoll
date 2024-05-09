use std::net::TcpStream;

use crate::{line::base_line::BaseLine, log::log_dir::LogDir};

use self::network::Step;

mod empty_trait_impl;
mod network;

pub struct LinePc {
    pub basic:BaseLine,
    pub pair_id:u64,
    pub step:Step,
    pub socket:TcpStream,
}

impl LinePc {
    pub fn new(id:u64,pair_id:u64,socket:TcpStream) -> LinePc {
        let buf_writer = LinePc::create_log_buf_writer(id);
        let basic = BaseLine::new(id, buf_writer);
        LinePc { 
            basic, 
            pair_id, 
            step: Step::Raw,
            socket, 
        }
    }
}