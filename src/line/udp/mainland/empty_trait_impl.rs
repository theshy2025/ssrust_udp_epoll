use std::{any::Any, collections::HashMap};

use crate::{line::traits::{event::LineTraitEvent, heart_beat::LineTraitHeartBeat, pair::LineTraitPair, status::LineTraitStatus, tunnel::LineTraitTunnel, Line}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LineUdp2MainLand;

impl LogDir for LineUdp2MainLand {
    
}

impl Log for LineUdp2MainLand {
    fn id(&self) -> u64 {
        self.basic.id
    }
    
    fn logger(&mut self) -> &mut LogBufWriter {
        &mut self.basic.log_buf_writer
    }

    fn log(&mut self,s:String) {
        let s = format!("[{}][{:?}]{}",self.pair_id(),self.status(),s);
        self.logger().add(s);
        self.logger().flush();
    }
}

impl Line for LineUdp2MainLand {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl LineTraitEvent for LineUdp2MainLand {
    fn on_error(&mut self) {
        self.log(format!("on_error {:?}",self.socket.take_error()));
    }
}

impl LineTraitHeartBeat for LineUdp2MainLand {
    
}

impl LineTraitTunnel for LineUdp2MainLand {
    fn last_packet_id(&self) -> u64 {
        self.last_packet_id
    }
    
    fn update_last_packet_id(&mut self,new_id:u64) {
        self.last_packet_id = new_id;
    }

    fn http_send_queue(&mut self) -> Option<&mut HashMap<u64,(i64,Vec<u8>)>> {
        Some(&mut self.http_send_queue)
    }

    fn http_recive_map(&mut self) -> Option<&mut HashMap<u64,Vec<u8>>> {
        Some(&mut self.http_recive_map)
    }

    fn ids_recive(&mut self) -> Option<&mut Vec<u64>> {
        Some(&mut self.ids_recive)
    }
}
