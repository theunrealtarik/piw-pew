use crate::components::health::Health;
use lib::types::Position;

pub trait Entity {
    fn collides<T>(entity: T) -> bool
    where
        T: Entity,
    {
        true
    }

    fn position(&self) -> &Position;
    fn health(&self) -> &Health;
}
