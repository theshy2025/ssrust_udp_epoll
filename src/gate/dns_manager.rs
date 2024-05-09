use std::{net::UdpSocket, time::Instant};

use crate::{config::{self, DNS_AGENT}, line::{tcp::world::LineWorld, udp::dns::LineDns}, log::{log_dir::LogDir, Log}};

use super::Gate;

impl Gate {
    pub fn activate_dns_manager(&mut self) {
        LineDns::create_dir();
        //LineTcp2MainLand::create_dir();
        LineWorld::create_dir();

        let dns_server = config::loader::get("dns_server").unwrap();
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        match socket.connect(&dns_server) {
            Ok(_) => {
                self.log(format!("connect to dns server {:?} success",socket.peer_addr()));
                self.new_dns_line(socket);
            },
            Err(e) => self.log(format!("unbale connect to {},{}",dns_server,e)),
        };
    }

    pub fn gather_dns_query(&mut self) {
        let mut names:Vec<(u64,String)> = Vec::new();

        for (_,line) in self.lines.iter_mut() {
            match line.dns_collect() {
                Some(host) => names.push((line.id(),host)),
                None => {},
            }
        }

        for (id,host) in names {
            let line = self.lines.get_mut(&DNS_AGENT).unwrap();
            line.new_dns_query(id, host);
        }
    }

    pub fn check_dns_result(&mut self) {
        let clock = Instant::now();
        match self.lines.get_mut(&DNS_AGENT) {
            Some(line) => {
                let dns = line.as_any_mut().downcast_mut::<LineDns>().unwrap();
                match dns.move_out_dns_result() {
                    Some(mut data) => {
                        for _ in 0..data.len() {
                            let (id,ip) = data.pop().unwrap();
                            self.on_dns_result(id, ip);
                        }
                    },
                    None => {},
                }
            },
            None => {},
        }
        let n = clock.elapsed().as_micros();
        if n > 100 {
            self.log(format!("check_dns_result:{}",n));
        }
    }

    fn on_dns_result(&mut self,id:u64,ret:Option<String>) {
        let line = self.lines.get_mut(&id).unwrap();
        
        let old_ip_port = line.peer_ip_port().clone();
        let port = old_ip_port.split(":").last().unwrap();
        match ret {
            Some(ip) => {
                let new_ip_port = format!("{}:{}",ip,port);
                let world_id = self.new_world_line(id);
                
                let line = self.lines.get_mut(&id).unwrap();
                line.dns_query_success(world_id);
                
                let line = self.lines.get_mut(&world_id).unwrap();
                
                line.update_ip_port(old_ip_port);
                line.update_ip_port(new_ip_port);
                
                line.start_connect();
            }

            None => line.dns_query_fail(),
        }
    }

}

