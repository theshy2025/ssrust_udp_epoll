use crate::{line::{line_header::DataType, traits::{pair::LineTraitPair, tunnel::LineTraitTunnel}}, log::Log};

use super::LineUdp2MainLand;

impl LineTraitPair for LineUdp2MainLand {
    fn pair_id(&self) -> u64 {
        self.pair_id
    }
    
    fn set_pair_id(&mut self,id:u64) {
        self.pair_id = id;
    }

    fn on_pair_data(&mut self,buf:&[u8],data_type:DataType) {
        assert_eq!(data_type,DataType::Http);
        self.log(format!("on data from pair {} bytes,http_send_queue_len:{}",buf.len(),self.http_send_queue.len()));
        self.send_http_buf(buf);
    }
}
