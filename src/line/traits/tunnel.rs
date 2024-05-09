use std::collections::HashMap;

use crate::{config::BUFF_SIZE, global, line::line_enum::{DataType, OrderResult}, log};

use super::network::LineTraitNetWork;

pub trait LineTraitTunnel: LineTraitNetWork {
    fn last_packet_id(&self) -> u64{0}
    fn update_last_packet_id(&mut self,_new_id:u64){}
    //[ids_recive]
    fn ids_recive(&mut self) -> Option<&mut Vec<u64>>{None}

    fn pop_ids_recive(&mut self,id:u64) {
        match self.ids_recive() {
            Some(vd) => {
                vd.remove(0);
                let head = vd.get(0).unwrap().clone();
                if head != id {
                    vd.push(id);
                }
            },
            None => {},
        }
    }
        
    fn ack_recive_packet(&mut self,id:u64,t:i64) -> OrderResult {
        self.log(format!("recive_packet:{},{}",id,t));

        let mut vec = Vec::new();
        vec.push(DataType::Ack.u8());
        vec.extend(id.to_be_bytes());
        vec.extend(t.to_be_bytes());
        self.socket_send(&vec);


        let vd = self.ids_recive().unwrap();

        if vd.is_empty() {
            vd.push(id);
            if id == 1 {
                return OrderResult::Normal;
            } else {
                vd.push(0);
                vd.sort();
                return OrderResult::Save;
            }
        }

        let head = vd.get(0).unwrap().clone();
        if id <= head {
            return OrderResult::Ignore;
        }

        if id - head == 1 {
            vd.remove(0);
            vd.push(id);
            vd.sort();
            return OrderResult::Normal;
        } else {
            vd.push(id);
            vd.sort();
            return OrderResult::Save;
        }
    }

    fn on_http_packet(&mut self,buf:&[u8]) -> (usize,usize,DataType) {
        let id = global::u64_from_slice(&buf[0..8]);
        let t = global::i64_from_slice(&buf[8..16]);
        let pay_load_len = u16::from_be_bytes([buf[16],buf[17]]);
        let stop = 19+pay_load_len as usize;
        
        let ret = self.ack_recive_packet(id,t);

        self.log(format!("on_http_packet id:{},t:{},pay_load_len:{},stop:{},ret:{:?},",id,t,pay_load_len,stop,ret));
        
        match ret {
            OrderResult::Ignore => (0,0,DataType::Error),
            
            OrderResult::Normal => (19,stop,DataType::Http),
            
            OrderResult::Save => {
                let vec = Vec::from(&buf[18..stop-1]);
                let n = self.new_saving(id,vec);
                self.log(format!("now saving:{}",n));
                (0,0,DataType::Error)
            },
        }
    }

    //[http_send_queue]
    fn http_send_queue(&mut self) -> Option<&mut HashMap<u64,(i64,Vec<u8>)>>{None}

    fn on_ack(&mut self,buf:&[u8]) {
        let id = global::u64_from_slice(&buf[0..8]);
        let t = global::i64_from_slice(&buf[8..]);
        let gap = global::now_millis() - t;
        self.log(format!("on_ack packet:[{}],{}",id,gap));
        let m = self.http_send_queue().unwrap();
        m.remove(&id);
    }

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

        for (id,gap) in miss {
            self.log(format!("id:{},gap:{}",id,gap));
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

        let id = self.last_packet_id()+1;
        let t = global::now_millis();
        let mut vec = Vec::new();
        vec.push(data_type.u8());
        vec.extend(id.to_be_bytes());
        vec.extend(t.to_be_bytes());
        vec.extend(len.to_be_bytes());
        vec.extend(buf);

        let node = (t,vec);
        
        let m = self.http_send_queue().unwrap();
        m.insert(id, node);

        self.update_last_packet_id(id);
        id
    }

    fn send_packet(&mut self,id:u64) {
        self.log(format!("send_packet:[{}]",id));
        match self.http_send_queue() {
            Some(m) => {
                let (_,buf) = m.get(&id).unwrap();
                let buf = buf.clone();
                self.socket_send(&buf);
            },
            None => todo!(),
        }
    }

    //[http_recive_map]
    fn http_recive_map(&mut self) -> Option<&mut HashMap<u64,Vec<u8>>>{None}

    fn new_saving(&mut self,packet_id:u64,data:Vec<u8>) -> usize {
        match self.http_recive_map() {
            Some(m) => {
                m.insert(packet_id, data);
                m.len()
            },
            None => 0,
        }
    }

    fn move_out_saving_packet(&mut self) -> Vec<u8> {
        let mut ret = Vec::new();
        
        for _ in 0..50 {
            let (id,buf) = self.move_out_saving_once();
            if id == 0 {
                return ret;
            }
            ret.extend(buf.unwrap());
            self.pop_ids_recive(id);
        }

        ret
    }

    fn move_out_saving_once(&mut self) -> (u64,Option<Vec<u8>>) {
        let mut vec = Vec::new();
        match self.ids_recive() {
            Some(vd) => {
                if vd.is_empty() {
                    return (0,None);
                }
                vec.extend(vd.clone());
            },
            None => return (0,None),
        };
        
        self.log(format!("move_out_saving_once {:?}",vec));
        if vec.len() > 13 {
            log::err(format!("[{}]panic",self.id()));
            //panic!()
        }
        
        let head = vec.get(0).unwrap().clone();

        let id = head+1;

        match self.http_recive_map() {
            Some(m) => {
                match m.get(&id) {
                    Some(v) => {
                        (id,Some(v.clone()))
                    },
                    None => (0,None),
                }
            },
            None => (0,None),
        }
        
    }

    

    


}


/*

self.missing_check();

fn missing_check(&mut self) {
        match self.ids_recive() {
            Some(vd) => {
                if vd.len() > 2 {
                    let head = vd.get(0).unwrap().clone();
                    let next = vd.get(1).unwrap().clone();
                    let tail = vd.get(2).unwrap().clone();
                    if next > head && next - head > 1 {
                        self.req_resend_missing(head,next,tail);
                    }
                }
            },
            None => {},
        }
    }

    //[missing_report]
    fn missing_report(&mut self) -> Option<&mut HashMap<u16,i64>>{None}


fn req_resend_missing(&mut self,start:u16,mid:u16,stop:u16) {
        match self.missing_report() {
            Some(m) => {
                let now = crate::global::now_millis();
                match m.get(&start) {
                    Some(t) => {
                        let t = t.clone();
                        if now > t && now - t > 100 {
                            m.remove(&start);
                        }
                    },
                    None => {
                        m.insert(start,now);

                        
                    },
                }
            },
            None => {},
        }
    }*/