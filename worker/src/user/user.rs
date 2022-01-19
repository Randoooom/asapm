use serde::{Deserialize, Serialize};
use worker::kv::{KvError, KvStore};
use crate::utils::kv::KvStoreWrapper;

pub struct User {
    username: String,
    account_kv: KvStoreWrapper,
    password_kv: KvStoreWrapper,
}

impl User {
    /// create new instance from the username and storage kv's
    pub fn new(username: String, password_kv: KvStore, account_kv: KvStore) -> Self {
        Self {
            username,
            password_kv: KvStoreWrapper::from(password_kv),
            account_kv: KvStoreWrapper::from(account_kv),
        }
    }

    /// get all passwords from the account
    /// all passwords are JSON.stringify() and aes encrypted by the client for the best security
    /// identification of the passwords are based on the full ciphertext
    /// the rust struct would look like this:
    ///
    /// pub struct Password {
    ///     login: String,
    ///     password: String,
    ///     url: Option<String>,
    ///     description: Option<String>
    /// }
    ///
    pub async fn get_passwords(&self) -> Option<Vec<String>> {
        // get the passwords
        self.password_kv.get_base64::<Vec<String>>(self.username.as_str()).await
    }

    /// delete an existing password
    pub async fn delete_password(&mut self, encrypted: String) -> Result<(), KvError> {
        // get current passwords
        match self.get_passwords().await {
            Some(passwords) => {
                // filter
                let passwords = passwords.iter()
                    .filter(|password| password.eq(&&encrypted))
                    .collect::<Vec<&String>>();

                // save filtered
                self.password_kv.put_base64(self.username.as_str(), &passwords).await.execute().await
            }
            None => Err(KvError::InvalidKvStore("".to_string()))
        }
    }

    /// insert a new password
    pub async fn save_password(&mut self, encrypted: String) -> Result<(), KvError> {
        // get passwords
        let mut passwords = match self.get_passwords().await {
            Some(passwords) => passwords,
            // create new vec
            None => Vec::new()
        };
        // push the new password
        passwords.push(encrypted);

        // save updated into kv
        self.password_kv.put_base64(self.username.as_str(), &passwords).await.execute().await
    }

    /// delete entire data of the user
    pub async fn delete_user(&mut self) -> Result<(), KvError> {
        // delete all passwords
        self.password_kv.delete(self.username.as_str()).await?;
        // delete login
        self.account_kv.delete(self.username.as_str()).await
    }
}