use crate::{global::frame, line::line_header::DataType};

use super::network::LineTraitNetWork;

pub trait LineTraitHeartBeat: LineTraitNetWork {
    fn last_send_heart_beat(&self) -> i64{0}
    fn last_recv_network_data(&self) -> i64{0}
    fn update_last_send_heart_beat(&mut self){}

    
    fn on_recv_heart_beat(&mut self,buf:&[u8]) {
        let id = self.decode_heart_beat(buf);
        if frame() < 1000 {
            self.log(format!("on_recv_heart_beat network peer id[{}]",id));
        }
        
    }

    fn send_heart_beat(&mut self) {
        if crate::global::now() - self.last_send_heart_beat() < 10 {
            return;
        }

        if crate::global::now() - self.last_recv_network_data() < 10 {
            return;
        }
        
        self.update_last_send_heart_beat();

        let mut buf = Vec::new();
        buf.push(DataType::HeartBeat.u8());
        buf.extend(self.id().to_be_bytes());
        self.socket_send(&buf);

    }

    fn decode_heart_beat(&self,buf:&[u8]) -> u64 {
        match buf.try_into() {
            Ok(arr) => {
                let id = u64::from_be_bytes(arr);
                id
            },
            Err(e) => {
                crate::log::err(format!("{}",e));
                0
            },
        }
    }
}