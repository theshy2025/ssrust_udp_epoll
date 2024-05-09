use crate::log::Log;

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum Status {
    Raw,
    WriteOpen,
    Register,
    Establish,
    PairClose,
    ReadClose,
    WriteClose,
    CoolDown,
    ReadWriteBothClose,
    DeRegister,
    Close,
    Dead,
}

pub trait LineTraitStatus : Log {
    fn status(&self) -> Status {
        Status::Raw
    }

    fn set_status(&mut self,_new:Status){}

    fn next_status(&mut self) {
        let mut next:Option<Status> = None;

        match self.status() {
            Status::Raw => {},
            Status::WriteOpen => next = Some(Status::Register),
            Status::Register => {},
            Status::Establish => {},
            Status::PairClose => next = Some(Status::CoolDown),
            Status::ReadClose => {},
            Status::WriteClose => {},
            Status::CoolDown => next = Some(Status::ReadWriteBothClose),
            Status::ReadWriteBothClose => next = Some(Status::DeRegister),
            Status::DeRegister => next = Some(Status::Close),
            Status::Close => next = Some(Status::Dead),
            Status::Dead => {},
            
        }

        match next {
            Some(new) => self.set_status(new),
            None => {},
        }
    }

}