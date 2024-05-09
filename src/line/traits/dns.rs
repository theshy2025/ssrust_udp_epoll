pub trait LineTraitDns {
    fn dns_collect(&mut self) -> Option<String> {None}
    fn new_dns_query(&mut self,_id:u64,_host:String) {}
    fn dns_query_fail(&mut self) {}
    fn dns_query_success(&mut self,_world_id:u64) {}
    fn on_world_connect_success(&mut self) {}
    fn move_out_client_hello_data(&mut self) -> Option<Vec<u8>> {None}
}