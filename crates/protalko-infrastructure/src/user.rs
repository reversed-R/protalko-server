use std::ops::Deref;

use protalko_domain::{
    entities::user::{User, UserId, UserName, UserVisibleId},
    repositories::user::{UserRepository, UserRepositoryError},
};

use crate::shared::postgresql::Postgresql;

use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    name: String,
    visible_id: String,
}

struct UserIdColumn {
    id: Uuid,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self::new(
            UserId::new(value.id),
            UserVisibleId::new(value.visible_id),
            UserName::new(value.name),
        )
    }
}

pub struct PgUserRepository {
    db: Postgresql,
}

impl PgUserRepository {
    pub fn new(db: Postgresql) -> Self {
        Self { db }
    }
}

impl UserRepository for PgUserRepository {
    async fn create(&self, user: User) -> Result<UserId, UserRepositoryError> {
        let res = sqlx::query_as!(
            UserIdColumn,
            r#"
           INSERT INTO users (name, visible_id)
           VALUES ($1, $2)
           RETURNING id
            "#,
            user.name().clone().value(),
            user.visible_id().clone().value()
        )
        .fetch_one(self.db.deref())
        .await;

        match res {
            Ok(id_column) => Ok(UserId::new(id_column.id)),
            Err(e) => match e.as_database_error() {
                Some(e) => Err(UserRepositoryError::InternalError(e.message().to_string())),
                _ => Err(UserRepositoryError::InternalError(e.to_string())),
            },
        }
    }

    async fn update(&self, user: User) -> Result<(), UserRepositoryError> {
        let res = sqlx::query!(
            r#"
           UPDATE users
           SET name = $2, visible_id = $3
           WHERE id = $1
            "#,
            user.id().clone().value(),
            user.name().clone().value(),
            user.visible_id().clone().value()
        )
        .execute(self.db.deref())
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.as_database_error() {
                Some(e) => Err(UserRepositoryError::InternalError(e.message().to_string())),
                _ => Err(UserRepositoryError::InternalError(e.to_string())),
            },
        }
    }

    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, UserRepositoryError> {
        let res = sqlx::query_as!(
            UserRow,
            r#"
           SELECT id, visible_id, name
           FROM users
           WHERE id = $1 AND deleted_at IS NULL
            "#,
            id.value(),
        )
        .fetch_optional(self.db.deref())
        .await;

        match res {
            Ok(optional) => match optional {
                Some(user) => Ok(Some(User::from(user))),
                None => Ok(None),
            },
            Err(e) => match e.as_database_error() {
                Some(e) => Err(UserRepositoryError::InternalError(e.message().to_string())),
                _ => Err(UserRepositoryError::InternalError(e.to_string())),
            },
        }
    }

    async fn delete_by_id(&self, id: UserId) -> Result<(), UserRepositoryError> {
        let res = sqlx::query_as!(
            UserRow,
            r#"
           UPDATE users
           SET deleted_at = NOW()
           WHERE id = $1 AND deleted_at IS NULL
            "#,
            id.value(),
        )
        .fetch_one(self.db.deref())
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.as_database_error() {
                Some(e) => Err(UserRepositoryError::InternalError(e.message().to_string())),
                _ => Err(UserRepositoryError::InternalError(e.to_string())),
            },
        }
    }
}
