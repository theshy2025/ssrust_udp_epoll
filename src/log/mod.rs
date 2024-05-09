use std::{fs::{self, File, OpenOptions}, io::Write};

use crate::config;

use self::buf_writer::LogBufWriter;

pub mod buf_writer;
pub mod log_dir;

pub trait Log {
    fn id(&self) -> u64 {0}

    fn logger(&mut self) -> &mut LogBufWriter;

    fn log(&mut self,s:String) {
        self.logger().add(s);
        self.logger().flush();
    }

    fn err(&mut self,s:String) {
        crate::log::err(format!("[{}]{}",self.id(),s));
        self.log(s);
    }
}

pub fn def(s:String) {
    let dir = device_log_path();
    write(format!("[{}]{}\n",crate::global::frame(),s),format!("{}/default.log",dir));
}

pub fn err(s:String) {
    let dir = device_log_path();
    write(format!("[{}]{}\n",crate::global::frame(),s),format!("{}/err.log",dir));
}

pub fn init() {
    let dir = device_log_path();
    match fs::remove_dir_all( &dir ) {
        Ok(_) => {}
        Err(_) => {},
    }
    fs::create_dir_all( &dir ).unwrap();
    File::create( format!("{}/default.log",&dir) ).unwrap();
    File::create( format!("{}/err.log",&dir) ).unwrap();
}

pub fn device_log_path() -> String {
    let device = config::loader::get("device").unwrap();
    format!("{}_log",device)
}

fn write(s:String,path:String) {
    match OpenOptions::new().append(true).open( &path ) {
        Ok(mut f) => {
            f.write(s.as_bytes()).unwrap();
        },
        Err(e) => println!("{:?},{:?}",e,path)
    }
}