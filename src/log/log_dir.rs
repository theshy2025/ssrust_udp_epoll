use std::fs;

use super::buf_writer::LogBufWriter;

pub trait LogDir {
    fn dir_name() -> String {
        let ret = std::any::type_name::<Self>();
        let name = ret.split("::").last().unwrap();
        name.to_string()
    }

    fn create_dir() {
        let log_dir = crate::log::device_log_path();
        let dir = Self::dir_name();
        let path = format!("{}/{}",log_dir,dir);
        fs::create_dir_all( path ).unwrap();
    }

    fn create_log_buf_writer(id:u64) -> LogBufWriter {
        let log_dir = crate::log::device_log_path();
        let dir = Self::dir_name();
        let path = format!("{}/{}/{}.log",log_dir,dir,id);
        LogBufWriter::new(path).unwrap()
    }
}