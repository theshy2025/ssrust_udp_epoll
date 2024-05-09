use std::any::Any;

use crate::{line::traits::{dns::LineTraitDns, event::LineTraitEvent, heart_beat::LineTraitHeartBeat, pair::LineTraitPair, status::{LineTraitStatus, Status}, tunnel::LineTraitTunnel, Line}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LinePc;

impl Log for LinePc {
    fn logger(&mut self) -> &mut LogBufWriter {
        &mut self.basic.log_buf_writer
    }

    fn id(&self) -> u64 {
        self.basic.id
    }

    fn log(&mut self,s:String) {
        let s = format!("[{}][{:?}]{}",self.pair_id,self.status(),s);
        self.logger().add(s);
        self.logger().flush();
    }
}



impl LineTraitPair for LinePc {
    fn pair_id(&self) -> u64 {
        self.pair_id
    }
    
    fn set_pair_id(&mut self,new:u64) {
        self.pair_id = new;
    }
}

impl LineTraitStatus for LinePc {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}

impl LogDir for LinePc {
    
}

impl LineTraitEvent for LinePc {
    fn on_error(&mut self) {
        self.log(format!("network error {:?}",self.socket.take_error()));
    }
}

impl LineTraitHeartBeat for LinePc {
    
}

impl LineTraitTunnel for LinePc {
    
}

impl LineTraitDns for LinePc {
    
}

impl Line for LinePc {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}