#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u32);

pub struct IdGenerator {
    next_id: u32,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn generate(&mut self) -> Id {
        let id = Id(self.next_id);
        self.next_id += 1;
        id
    }
}
