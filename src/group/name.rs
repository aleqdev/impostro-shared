use std::ops::{Deref, DerefMut};


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GroupName(pub String);

impl Deref for GroupName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GroupName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}