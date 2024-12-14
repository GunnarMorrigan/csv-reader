use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize, Hash)]
pub struct Client(u16);

#[cfg(test)]
impl Client {
    pub fn new(id: u16) -> Self {
        Self(id)
    }
}
