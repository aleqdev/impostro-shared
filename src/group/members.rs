use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::player::Player;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub struct GroupMembers(pub Vec<Player>);

impl<T, U> From<T> for GroupMembers
where 
    T: Iterator<Item = U>,
    Vec<Player>: FromIterator<U>
{
    fn from(iter: T) -> Self {
        Self(iter.collect())
    }
}

impl Deref for GroupMembers {
    type Target = Vec<Player>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GroupMembers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}