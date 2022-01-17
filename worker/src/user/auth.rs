use pbkdf2::password_hash::{Error, PasswordHash, PasswordVerifier};
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use worker::kv::KvStore;
use crate::user::user::User;
use crate::utils::kv::KvStoreWrapper;

#[derive(Deserialize, Serialize)]
pub struct AuthUser {
    username: String,
    password: String,
}

impl AuthUser {
    pub fn username(&self) -> String {
        self.username.clone()
    }
}

pub struct UserAuthentication {
    kv: KvStoreWrapper,
    user: AuthUser,
}

impl UserAuthentication {
    /// init new instance
    pub fn new(kv: KvStore, user: AuthUser) -> Self {
        // create new wrapper
        let kv = KvStoreWrapper::from(kv);

        Self {
            kv,
            user,
        }
    }

    /// verify the user
    pub async fn verify_user(&self) -> Result<(), Error> {
        // get user data from the kv
        match self.kv.get_base64::<AuthUser>(self.user.username().as_str()).await {
            Some(user_data) => {
                // get password as hash
                let hash = PasswordHash::new(&user_data.password)?;

                // verify the password
                match Pbkdf2.verify_password(self.user.password.as_bytes(), &hash) {
                    Ok(()) => Ok(()),
                    Err(error) => Err(error)
                }
            }
            None => Err(Error::Password)
        }
    }
}