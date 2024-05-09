use crate::log::buf_writer::LogBufWriter;

use super::traits::status::Status;

pub struct BaseLine {
    pub id:u64,
    pub status:Status,
    pub log_buf_writer:LogBufWriter,
}

impl BaseLine {
    pub fn new(id:u64,log_buf_writer:LogBufWriter) -> BaseLine {
        BaseLine { id, log_buf_writer, status: Status::Raw }
    }
}