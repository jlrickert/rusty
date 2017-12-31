use std::fmt::{Display,Formatter,Result};

#[derive(PartialEq, Debug)]
pub enum Behavior {
    Attacker,
    Defense,
    Raider,
    Settler,
    Sabotage,
}

impl Display for Behavior {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let string = match *self {
            Behavior::Attacker => "attacker",
            Behavior::Defense => "defense",
            Behavior::Raider => "raider",
            Behavior::Settler => "settler",
            Behavior::Sabotage => "sabotage",
        };
        write!(f, "{}", string)
    }
}
