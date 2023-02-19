use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}