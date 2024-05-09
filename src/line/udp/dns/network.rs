use std::os::fd::{AsFd, BorrowedFd};

use simple_dns::*;

use crate::{line::{line_enum::DataType, traits::{dns::LineTraitDns, network::LineTraitNetWork}}, log::Log};

use super::LineDns;

impl LineTraitDns for LineDns {
    fn new_dns_query(&mut self,id:u64,host:String) {
        self.log(format!("new_dns_query line_id:{},host:{:?}",id,host));
        let packet = build(id.try_into().unwrap(),host);
        self.socket_send(&packet);
    }
}

impl LineTraitNetWork for LineDns {
    fn peer_ip_port(&self) -> String {
        self.socket.peer_addr().unwrap().to_string()
    }

    fn on_network_data(&mut self,buf:&mut [u8]) -> (usize,usize,DataType) {
        self.log(format!("on_data_from_dns_server {} bytes",buf.len()));
        let ret = self.decode(buf);
        self.log(format!("decode result {:?}",ret));
        self.query_result.push(ret);
        (0,0,DataType::Error)
    }
    
    fn socket_read(&mut self,buf:&mut [u8]) -> std::io::Result<usize> {
        self.socket.recv(buf)
    }

    fn socket_send(&mut self,buf:&[u8]) {
        self.log(format!("udp send {} bytes to {:?} ",buf.len(),self.peer_ip_port()));
        self.socket.send_to(buf,self.peer_ip_port()).unwrap();
    }
    
    fn socket_fd(&self) -> BorrowedFd<'_> {
        self.socket.as_fd()
    }
    
}

impl LineDns {

    fn decode(&mut self,buf:&[u8]) -> (u64,Option<String>) {
        match Packet::parse(buf) {
            Ok(packet) => {
                let id = packet.id() as u64;
                match packet.rcode() {
                    RCODE::NoError => {
                        let ip = get_ip(packet.answers);
                        (id,ip)
                    },
                    other => {
                        self.log(format!("dns server reply with error code {:?}",other));
                        (id,None)
                    },
                }
            },
            Err(e) => {
                self.log(format!("packet parse fail {},{}",e,buf.len()));
                (0,None)
            },
        }
    }
    
}


fn get_ip(data:Vec<ResourceRecord>) -> Option<String> {
    for v in data {
        match v.rdata {
            rdata::RData::A(a) => {
                let b = a.address.to_be_bytes();
                let ret = format!("{}.{}.{}.{}",b[0],b[1],b[2],b[3]);
                return Some(ret)
            }
            _ => {}
        }
    }

    None
}

fn build(id:u16,host:String) -> Vec<u8> {
    let mut packet = Packet::new_query(id);
    packet.set_flags(PacketFlag::RECURSION_DESIRED);
    let qname = Name::new(&host).unwrap();
    let qtype = TYPE::A.into();
    let qclass = CLASS::IN.into();
    let question = Question::new(qname, qtype, qclass, false);
    packet.questions.push(question);
    packet.build_bytes_vec_compressed().unwrap()
}