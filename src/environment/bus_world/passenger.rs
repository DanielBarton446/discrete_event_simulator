#[derive(Copy, Clone)]
pub struct Passenger {
    uid: usize,
}

impl Passenger {
    pub fn new(uid: usize) -> Passenger {
        Passenger { uid }
    }
}
