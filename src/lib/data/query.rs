use super::{
    model::{Clip, NewClip},
    model::{GetClip, UpdateClip},
    DataError, DatabasePool,
};

type Result<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<GetClip>>(model: M, pool: &DatabasePool) -> Result<Clip> {
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(
        sqlx::query_as!(Clip, "SELECT * FROM clips WHERE shortcode = ?", shortcode)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn create_clip<M>(model: M, pool: &DatabasePool) -> Result<Clip>
where
    M: Into<NewClip>,
{
    let model = model.into();
    let _ = sqlx::query!(
        r#"
        INSERT INTO clips
        (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits
        ) values
        (
            ?, ?, ?, ?, ?, ?, ?, ?
        )
      "#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M>(model: M, pool: &DatabasePool) -> Result<Clip>
where
    M: Into<UpdateClip>,
{
    let model: UpdateClip = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
                content = ?,
                title = ?,
                expires = ?,
                password = ?
            WHERE shortcode = ?
        "#,
        model.content,
        model.title,
        model.expires,
        model.password,
        model.shortcode
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}
