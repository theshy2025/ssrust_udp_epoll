use crate::line::line_header::DataType;

use super::{network::LineTraitNetWork, status::Status};

pub trait LineTraitPair : LineTraitNetWork {
    fn pair_id(&self) -> u64 {
        0
    }

    fn set_pair_id(&mut self,_id:u64){}

    fn on_pair_close(&mut self) {
        self.log(format!("on_pair_close"));
        self.set_pair_id(0);
        self.set_status(Status::PairClose);
    }

    fn on_pair_data(&mut self,buf:&[u8],data_type:DataType) {
        self.log(format!("{} bytes data from pair,data_type:{:?}",buf.len(),data_type));
        self.socket_send(buf);
    }
}