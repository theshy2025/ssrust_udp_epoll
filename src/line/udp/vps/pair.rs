use crate::{line::{line_header::{DataType, Step}, traits::{pair::LineTraitPair, tunnel::LineTraitTunnel}}, log::Log};

use super::LineUdp2Vps;

impl LineTraitPair for LineUdp2Vps {
    fn pair_id(&self) -> u64 {
        self.pair_id
    }
    
    fn set_pair_id(&mut self,id:u64) {
        self.pair_id = id;
    }

    fn on_pair_data(&mut self,buf:&[u8],data_type:DataType) {
        self.log(format!("on_pair_data len:{},data_type:{:?},step:{:?}",buf.len(),data_type,self.step));

        match data_type {
            DataType::Sni => {
                assert!(self.http_send_queue.is_empty());
                self.new_http_packet(buf, DataType::ClientHello);
            },

            DataType::ClientHello => self.extend_sni(buf),
            
            DataType::Http => {
                if self.step == Step::ClientHelloDone {
                    self.step = Step::Http;
                    self.http_send_queue.clear();
                }
                self.send_http_buf(buf);
            },

            _ => todo!()
        }
    }

    
}

impl LineUdp2Vps {
    fn extend_sni(&mut self,buf:&[u8]) {
        assert_eq!(self.http_send_queue.len(),1);
        let (_,sni) = self.http_send_queue.get_mut(&1).unwrap();
        sni.extend(buf);
        self.send_packet(1);
        self.step = Step::ClientHelloDone;
    }
}
