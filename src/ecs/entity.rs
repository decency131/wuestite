#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(usize);

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity(id)
    }
}