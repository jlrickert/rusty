use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Command {
    Dock(i32, i32),
    Undock(i32),
    Thrust(i32, i32, i32),
    Nop,
}

impl Command {
    pub fn encode(&self) -> String {
        match *self {
            Command::Dock(s, p) => format!("d {} {}", s, p),
            Command::Undock(s) => format!("u {}", s),
            Command::Thrust(s, m, a) => format!("t {} {} {}", s, m, a),
            Command::Nop => "".to_string(),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let msg = match *self {
            Command::Dock(_, _) => format!("Dock({})", self.encode()),
            Command::Undock(_) => format!("Undock({})", self.encode()),
            Command::Thrust(_,_,_) => format!("Thrust({})", self.encode()),
            Command::Nop => "Nop".to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn test_thing() {
        assert_eq!("d 10 4", Command::Dock(10, 4).encode());
        assert_eq!("t 3 9 4", Command::Thrust(3, 9, 4).encode());
        assert_eq!("u 3", Command::Undock(3).encode());
    }
}
