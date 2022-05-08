use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::group::Group;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub struct SessionGroups(pub Vec<Group>);

impl<T, U> From<T> for SessionGroups
where 
    T: Iterator<Item = U>,
    Vec<Group>: FromIterator<U>
{
    fn from(iter: T) -> Self {
        Self(iter.collect())
    }
}

impl Deref for SessionGroups {
    type Target = Vec<Group>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SessionGroups {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}