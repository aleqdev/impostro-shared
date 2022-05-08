use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Deserialize, Serialize)]
pub struct SessionCounter(pub usize);

impl Deref for SessionCounter {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SessionCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}