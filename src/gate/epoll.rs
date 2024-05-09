use std::os::fd::BorrowedFd;

use nix::sys::epoll::{EpollEvent,EpollFlags,EpollTimeout};

use crate::log;

use super::Gate;

impl Gate {
    pub fn poll(&mut self) {
        let raw = EpollEvent::empty();
        let mut events = [raw;32];
        let timeout = EpollTimeout::from(20u16);
        self.epoll.wait(&mut events, timeout).unwrap();

        for v in events {
            let id = v.data();
            if id > 0 {
                self.on_epoll_event(id,v);
            }
        }
    }

    fn on_epoll_event(&mut self,id:u64,evt:EpollEvent) {
        for flags in evt.events() {
            //self.log(format!("id:{},flags:{:?}",id,flags));
            match flags {
                EpollFlags::EPOLLIN => self.epoll_in(id),
                EpollFlags::EPOLLOUT => self.on_write_able_event(id),
                EpollFlags::EPOLLRDHUP => self.on_rd_hang_up_event(id),
                EpollFlags::EPOLLERR => self.epoll_err(id),
                EpollFlags::EPOLLHUP => self.on_hang_up_event(id),
                other => self.on_other_event(id,other),
            }
        }
    }
    

    pub fn register_read_event(&self,fd:BorrowedFd<'_>,id:u64) {
        let mut flags = EpollFlags::empty();
        flags.insert(EpollFlags::EPOLLIN);
        flags.insert(EpollFlags::EPOLLRDHUP);
        self.add_fd(fd, id, flags);
    }

    pub fn register_write_event(&self,fd:BorrowedFd<'_>,id:u64) {
        let mut flags = EpollFlags::empty();
        flags.insert(EpollFlags::EPOLLOUT);
        self.add_fd(fd, id, flags);
    }

    pub fn add_fd(&self,fd:BorrowedFd<'_>,id:u64,flags:EpollFlags) {
        let event = EpollEvent::new(flags,id);
        
        match self.epoll.add(fd, event) {
            Ok(_) => {
                //let str = flags_str_name(flags);
                //log::err(format!("id:{} add_fd {:?} success",id,fd));
            },

            Err(e) => log::err(format!("[{}]add_fd {:?} fail {}",id,fd,e)),
        }
    }

    pub fn remove_fd(&self,fd:BorrowedFd<'_>) {
        match self.epoll.delete(fd) {
            Ok(_) => {
                //log::err(format!("remove_fd {:?} success",fd));
            },
            
            Err(e) => log::err(format!("remove_fd {:?} fail {}",fd,e)),
        }
    }
}