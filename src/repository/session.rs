use fake::{Fake, rand};
use sqlx::SqlitePool;

use crate::models::{
  dto::{CreateSessionReq, CreateSessionRes, InitSessionReq, InitSessionRes},
  error::Error,
};

pub struct Session<'a> {
  db: &'a SqlitePool,
}

impl<'a> Session<'a> {
  pub const fn new(db: &'a SqlitePool) -> Self {
    Self { db }
  }

  pub async fn migrate(&self) -> Result<(), Error> {
    sqlx::query(
      r"
        CREATE TABLE session (
          session_id TEXT PRIMARY KEY,
          username TEXT NOT NULL,
          password TEXT NOT NULL,
          is_active BOOLEAN DEFAULT FALSE,
          debug_pipe_name TEXT,
        );
        ",
    )
    .execute(self.db)
    .await?;
    Ok(())
  }

  pub async fn create(
    &self,
    CreateSessionReq { username, password }: &CreateSessionReq,
  ) -> Result<CreateSessionRes, Error> {
    // Генерируем 16 случайных байт, для id сессии
    // 1 байт = 2 hex-символа, значит 16 байт = 32 hex-символа.
    let session_id = hex::encode(rand::random::<[u8; 16]>()).to_uppercase();

    sqlx::query("INSERT INTO session (session_id, username, password) VALUES ($1, $2, $3)")
      .bind(&session_id)
      .bind(username.as_ref()) // Преобразуем Box<str> в &str для sqlx
      .bind(password.as_ref())
      .execute(self.db)
      .await?;

    Ok(CreateSessionRes {
      session_id: session_id.into_boxed_str(),
    })
  }

  pub async fn init(&self, InitSessionReq { session_id }: &InitSessionReq) -> Result<InitSessionRes, Error> {
    // Authenticated user not found for session: FA6B6C7981D454D931DA7DC66F6AAA78
    let debig_pipe_name = format!("debug${}", (0b0..9_999_999_999).fake::<u64>());

    let result = sqlx::query(
      r"
        UPDATE session
        SET is_active = TRUE, debig_pipe_name = $2
        WHERE session_id = $1
        ",
    )
    .bind(session_id)
    .bind(&debig_pipe_name)
    .execute(self.db)
    .await?;

    if result.rows_affected() == 0 {
      return Err(Error::AuthenticatedUserNotFound(session_id.clone()));
    }

    Ok(InitSessionRes {
      session_id: session_id.clone(),
      debig_pipe_name: debig_pipe_name.into_boxed_str(),
    })
  }
}
