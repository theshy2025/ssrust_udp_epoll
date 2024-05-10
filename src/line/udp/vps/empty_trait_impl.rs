use std::{any::Any, collections::HashMap};

use crate::{global, line::traits::{dns::LineTraitDns, event::LineTraitEvent, heart_beat::LineTraitHeartBeat, tunnel_response::LineTraitTunnelResponse, pair::LineTraitPair, status::LineTraitStatus, tunnel::LineTraitTunnel, Line}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LineUdp2Vps;

impl Line for LineUdp2Vps {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl LineTraitHeartBeat for LineUdp2Vps {
    fn last_send_heart_beat(&self) -> i64 {
        self.last_send_heart_beat
    }

    fn update_last_send_heart_beat(&mut self) {
        self.last_send_heart_beat = global::now();
    }
}

impl LineTraitEvent for LineUdp2Vps {

    
}

impl Log for LineUdp2Vps {
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

impl LogDir for LineUdp2Vps {
    
}

impl LineTraitTunnel for LineUdp2Vps {
    fn last_packet_id(&self) -> u64 {
        self.last_packet_id
    }
    
    fn update_last_packet_id(&mut self,new_id:u64) {
        self.last_packet_id = new_id;
    }

    fn http_send_queue(&mut self) -> Option<&mut HashMap<u64,(i64,Vec<u8>)>> {
        Some(&mut self.http_send_queue)
    }
}

impl LineTraitDns for LineUdp2Vps {

    
}

impl LineTraitTunnelResponse for LineUdp2Vps {

    
}