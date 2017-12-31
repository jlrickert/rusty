use std::fmt::{Display,Formatter,Result};

#[derive(PartialEq, Debug)]
pub enum Behavior {
    Defense,
    Raider,
    Settler,
}

impl Display for Behavior {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let string = match *self {
            Behavior::Defense => "defense",
            Behavior::Raider => "raider",
            Behavior::Settler => "settler",
        };
        write!(f, "{}", string)
    }
}
