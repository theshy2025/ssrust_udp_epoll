use std::{collections::HashMap, net::{TcpListener, UdpSocket}, os::fd::AsFd};

use nix::sys::epoll::{Epoll, EpollCreateFlags};

use crate::{config::{self, TCP_GATE_ID}, global, line::traits::Line, log::{buf_writer::LogBufWriter, Log}};

mod epoll;
mod event;
mod line_creater;
mod line_manager;
mod udp_manager;
mod tcp_manager;
mod dns_manager;


pub struct Gate {
    tcp_gate:TcpListener,
    epoll:Epoll,
    lines:HashMap<u64,Box<dyn Line>>,
    buf_writer:LogBufWriter,
    udp_gate:Option<UdpSocket>,
}

impl Gate {
    pub fn new() -> Gate {
        let port = config::loader::get("tcp_port").unwrap();
        let tcp_gate = TcpListener::bind(format!("0.0.0.0:{}",port)).unwrap();
        
        let epoll = Epoll::new(EpollCreateFlags::empty()).unwrap();
        let dir = crate::log::device_log_path();
        let path = format!("{}/{}.log",dir,module_path!().split("::").last().unwrap());
        let buf_writer = LogBufWriter::new(path).unwrap();

        Gate{ tcp_gate , epoll , lines:HashMap::new(), buf_writer, udp_gate: None }
    }
}

impl Gate {
    pub fn start(&mut self) {
        self.init();
        loop {
            global::next_frame();
            self.tick();
            self.poll();
            self.check_dns_result();
            self.gather_dns_query();
            self.gather_client_hello();
        }
    }

    fn init(&mut self) {
        
        self.start_tcp_gate();

        match config::loader::get("line_num") {
            Some(n) => {
                let n:u8 = n.parse().unwrap();
                //self.create_tcp_2_vps_lines(n);
                self.create_udp_2_vps_lines(n);
            }

            None => {
                self.activate_dns_manager();
                self.start_udp_gate();
            },
        }
    }

    fn start_tcp_gate(&mut self) {
        self.tcp_gate.set_nonblocking(true).unwrap();
        self.register_read_event(self.tcp_gate.as_fd(), TCP_GATE_ID);
        self.log(format!("tcp gate socket listening on {:?}",self.tcp_gate.local_addr()));
    }
}

impl Log for Gate {
    fn logger(&mut self) -> &mut LogBufWriter {
       &mut self.buf_writer
    }
}