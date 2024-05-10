use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::Local;

use crate::{config::ATYP_HOST_NAME, log::{self}};

static mut FRAME: i32 = 0;

static mut ID: u64 = 10;

pub fn frame() -> i32 {
    unsafe { FRAME }
}

pub fn next_frame() {
    unsafe { FRAME = FRAME + 1 };
}

pub fn next_id() -> u64 {
    unsafe { 
        ID = ID + 1;
        return ID; 
    };
}

pub fn u8r(input:u8) -> u8 {
    if input > 45 && input < 255 - 45 {
        255 - input
    } else {
        input
    }
}

pub fn reverse(buf:&mut[u8]) {
    for i in 0..buf.len() {
        buf[i] = u8r(buf[i]);
    }
}

pub fn now() -> i64 {
    let now = Local::now();
    now.timestamp()
}

pub fn now_millis() -> i64 {
    let now = Local::now();
    now.timestamp_millis()
}

pub fn _tsc() -> u64 {
    unsafe { std::arch::x86_64::_rdtsc() }
}

pub fn u64_from_slice(input:&[u8]) -> u64 {
    assert_eq!(input.len(),8);
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(input);
    u64::from_be_bytes(bytes)
}

pub fn i64_from_slice(input:&[u8]) -> i64 {
    assert_eq!(input.len(),8);
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(input);
    i64::from_be_bytes(bytes)
}

pub fn hash(input:&[u8]) -> [u8; 8] {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish().to_be_bytes()
}

pub fn decode_host_name(buf:&[u8]) -> String {
    assert!(buf.len() > 5);
    
    let mut host = String::new();
    let mut port = 0;

    let buf_len = buf.len();
    
    let atyp = buf[3];//ATYP index 3
    
    match atyp {
        ATYP_HOST_NAME => {
            let len = buf[4] as usize;
            let host_buf = &buf[5..5+len];

            match String::from_utf8(host_buf.to_vec()) {
                Ok(ret) => host = ret,
                Err(e) => log::err(format!("{:?}",e)),
            }
            
            let p1 = buf[buf_len-2];
            let p2 = buf[buf_len-1];
            port = u16::from_be_bytes([p1,p2]);
        },

        other => {
            let m = format!("fail decode host name {},{:?}",other,buf);
            log::err(m);
        },
    }
    
    format!("{}:{}",host,port)
}
