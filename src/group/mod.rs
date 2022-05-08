use serde::{Deserialize, Serialize};

pub mod members;
pub mod name;

pub use members::*;
pub use name::*;

use crate::resources::Resources;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Group {
    pub name: GroupName,
    pub members: GroupMembers,
    pub resources: Resources,
}

impl Group {
    pub fn new(name: String) -> Self {
        Self {
            name: GroupName(name),
            members: Default::default(),
            resources: Resources::default(),
        }
    }

    pub fn init_resources(&mut self, rooms_count: usize) {
        self.resources = Resources::new(rooms_count, self.members.iter())
    }
}
