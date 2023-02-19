use std::{convert::TryFrom, str::FromStr};

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::{
    domain::{self, clip::field::Password},
    service::ask,
    ClipError, ShortCode, Time,
};

use super::DbId;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

impl TryFrom<Clip> for domain::Clip {
    type Error = ClipError;

    fn try_from(value: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;
        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(value.clip_id.as_str())?),
            shortcode: field::ShortCode::from(value.shortcode),
            content: field::Content::new(value.content.as_str())?,
            title: field::Title::new(value.title),
            posted: field::Posted::new(Time::from_naive_utc(value.posted)),
            expires: field::Expires::new(value.expires.map(Time::from_naive_utc)),
            password: field::Password::new(value.password.unwrap_or_default())?,
            hits: field::Hits::new(u64::try_from(value.hits)?),
        })
    }
}

#[derive(Debug)]
pub struct GetClip {
    pub(in crate::data) shortcode: String,
}

impl From<crate::service::ask::GetClip> for GetClip {
    fn from(req: crate::service::ask::GetClip) -> Self {
        Self {
            shortcode: req.shortcode.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(value: String) -> Self {
        Self { shortcode: value }
    }
}

impl From<ShortCode> for GetClip {
    fn from(value: ShortCode) -> Self {
        Self {
            shortcode: value.into_inner(),
        }
    }
}

#[derive(Debug)]
pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: i64,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<ask::NewClip> for NewClip {
    fn from(value: ask::NewClip) -> Self {
        Self {
            clip_id: DbId::new().into(),
            shortcode: ShortCode::default().into(),
            content: value.content.into_inner(),
            title: value.title.into_inner(),
            posted: Utc::now().timestamp(),
            expires: value.expires.into_inner().map(|time| time.timestamp()),
            password: value.password.into_inner(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<i64>,
    pub(in crate::data) password: Option<String>,
}

impl From<ask::UpdateClip> for UpdateClip {
    fn from(value: ask::UpdateClip) -> Self {
        Self {
            shortcode: value.shortcode.into_inner(),
            content: value.content.into_inner(),
            title: value.title.into_inner(),
            expires: value.expires.into_inner().map(|time| time.timestamp()),
            password: value.password.into_inner(),
        }
    }
}
