pub struct Minion {}

impl Minion {
    pub fn new() -> Minion {
        Minion {}
    }

    pub fn state(&self) -> MinionState {
        MinionState {}
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinionState {}
