use std::fmt::{Display, Formatter, Result};
use hlt::parse::Decodable;

#[derive(PartialEq, Debug)]
pub enum DockingStatus {
    UNDOCKED = 0,
    DOCKING = 1,
    DOCKED = 2,
    UNDOCKING = 3,
}

impl Decodable for DockingStatus {
    fn parse<'a, I>(tokens: &mut I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {

        let raw = i32::parse(tokens);
        match raw {
            0 => DockingStatus::UNDOCKED,
            1 => DockingStatus::DOCKING,
            2 => DockingStatus::DOCKED,
            3 => DockingStatus::UNDOCKING,
            _ => panic!("Not a valid docking status: {:?}", raw),
        }
    }
}


impl Display for DockingStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match *self {
            DockingStatus::UNDOCKED => "undocked",
            DockingStatus::DOCKING => "docked",
            DockingStatus::DOCKED => "docked",
            DockingStatus::UNDOCKING => "undocking",
        })
    }
}
