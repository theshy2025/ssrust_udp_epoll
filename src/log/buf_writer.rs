use std::{fs::File, io::{BufWriter, Write}};

use chrono::{Local, Timelike};

pub struct LogBufWriter { 
    bw:BufWriter<File>
}

impl LogBufWriter {
    pub fn new(path:String) -> Option<LogBufWriter> {
        match File::create(&path) {
            Ok(f) => {
                let bw = BufWriter::new(f);
                Some(LogBufWriter { bw })
            },
            Err(e) => {
                println!("{} {:?}",e,path);
                None
            },
        }
    }
}

impl LogBufWriter {
    pub fn add(&mut self,s:String) {
        let now = Local::now();
        let t = format!("[{}:{:02}:{:02}:{}]",now.hour(),now.minute(),now.second(),now.timestamp_subsec_millis());
        let s = format!("[{}]{}{}",crate::global::frame(),t,s);
        writeln!(self.bw,"{s}").unwrap();
    }

    pub fn flush(&mut self) {
        self.bw.flush().unwrap();
    }
}

