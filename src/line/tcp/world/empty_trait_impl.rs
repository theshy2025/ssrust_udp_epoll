use std::any::Any;

use crate::{line::traits::{dns::LineTraitDns, event::LineTraitEvent, heart_beat::LineTraitHeartBeat, status::{LineTraitStatus, Status}, tunnel::LineTraitTunnel, Line}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LineWorld;

impl Log for LineWorld {
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

impl LineTraitStatus for LineWorld {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}

impl LineTraitEvent for LineWorld {
    fn on_error(&mut self) {
        self.log(format!("network error {:?}",self.socket.take_error()));
    }
}


impl LineTraitHeartBeat for LineWorld {
    
}

impl LineTraitTunnel for LineWorld {
    
}

impl LineTraitDns for LineWorld {
    
}

impl LogDir for LineWorld {
    
}


impl Line for LineWorld {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

