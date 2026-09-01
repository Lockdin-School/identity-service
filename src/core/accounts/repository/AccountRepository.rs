use crate::core::accounts::models::Account::Account;
use crate::core::accounts::models::AccountNew::AccountNew;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait AccountRepository {
    async fn create_account(&self, new_account: AccountNew) -> sqlx::Result<Account, sqlx::Error>;
    async fn get_account_by_id(&self, id: Uuid) -> sqlx::Result<Option<Account>, sqlx::Error>;
    async fn get_account_by_cognito_sub(
        &self,
        cognito_sub: String,
    ) -> sqlx::Result<Option<Account>, sqlx::Error>;
    async fn update_account(
        &self,
        id: Uuid,
        new_account: AccountNew,
    ) -> sqlx::Result<bool, sqlx::Error>;
}
