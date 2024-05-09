use std::env;

use gate::Gate;

mod global;
mod config;
mod log;
mod gate;
mod line;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("need to be [cargo run local] or [cargo run server]");
    }
    config::loader::load(&args[1]);

    log::init();

    Gate::new().start();

}
