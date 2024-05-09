use std::time::Instant;

use crate::{line::traits::status::{LineTraitStatus, Status}, log::Log};

use super::LineUdp2MainLand;

impl LineTraitStatus for LineUdp2MainLand {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
        if new == Status::PairClose {
            self.clock = Instant::now();
        }
    }
}