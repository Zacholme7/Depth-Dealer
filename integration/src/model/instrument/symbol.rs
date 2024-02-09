use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Debug, Display, Formatter};

/// Represents an individiaul symbol
/// Ex: "btc", "etc", ...
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct Symbol(String);


// Implement the Debug and Display for easy printing
impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}



impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Symbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Symbol::new)
    }
}

// From (Into<String>) to Symbol
// Allow for the coversion from anything that can be converted into a string (String, &str) into a symbol
impl<S> From<S> for Symbol
where
    S: Into<String>,
{
    fn from(input: S) -> Self {
        Symbol::new(input)
    }
}

impl Symbol {
    /// Construct a new [`Symbol`] new type using the provided `Into<Symbol>` value.
    pub fn new<S>(input: S) -> Self
    where
        S: Into<String>,
    {
        Self(input.into().to_lowercase())
    }
}
