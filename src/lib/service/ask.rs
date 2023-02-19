use crate::domain::clip::field::*;
use crate::ShortCode;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub content: Content,
    pub title: Title,
    pub expires: Expires,
    pub password: Password,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClip {
    pub content: Content,
    pub title: Title,
    pub expires: Expires,
    pub password: Password,
    pub shortcode: ShortCode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: Password,
}

impl GetClip {
    pub fn from_raw(raw: &str) -> Self {
        Self {
            shortcode: ShortCode::from(raw),
            password: Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(value: ShortCode) -> Self {
        Self {
            shortcode: value,
            password: Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(value: &str) -> Self {
        Self::from_raw(value)
    }
}
