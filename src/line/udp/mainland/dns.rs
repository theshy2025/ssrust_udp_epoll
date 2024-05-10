use crate::{line::{line_header::Step, traits::{dns::LineTraitDns, pair::LineTraitPair}}, log::Log};

use super::LineUdp2MainLand;

impl LineTraitDns for LineUdp2MainLand {
    fn dns_collect(&mut self) -> Option<String> {
        if self.step == Step::WaitingDnsCollect {
            self.step = Step::WaitingDnsResult;
            let host = self.peer_ip_port.split(":").next().unwrap();
            Some(host.to_string())
        } else {
            None
        }
    }

    fn dns_query_success(&mut self,world_id:u64) {
        self.log(format!("dns_query_success world_id:{}",world_id));
        self.set_pair_id(world_id);
        self.step = Step::DnsQuerySuccess;
    }

    fn on_world_connect_success(&mut self) {
        let len = self.client_hello_data.len();
        self.log(format!("on_world_connect_success client_hello len {}",len));
        self.step = Step::WorldConnectSuccess;
    }

    fn move_out_client_hello_data(&mut self) -> Option<Vec<u8>> {
        
        if self.step != Step::WorldConnectSuccess {
            return None;
        }

        let len = self.client_hello_data.len();
        
        if len > 0 {
            let data = self.client_hello_data.clone();
            self.client_hello_data.clear();
            self.log(format!("move_out_client_hello_data {}",len));
            self.step = Step::ClientHelloDone;
            Some(data)
        } else {
            None
        }
    }

    fn dns_query_fail(&mut self) {
        self.log(format!("dns_query_fail {:?}",self.peer_ip_port));
    }
}

