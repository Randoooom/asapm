use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::kv::{KvError, KvStore};
use crate::utils::kv::KvStoreWrapper;

#[derive(Clone)]
pub struct User {
    username: String,
    user_kv: KvStoreWrapper,
    password_kv: KvStoreWrapper,
}

/// all passwords are JSON.stringify() and aes encrypted by the client for the best security
/// identification of the passwords are based on the uuid
/// the rust struct for the 'data' would look like this:
///
/// pub struct PasswordData {
///     login: String,
///     password: String,
///     url: Option<String>,
///     description: Option<String>
/// }
///
#[derive(Deserialize, Serialize)]
pub struct Password {
    // encrypted data
    data: String,
    // uuid as identifier
    uuid: String,
}

impl Password {
    pub fn new(data: String) -> Self {
        Self {
            data,
            uuid: Uuid::new_v4().to_string(),
        }
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }
}

impl User {
    /// create new instance from the username and storage kv's
    pub fn new(username: String, password_kv: KvStore, user_kv: KvStore) -> Self {
        Self {
            username,
            password_kv: KvStoreWrapper::from(password_kv),
            user_kv: KvStoreWrapper::from(user_kv),
        }
    }

    /// init new user
    pub async fn init_new_user(username: &String, hash: &String, kv: KvStore) -> Result<(), KvError> {
        // save the hash into kv
        kv.put(username, hash)?.execute().await
    }

    /// get all passwords from the account
    pub async fn get_passwords(&self) -> Option<Vec<Password>> {
        // get the passwords
        self.password_kv.get_base64::<Vec<Password>>(self.username.as_str()).await
    }

    /// delete an existing password
    pub async fn delete_password(&mut self, uuid: String) -> Result<(), KvError> {
        // get current passwords
        match self.get_passwords().await {
            Some(passwords) => {
                // filter
                let passwords = passwords.iter()
                    .filter(|password| !password.uuid.eq(&uuid))
                    .collect::<Vec<&Password>>();

                // save filtered
                self.password_kv.put_base64(self.username.as_str(), &passwords).await.execute().await
            }
            None => Err(KvError::InvalidKvStore("".to_string()))
        }
    }

    /// insert a new password
    pub async fn create_password(&mut self, data: String) -> Result<(), KvError> {
        // get passwords
        let mut passwords = match self.get_passwords().await {
            Some(passwords) => passwords,
            // create new vec
            None => Vec::new()
        };
        // push the new password
        let password = Password::new(data);
        passwords.push(password);

        // save updated into kv
        self.password_kv.put_base64(self.username.as_str(), &passwords).await.execute().await
    }

    /// edit an existing password
    pub async fn edit_password(&mut self, new: Password) -> Result<(), KvError> {
        // get all passwords
        match self.get_passwords().await {
            Some(passwords) => {
                // iter passwords
                let passwords = passwords.iter()
                    // replace the one
                    .map(|password| {
                        match password.uuid.eq(&new.uuid) {
                            true => &new,
                            false => password
                        }
                    })
                    .collect::<Vec<&Password>>();


                // save into kv
                self.password_kv.put_base64(self.username.as_str(), &passwords).await.execute().await
            },
            None => Err(KvError::InvalidMetadata("Passwords not found".to_string()))
        }
    }

    /// delete entire data of the user
    pub async fn delete_user(&mut self) -> Result<(), KvError> {
        // delete all passwords
        self.password_kv.delete(self.username.as_str()).await?;
        // delete login
        self.user_kv.delete(self.username.as_str()).await
    }
}