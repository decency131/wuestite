/// [Entity] is a unique identifier that can have [`Component`](crate::Component) attached to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u64);

impl Entity {
    /// Creates a new [Entity] with the given unique identifier.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the unique identifier of the [Entity].
    pub fn id(&self) -> u64 {
        self.0
    }
}
