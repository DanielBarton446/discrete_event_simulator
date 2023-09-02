use std::fmt::{Display, Error, Formatter};

#[derive(Copy, Clone)]
pub struct Passenger {
    uid: usize,
}

impl Passenger {
    pub fn new(uid: usize) -> Passenger {
        Passenger { uid }
    }
}

impl Display for Passenger {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Passenger {}", self.uid)
    }
}
