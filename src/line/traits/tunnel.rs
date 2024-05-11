use std::collections::HashMap;

use crate::{config::BUFF_SIZE, global, line::line_header::DataType};

use super::tunnel_response::LineTraitTunnelResponse;

pub trait LineTraitTunnel: LineTraitTunnelResponse {
    fn last_packet_id(&self) -> u64{0}
    fn update_last_packet_id(&mut self,_new_id:u64){}
        
    fn send_ack(&mut self,id:u64,packet_send_time:i64) {
        self.log(format!("send_ack id:[{}],{}",id,packet_send_time));

        let mut vec = Vec::new();
        vec.push(DataType::Ack.u8());
        vec.extend(id.to_be_bytes());
        vec.extend(packet_send_time.to_be_bytes());
        let ack_time = global::now_millis();
        vec.extend(ack_time.to_be_bytes());

        self.socket_send(&vec);
    }

    fn on_recive_ack(&mut self,buf:&[u8]) {
        let id = self.get_packet_id(buf);
        let send_time = self.get_packet_send_time(buf);
        let ack_time = global::i64_from_slice(&buf[16..]);
        let gap = global::now_millis() - send_time;
        self.log(format!("recive_ack id:[{}],{},{},{}",id,gap,send_time,ack_time));
        let m = self.http_send_queue().unwrap();
        m.remove(&id);
    }

    //[http_send_queue]
    fn http_send_queue(&mut self) -> Option<&mut HashMap<u64,(i64,Vec<u8>)>>{None}

    

    fn resend_timeout_packet(&mut self) {
        let mut miss = Vec::new();

        match self.http_send_queue() {
            Some(m) => {
                if m.len() > 0 {
                    let now = global::now_millis();
                    for (id,(t,_)) in m.iter_mut() {
                        let gap = now - *t;
                        if gap > 100 { 
                            miss.push((id.clone(),gap));
                            *t = now;
                            
                        }
                    }
                }
            },
            None => {},
        }

        for (id,_gap) in miss {
            //self.log(format!("id:{},gap:{}",id,gap));
            self.send_packet(id);
        }
    }

    fn send_http_buf(&mut self,buf:&[u8]) {
        let data_len = buf.len();
        for i in 0..BUFF_SIZE/1024 {
            let start = i*1024;
            let stop = (start + 1024).min(data_len);
            let id = self.new_http_packet(&buf[start..stop],DataType::Http);
            self.send_packet(id);
            if stop == data_len {
                break;
            }
        }
    }

    fn new_http_packet(&mut self,buf:&[u8],data_type:DataType) -> u64 {
        let len = buf.len() as u16;
        assert!(len <= 1024);

        //println!("Hash is [{:x}],{},{:?}", hasher.finish(),buf.len(),bytes);

        let id = self.last_packet_id()+1;
        let t = global::now_millis();
        let mut vec = Vec::new();
        vec.push(data_type.u8());
        vec.extend(id.to_be_bytes());
        vec.extend(t.to_be_bytes());
        vec.extend(global::hash(buf));
        vec.extend(len.to_be_bytes());
        vec.extend(buf);

        let node = (t,vec);
        
        let m = self.http_send_queue().unwrap();
        m.insert(id, node);

        self.update_last_packet_id(id);
        id
    }

    fn send_packet(&mut self,id:u64) {
        
        match self.http_send_queue() {
            Some(m) => {
                let (_,buf) = m.get(&id).unwrap();
                let buf = buf.clone();
                let hash = &buf[17..25];
                self.log(format!("send_packet id:[{}],hash:{:?}",id,hash));
                self.socket_send(&buf);
            },
            None => todo!(),
        }
    }

}
