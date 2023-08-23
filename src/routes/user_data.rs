use sqlx::{PgPool,Transaction,Postgres};
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError, Result};
use anyhow::Context;
use uuid::Uuid;

#[derive(serde::Deserialize,Debug)]
pub struct UserInfo{
    id: String,
}
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserData{
    id: Uuid,
    name: String,
    level: i32,
    follower: i32,
    progress: i32,
    token: i32,
    gold: i32,
    energy: i32,
    wood: i32,
    leather: i32,
    iron: i32,
    fabric: i32,
}

pub async fn fetch_userdata(
    form: web::Form<UserInfo>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserDataError> {

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to aquire postgres connection from pool")?;

    let data = get_user_data(&mut transaction, &form.0.id)
        .await
        .context("Failed to get user from the database.")?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction")?;

    Ok(HttpResponse::Ok().json(web::Json(data)))

}
async fn get_user_data(
    transaction: &mut Transaction<'_,Postgres>,
    id: &str
) -> Result<UserData, sqlx::Error> {
    let userdata: UserData = sqlx::query_as!(
        UserData,
        r#"
        SELECT id, name, level, follower, progress, token, gold, energy, wood, leather, iron, fabric
        FROM users
        WHERE name = $1
        "#,
        id
    ).fetch_one(transaction).await.map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(userdata)
}

//NOTE: Custom Error Type
#[derive(thiserror::Error)]
pub enum UserDataError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
impl std::fmt::Debug for UserDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl ResponseError for UserDataError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserDataError::ValidationError(_) => StatusCode::BAD_REQUEST,
            UserDataError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by: \n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

