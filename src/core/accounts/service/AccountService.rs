use crate::core::accounts::models::Account::Account;
use crate::core::accounts::models::AccountNew::AccountNew;
use crate::core::accounts::repository::AccountRepository::AccountRepository;
use std::sync::Arc;
use tokio::io;
use uuid::Uuid;

pub struct AccountService {
    repo: Arc<dyn AccountRepository + Send + Sync>,
}

impl AccountService {
    pub fn new(repo: Arc<dyn AccountRepository + Send + Sync>) -> Self {
        Self { repo }
    }

    pub async fn create_account(&self, new_account: AccountNew) -> Result<Account, io::Error> {
        log::info!("account.create | service | create_account | started | Creating account...");
        match self.repo.create_account(new_account).await {
            Ok(account) => {
                log::info!(
                    "account.create | service | create_account | success | Account created successfully!"
                );
                Ok(account)
            }
            Err(e) => {
                log::error!(
                    "account.create | service | create_account | failed | Something wrong happened | {}",
                    e
                );
                Err(io::Error::other(e))
            }
        }
    }

    pub async fn get_account_by_id(&self, id: Uuid) -> Result<Account, io::Error> {
        log::info!(
            "account.get | service | get_account_by_id | started | Getting account by id..."
        );
        match self.repo.get_account_by_id(id).await {
            Ok(opt_account) => {
                log::info!(
                    "account.get | service | get_account_by_id | success | Account retrieved successfully!"
                );
                match opt_account {
                    None => {
                        log::info!(
                            "account.get | service | get_account_by_id | success | Account not found!"
                        );
                        Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "Account not found!",
                        ))
                    }
                    Some(account) => {
                        log::info!(
                            "account.get | service | get_account_by_id | success | Account retrieved successfully!"
                        );
                        Ok(account)
                    }
                }
            }
            Err(e) => {
                log::error!(
                    "account.get | service | get_account_by_id | failed | Something wrong happened | {}",
                    e
                );
                Err(io::Error::other(e))
            }
        }
    }

    pub async fn get_account_by_cognito_sub(&self, id: String) -> Result<Account, io::Error> {
        log::info!(
            "account.get | service | get_account_by_cognito_sub | started | Getting account by cognito_sub..."
        );
        match self.repo.get_account_by_cognito_sub(id).await {
            Ok(opt_account) => {
                log::info!(
                    "account.get | service | get_account_by_cognito_sub | success | Account retrieved successfully!"
                );
                match opt_account {
                    None => {
                        log::info!(
                            "account.get | service | get_account_by_cognito_sub | success | Account not found!"
                        );
                        Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "Account not found!",
                        ))
                    }
                    Some(account) => {
                        log::info!(
                            "account.get | service | get_account_by_cognito_sub | success | Account retrieved successfully!"
                        );
                        Ok(account)
                    }
                }
            }
            Err(e) => {
                log::error!(
                    "account.get | service | get_account_by_cognito_sub | failed | Something wrong happened | {}",
                    e
                );
                Err(io::Error::other(e))
            }
        }
    }
}
