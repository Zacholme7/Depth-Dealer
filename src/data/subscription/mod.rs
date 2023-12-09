use std::fmt::Debug;



// defines the type of a subscription adn the event that it yields
// subkind should implement Debug and clone
// the event should implement debug
pub trait SubKind
where
    Self: Debug + Clone
{
    type Event: Debug;
}