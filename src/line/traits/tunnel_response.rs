use std::collections::HashMap;

use crate::global;

use super::network::LineTraitNetWork;

pub trait LineTraitTunnelResponse: LineTraitNetWork {
    //[http_response_packets]
    fn last_normal(&self) -> u64{0}
    fn update_last_normal(&mut self,_new:u64){}
    
    fn tunnel_response_packets(&mut self) -> Option<&mut HashMap<u64,Vec<u8>>>{None}

    fn get_packet(&mut self,id:u64) -> Vec<u8> {
        match self.tunnel_response_packets() {
            Some(m) => {
                match m.get(&id) {
                    Some(v) => {
                        v.clone()
                    },
                    None => Vec::new(),
                }
            },
            None => Vec::new(),
        }
    }

    fn get_packet_id(&self,buf:&[u8]) -> u64 {
        crate::global::u64_from_slice(&buf[0..8])
    }

    fn get_packet_send_time(&self,buf:&[u8]) -> i64 {
        crate::global::i64_from_slice(&buf[8..16])
    }

    fn get_packet_pay_load_len(&self,buf:&[u8]) -> u16 {
        u16::from_be_bytes([buf[24],buf[25]])
    }

    fn on_normal_packet(&mut self,packet_id:u64,buf:&[u8]) {
        let id = self.get_packet_id(buf);
        assert_eq!(packet_id,id);
        self.update_last_normal(id);
        let len = self.get_packet_pay_load_len(buf);
        let hash_server = &buf[16..24];
        let pay_load = &buf[26..];
        assert_eq!(len,pay_load.len() as u16);
        let hash_local = global::hash(pay_load);
        assert_eq!(hash_server,hash_local);
        self.socket_send(pay_load);
    }

    fn save_packet(&mut self,id:u64,buf:&[u8]) {
        let m = self.tunnel_response_packets().unwrap();
        let len = m.len();
        let vec = Vec::from(buf);
        m.insert(id, vec);
        self.log(format!("save_packet:[{}],saving:{}",id,len));
    }

    fn on_tunnel_response_packet(&mut self,buf:&[u8]) {
        let last_normal = self.last_normal();
        let id = self.get_packet_id(buf);
        self.log(format!("on_tunnel_response_packet id:[{}],last_normal:[{}]",id,last_normal));

        if id <= last_normal {
            return;
        }
        
        if id - last_normal == 1 {
            self.on_normal_packet(id,buf);
        } else {
            self.save_packet(id,buf);
        }
    }

    fn check_tunnel_response_packet(&mut self) {
        let id = self.last_normal()+1;
        let v = self.get_packet(id);
        if v.len() > 0 {
            self.log(format!("tunnel tick packet id:[{}]",id));
            self.on_normal_packet(id,&v);
        }
    }
}

/* 

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

fn ack_recive_packet(&mut self,id:u64,peer_t:i64) -> OrderResult {
        self.log(format!("recive_packet:[{}],{}",id,peer_t));

        let mut vec = Vec::new();
        vec.push(DataType::Ack.u8());
        vec.extend(id.to_be_bytes());
        vec.extend(peer_t.to_be_bytes());
        
        let ack_t = global::now_millis();
        vec.extend(ack_t.to_be_bytes());

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

        if vd.contains(&id) {
            return OrderResult::Ignore;
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

pub fn check_udp_packet(&mut self) {
        //let clock = Instant::now();
        let mut data:Vec<(u64,Vec<u8>)> = Vec::new();

        for (_,line) in self.lines.iter_mut() {
            let buf = line.move_out_saving_packet();
            if buf.len() > 0 {
                let pid = line.pair_id();
                if pid > 0 {
                    let node = (pid,buf);
                    data.push(node);
                }
            }
        }
        
        for (id,buf) in data.iter() {
            match self.lines.get_mut(id) {
                Some(line) => {
                    line.on_pair_data(buf,DataType::Http);
                },
                None => {
                    log::err(format!("error can not find line {}",id));
                },
            }
        }
        //let n = clock.elapsed().as_micros();
        //if n > 100 {
            //self.log(format!("check_udp_packet:{}",n));
        //}
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
            //log::err(format!("[{}]panic",self.id()));
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


    fn on_http_packet(&mut self,buf:&[u8]) -> (usize,usize,DataType) {
        let id = global::u64_from_slice(&buf[0..8]);
        let t = global::i64_from_slice(&buf[8..16]);
        let hash = &buf[16..24];
        let pay_load_len = u16::from_be_bytes([buf[24],buf[25]]);
        let stop = 27+pay_load_len as usize;
        
        let ret = self.ack_recive_packet(id,t);

        self.log(format!("on_http_packet id:{},t:{},pay_load_len:{},stop:{},hash:{:?},ret:{:?}",id,t,pay_load_len,stop,hash,ret));
        
        match ret {
            OrderResult::Ignore => (0,0,DataType::Error),
            
            OrderResult::Normal => (27,stop,DataType::Http),
            
            OrderResult::Save => {
                let vec = Vec::from(&buf[26..stop-1]);
                let n = self.new_saving(id,vec);
                self.log(format!("now saving:{}",n));
                (0,0,DataType::Error)
            },
        }
    }


*/