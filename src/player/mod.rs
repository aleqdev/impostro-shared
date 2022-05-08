pub mod ip_hash;

use serde::{Deserialize, Serialize};

use super::role::Role;

pub use ip_hash::*;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Player {
    pub name: String,
    pub role: Role,
    #[serde(skip_serializing, skip_deserializing)]
    pub ip_hash: IpHash
}

impl Player {
    pub fn new(name: String, role: Role, ip_hash: IpHash) -> Self {
        Self { 
            name, 
            role,
            ip_hash
        }
    }
}
