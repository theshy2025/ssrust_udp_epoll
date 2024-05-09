use std::any::Any;

use crate::{config::DNS_AGENT, line::traits::{event::LineTraitEvent, heart_beat::LineTraitHeartBeat, pair::LineTraitPair, status::LineTraitStatus, tunnel::LineTraitTunnel, Line}, log::{buf_writer::LogBufWriter, log_dir::LogDir, Log}};

use super::LineDns;

impl Log for LineDns {
    fn id(&self) -> u64 {
        DNS_AGENT
    }

    fn logger(&mut self) -> &mut LogBufWriter {
        &mut self.basic.log_buf_writer
    }
    
}

impl LineTraitStatus for LineDns {
    
}

impl LineTraitPair for LineDns {
    
}

impl LineTraitEvent for LineDns {
    
}

impl LogDir for LineDns {
    
}

impl LineTraitHeartBeat for LineDns {
    
}

impl LineTraitTunnel for LineDns {
    
}

impl Line for LineDns {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}



