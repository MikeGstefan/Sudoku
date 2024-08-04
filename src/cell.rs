use std::fmt::{self, Display};
#[derive(Clone,Copy)]

pub struct Cell{
    sol: Option<u8>,
}

impl Cell{
    pub fn new() -> Cell{
        Cell{
            sol: None,
        }
    }
}

impl Display for Cell{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.sol.unwrap_or(0))
    }
}

