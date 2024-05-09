use crate::{line::traits::status::{LineTraitStatus, Status}, log::Log};

use super::LineUdp2Vps;

impl LineTraitStatus for LineUdp2Vps {
    fn status(&self) -> Status {
        self.basic.status
    }

    fn set_status(&mut self,new:Status) {
        let old = self.basic.status;
        self.basic.status = new;
        self.log(format!("status {:?} to {:?}",old,new));
    }
}

impl LineUdp2Vps {
    pub fn is_ready(&self) -> bool {
        if self.pair_id > 0 {
            return false;
        }

        if self.status() == Status::Establish {
            return true;
        }
        
        false
    }
}
