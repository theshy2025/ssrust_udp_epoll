#[derive(Debug,PartialEq, Eq)]
pub enum Step {
    Raw,
    WaitingDnsCollect,
    WaitingDnsResult,
    DnsQuerySuccess,
    WorldConnectSuccess,
    ClientHelloDone,
    Http,
}

/* 
#[derive(Debug,PartialEq, Eq)]
pub enum OrderResult {
    Ignore,
    Normal,
    Save,
}
*/

#[derive(Debug,PartialEq, Eq)]
pub enum DataType {
    Error,
    Port,
    HeartBeat,
    Sni,
    ClientHello,
    Http,
    Ack,
}

impl DataType {
    pub fn u8(&self) -> u8 {
        match self {
            DataType::Error => 0,
            DataType::Port => 1,
            DataType::HeartBeat => 2,
            DataType::Sni => 3,
            DataType::ClientHello => 4,
            DataType::Http => 5,
            DataType::Ack => 6,
        }
    }

    pub fn from(v:u8) -> DataType {
        match v {
            1 => DataType::Port,
            2 => DataType::HeartBeat,
            3 => DataType::Sni,
            4 => DataType::ClientHello,
            5 => DataType::Http,
            6 => DataType::Ack,
            _ => DataType::Error,
        }
    }
}
