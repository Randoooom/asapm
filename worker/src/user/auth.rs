use jwt_simple::prelude::{Claims, Duration, HS512Key, MACLike, JWTClaims};
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

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    token: String,
}

impl AuthUser {
    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
        }
    }
}

pub struct UserAuthentication {
    kv: KvStoreWrapper,
    user: AuthUser,
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    username: String,
}

impl UserClaims {
    pub fn username(&self) -> String {
        self.username.clone()
    }
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
    pub async fn verify_login(&self) -> Result<(), Error> {
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

    /// generate new jwt as auth
    pub fn generate_token(&self, secret: &String) -> TokenResponse {
        // setup jwt
        let key = HS512Key::from_bytes(secret.as_bytes());
        // create custom claim data
        let claim_data = UserClaims {
            username: self.user.username()
        };
        // init claims
        let claims = Claims::with_custom_claims(claim_data, Duration::from_mins(15));

        // return generated token
        TokenResponse {
            token: key.authenticate(claims).unwrap()
        }
    }

    /// verify a token
    pub fn verify_token(token: &String, secret: &String) -> Result<JWTClaims<UserClaims>, jwt_simple::Error> {
        // split Bearer off
        let token = token.split_whitespace().last().unwrap();
        // setup jwt
        let key = HS512Key::from_bytes(secret.as_bytes());

        // verify
        key.verify_token::<UserClaims>(token, None)
    }
}