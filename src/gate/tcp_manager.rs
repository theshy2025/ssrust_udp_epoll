use std::net::TcpStream;

use crate::{config, log::Log};

use super::Gate;

impl Gate {
    pub fn accept_tcp_connect(&mut self) {
        match self.tcp_gate.accept() {
            Ok((socket,_)) => self.on_stream(socket),
            Err(e) => {
                self.log(format!("{}",e));
            },
        }
    }

    pub fn on_stream(&mut self,socket:TcpStream) {
        match config::loader::get("line_num") {
            Some(_) => {
                self.find_chick_for_pc(socket);
            },
            None => {
                //self.new_tcp_mainland_line(socket);
            },
        }
    }

    fn find_chick_for_pc(&mut self,socket:TcpStream) {
        let (tid,idle) = self.find_idle_udp_2_vps_line(true);
        self.log(format!("find_chick_for_pc tid:{},idle:{}",tid,idle));
        if tid > 0 {
            let id = self.new_pc_line(tid,socket);
            let line = self.lines.get_mut(&tid).unwrap();
            line.set_pair_id(id);
        } else {
            self.err(format!("run out of chick"));
        }
    }
}