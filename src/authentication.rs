use crate::logging::spawn_blocking_with_tracing;
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}

struct Credentials {
    username: String,
    password: Secret<String>,
}

async fn validate_credentials(credentials: Credentials, pool: &PgPool) -> Result<Uuid, AuthError> {
    let mut user_id = None;
    let mut correct_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
            .to_string(),
    );

    if let Some((stored_user_id, stored_hash)) =
        get_stored_credentials(&credentials.username, &pool)
            .await
            .context("failed to get stored credentials from db")?
    {
        user_id = Some(stored_user_id);
        correct_hash = stored_hash;
    }

    spawn_blocking_with_tracing(move || verify_password_hash(correct_hash, credentials.password))
        .await
        .context("failed to spawn blocking task")
        .map_err(AuthError::UnexpectedError)??;

    user_id.ok_or_else(|| AuthError::InvalidCredentials)
}

async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(Uuid, Secret<String>)>, sqlx::Error> {
    // let row: Option<_> = sqlx::query!(
    //     r#"SELECT user_id, password_hash FROM users WHERE username = $1"#,
    //     credentials.username
    // )
    // .fetch_optional(pool)
    // .await?
    // .map(|row| (row.user_id, Secret::new(row.password_hash)));
    // Ok(row)
    Ok(None)
}

fn verify_password_hash(
    correct_hash: Secret<String>,
    candidate: Secret<String>,
) -> Result<(), AuthError> {
    let correct_hash = PasswordHash::new(&correct_hash.expose_secret())
        .context("failed to parse PHC string")
        .map_err(AuthError::UnexpectedError)?;

    Argon2::default()
        .verify_password(candidate.expose_secret().as_bytes(), &correct_hash)
        .map_err(|_| AuthError::InvalidCredentials)?;

    Ok(())
}
