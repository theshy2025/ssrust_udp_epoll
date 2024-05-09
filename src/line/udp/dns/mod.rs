use std::net::UdpSocket;

use crate::{config::DNS_AGENT, line::base_line::BaseLine, log::{log_dir::LogDir, Log}};

mod empty_trait_impl;
mod network;

pub struct LineDns {
    pub basic:BaseLine,
    query_result:Vec<(u64,Option<String>)>,
    socket:UdpSocket,
}

impl LineDns {
    pub fn new(socket:UdpSocket) -> LineDns {
        let buf_writer = LineDns::create_log_buf_writer(DNS_AGENT);
        let basic = BaseLine::new(DNS_AGENT, buf_writer);
        LineDns { basic, socket , query_result: Vec::new()  }
    }
}

impl LineDns {
    pub fn move_out_dns_result(&mut self) -> Option<Vec<(u64,Option<String>)>> {
        if self.query_result.is_empty() {
            return None;
        }
        self.log(format!("move_out_dns_result {:?}",self.query_result));
        let ret = self.query_result.clone();
        self.clear_dns_result();
        Some(ret)
    }

    pub fn clear_dns_result(&mut self) {
        self.log(format!("clear_dns_result {:?}",self.query_result));
        self.query_result.clear();
    }

}