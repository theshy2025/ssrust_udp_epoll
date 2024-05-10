use crate::{line::{line_header::DataType, traits::{tunnel_response::LineTraitTunnelResponse, pair::LineTraitPair}}, log::Log};

use super::LinePc;

impl LineTraitPair for LinePc {
    fn pair_id(&self) -> u64 {
        self.pair_id
    }
    
    fn set_pair_id(&mut self,new:u64) {
        self.pair_id = new;
    }

    fn on_pair_data(&mut self,buf:&[u8],data_type:DataType) {
        assert_eq!(data_type,DataType::Http);
        let dt = DataType::from(buf[0]);
        assert_eq!(dt,DataType::Http);
        self.log(format!("{} bytes data from pair",buf.len()));
        self.on_tunnel_response_packet(&buf[1..]);
    }
}