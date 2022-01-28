/*
 * MIT LICENSE
 *
 * Copyright (c) 2022 Randoooom
 *
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use jwt_simple::prelude::{Claims, Duration, HS512Key, MACLike};
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::Pbkdf2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::kv::{KvError, KvStore};

use crate::utils::kv::KvStoreWrapper;

///
/// struct for the decoded saved data in the kv
///
/// contains the saved data, a password for accessing the data
/// and an uuid for identification
///
#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    // identifier
    uuid: String,
    // custom password for the backup access
    password: String,
    // entire backup data (client side aes encrypted)
    data: String,
}

/// custom claims for the jwt
#[derive(Deserialize, Serialize)]
struct UserClaims {
    uuid: String,
}

/// response for the token
#[derive(Deserialize, Serialize)]
pub struct Token {
    token: String,
}

impl User {
    /// init new user
    ///
    /// will hash the password and save the created user in the kv too
    pub async fn create_new(kv: KvStore, password: String) -> Result<Self, KvError> {
        // hash the password
        let salt = SaltString::generate(&mut OsRng);
        let hash = Pbkdf2.hash_password(password.as_bytes(), &salt).unwrap();

        // generate uuid
        let uuid = Uuid::new_v4();

        // create user
        let user = Self {
            uuid: uuid.to_string(),
            password: hash.to_string(),
            data: "".to_string(),
        };
        // save into kv
        user.save(kv).await?;

        // return the user
        Ok(user)
    }

    /// create new user from login data
    pub async fn new_from_login(kv: KvStore, uuid: String, password: String) -> Result<Self, pbkdf2::password_hash::Error> {
        // init wrapper
        let wrapper = KvStoreWrapper::from(kv);
        // get the data
        match wrapper.get_base64::<Self>(uuid.as_str()).await {
            Some(user) => {
                // compare password and hash
                let hash = PasswordHash::new(user.password.as_str())?;
                match Pbkdf2.verify_password(password.as_bytes(), &hash) {
                    Ok(()) => Ok(user),
                    Err(error) => Err(error)
                }
            }
            // return err on None
            // we dont differ between not found and wrong password
            None => Err(pbkdf2::password_hash::Error::Password)
        }
    }

    /// get new from jwt
    pub async fn new_from_token(kv: KvStore, token: String, secret: String) -> Result<Self, ()> {
        // setup jwt
        let key = HS512Key::from_bytes(secret.as_bytes());

        // verify jwt
        match key.verify_token::<UserClaims>(token.as_str(), None) {
            Ok(data) => {
                // get uuid from claims
                let uuid = data.custom.uuid;
                let wrapper = KvStoreWrapper::from(kv);

                Ok(wrapper.get_base64::<Self>(uuid.as_str()).await.unwrap())
            },
            // unauthorized
            Err(_) => Err(())
        }
    }

    /// sign new jwt key
    pub fn sign_key(&self, secret: String) -> Token {
        // setup jwt
        let key = HS512Key::from_bytes(secret.as_bytes());
        // create custom claim data
        let claim_data = UserClaims {
            uuid: self.uuid.clone()
        };
        // init claims
        let claims = Claims::with_custom_claims(claim_data, Duration::from_mins(30));

        // return generated token
        Token {
            token: key.authenticate(claims).unwrap()
        }
    }

    /// save the user
    pub async fn save(&self, kv: KvStore) -> Result<(), KvError> {
        // get wrapper
        let mut wrapper = KvStoreWrapper::from(kv);
        // save
        wrapper.put_base64(self.uuid.as_str(), &self).await.execute().await
    }

    /// update the user data
    pub fn update_data(&mut self, data: String) {
        self.data = data;
    }

    /// get the data
    pub fn data(&self) -> String {
        self.data.clone()
    }

    /// get the data
    pub fn uuid(&self) -> String {
        self.uuid.clone()
    }
}
