use nix::sys::epoll::EpollFlags;

use crate::{config::{BUFF_SIZE, TCP_GATE_ID,UDP_GATE_ID}, log::Log};

use super::Gate;

impl Gate {
    pub fn epoll_in(&mut self,id:u64) {
        match id {
            TCP_GATE_ID => self.accept_tcp_connect(),
            UDP_GATE_ID => self.accept_udp_connect(),
            other => self.on_read_able_event(other),
        }
    }

    pub fn on_write_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_write_able();
    }

    pub fn on_read_able_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        let mut buf = [0;BUFF_SIZE];
        let (start,stop,data_tag) = line.on_read_able(&mut buf);
        if stop > 0 {
            let pid = line.pair_id();
            if pid > 0 {
                let line = self.lines.get_mut(&pid).unwrap();
                line.on_pair_data(&buf[start..stop],data_tag);
            }
        }
    }

    pub fn on_rd_hang_up_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_rd_hang_up();
    }

    pub fn epoll_err(&mut self,id:u64) {
        match id {
            TCP_GATE_ID => self.log(format!("gate error")),
            other => {
                let line = self.lines.get_mut(&other).unwrap();
                line.on_error();
            }
        }
    }

    pub fn on_hang_up_event(&mut self,id:u64) {
        let line = self.lines.get_mut(&id).unwrap();
        line.on_hang_up();
    }

    pub fn on_other_event(&mut self,id:u64,flags:EpollFlags) {
        self.log(format!("[{}]on_other_event {:?}",id,flags));
    }
}
