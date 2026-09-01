use crate::core::accounts::models::Account::Account;
use crate::core::accounts::models::AccountNew::AccountNew;
use crate::core::accounts::repository::AccountRepository::AccountRepository;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct PostgresAccountRepository {
    pool: PgPool,
}

impl PostgresAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AccountRepository for PostgresAccountRepository {
    async fn create_account(&self, new_account: AccountNew) -> sqlx::Result<Account, Error> {
        let id = Uuid::now_v7();
        sqlx::query_as(
            "INSERT INTO account(id, cognito_sub, email, email_verified, status) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
            .bind(id)
            .bind(&new_account.cognito_sub)
            .bind(&new_account.email)
            .bind(new_account.email_verified)
            .bind(new_account.status)
            .fetch_one(&self.pool)
            .await
    }

    async fn get_account_by_id(&self, id: Uuid) -> sqlx::Result<Option<Account>, Error> {
        sqlx::query_as("SELECT * FROM account WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn get_account_by_cognito_sub(
        &self,
        cognito_sub: String,
    ) -> sqlx::Result<Option<Account>, Error> {
        sqlx::query_as("SELECT * FROM account WHERE cognito_sub = $1")
            .bind(&cognito_sub)
            .fetch_optional(&self.pool)
            .await
    }

    async fn update_account(&self, id: Uuid, account_new: AccountNew) -> sqlx::Result<bool, Error> {
        let result = sqlx::query(
            r#"
            UPDATE account
            SET
                email = $2,
                email_verified = $3,
                status = $4
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(account_new.email)
        .bind(account_new.email_verified)
        .bind(account_new.status)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
