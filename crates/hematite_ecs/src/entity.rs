#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    pub(crate) fn new(id: u64) -> Self {
        Self(id)
    }
}