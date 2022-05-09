pub mod counter;
pub mod id;
pub mod groups;

pub use counter::*;
pub use id::*;
pub use groups::*;

use serde::{Deserialize, Serialize};

use crate::group::{Group, GroupName};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Session {
    pub id: SessionId,
    pub groups: SessionGroups,
}

impl Session {
    pub fn new(counter: &mut SessionCounter) -> Self {
        Self {
            id: SessionId::new_random(counter),
            groups: SessionGroups::default()
        }
    }

    pub fn get_group<'a, 'b: 'a>(&'b self, name: GroupName) -> Option<&'a Group> {
        for g in self.groups.iter() {
            if g.name == name {
                return Some(g);
            }
        }
        return None;
    }
}