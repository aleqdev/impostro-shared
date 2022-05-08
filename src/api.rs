use serde::{Deserialize, Serialize};

use crate::{session::SessionId, group::GroupName};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GetSessions {
    pass: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GetGroups {
    session: SessionId
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GetMembers {
    session: SessionId,
    group_name: GroupName
}