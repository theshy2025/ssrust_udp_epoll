use std::time::Instant;

use crate::{line::traits::status::Status, log::{self, Log}};

use super::Gate;

impl Gate {
    pub fn tick(&mut self) {
        let clock = Instant::now();
        let mut vec = Vec::new();
        
        for (_,v) in self.lines.iter_mut() {
            let id = v.id();
            let st = v.status();
            let pid = v.pair_id();
            vec.push((id,st,pid));
            v.tick();
        }

        let n = clock.elapsed().as_micros();
        if n > 150 {
            self.log(format!("gate_tick_a:{}",n));
        }

        for (id,status,pid) in vec {
            match status {
                Status::WriteOpen => {
                    let line = self.lines.get(&id).unwrap();
                    let fd = line.socket_fd();
                    self.remove_fd(fd);
                    self.register_read_event(fd,id);
                    if pid > 0 {
                        let mainland = self.lines.get_mut(&pid).unwrap();
                        mainland.on_world_connect_success();
                    }
                },

                Status::ReadWriteBothClose => {
                    let line = self.lines.get(&id).unwrap();
                    self.remove_fd( line.socket_fd() );
                },

                Status::Close => { 
                    if pid > 0 {
                        let line = self.lines.get_mut(&pid).expect(&pid.to_string());
                        line.on_pair_close();
                    }
                },
                
                Status::Dead => { self.lines.remove(&id); },
                
                _ => {},
            }
        }

        let n = clock.elapsed().as_micros();
        if n > 250 {
            self.log(format!("gate_tick:{}",n));
        }
    }

    

    pub fn gather_client_hello(&mut self) {
        let mut world:Vec<(u64,Vec<u8>)> = Vec::new();

        for (_,line) in self.lines.iter_mut() {
            match line.move_out_client_hello_data() {
                Some(buf) => {
                    let node = (line.pair_id(),buf);
                    world.push(node);
                },
                
                None => {},
            }
        }

        for (id,buf) in world.iter() {
            match self.lines.get_mut(id) {
                Some(line) => {
                    line.socket_send(buf);
                },
                None => {
                    log::err(format!("error can not find world line {}",id))
                },
            }
        }
    }
}
