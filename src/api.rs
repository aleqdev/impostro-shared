use serde::{Deserialize, Serialize};

use crate::{session::SessionId, group::GroupName};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GetSessions {
    pub pass: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GetGroups {
    pub session: SessionId
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct GetMembers {
    pub session: SessionId,
    pub group_name: GroupName
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ValidateSessionId {
    pub session: SessionId,
}