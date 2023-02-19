use std::convert::TryInto;

use crate::{
    data::{query, DatabasePool},
    Clip, ServiceError,
};

use super::ask;

pub async fn new_clip(req: ask::NewClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::create_clip(req, pool).await?.try_into()?)
}

pub async fn update_clip(req: ask::UpdateClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    Ok(query::update_clip(req, pool).await?.try_into()?)
}

pub async fn get_clip(req: ask::GetClip, pool: &DatabasePool) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone();

    let clip: Clip = query::get_clip(req, pool).await?.try_into()?;
    if clip.password.has_password() {
        if user_password == clip.password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError(
                "Permissions Not Met".to_owned(),
            ))
        }
    } else {
        Ok(clip)
    }
}
