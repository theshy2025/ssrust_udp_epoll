use std::collections::HashMap;

use crate::line::traits::tunnel_response::LineTraitTunnelResponse;

use super::LineWorld;

impl LineTraitTunnelResponse for LineWorld {
    fn last_normal(&self) -> u64 {
        self.last_normal_tunnel_response_packet_id
    }

    fn update_last_normal(&mut self,new:u64) {
        self.last_normal_tunnel_response_packet_id = new;
    }

    fn tunnel_response_packets(&mut self) -> Option<&mut HashMap<u64,Vec<u8>>> {
        Some(&mut self.tunnel_response_packets)
    }
}