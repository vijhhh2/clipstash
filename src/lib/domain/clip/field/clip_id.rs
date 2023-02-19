use serde::{Serialize, Deserialize};
use crate::data::DbId;
use derive_more::Constructor;

#[derive(Debug, Clone, Constructor, Serialize, Deserialize)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn into_inner(self) -> DbId {
        self.0
    }
}
impl From<DbId> for ClipId {
    fn from(value: DbId) -> Self {
        Self(value)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}