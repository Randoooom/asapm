use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHasher, SaltString};
use pbkdf2::Pbkdf2;
use worker::{Request, Response, RouteContext};
use serde::{Deserialize, Serialize};
use crate::DataContext;
use crate::user::auth::{AuthUser, UserAuthentication};

#[derive(Deserialize, Serialize)]
struct SignUp {
    username: String,
    password: String
}

/// POST /auth/signup
/// signup as a new user
/// will init the password kv
pub async fn post_signup(mut request: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // parse request body as json
    match request.json::<SignUp>().await {
        Ok(data) => {
            // get user kv
            let kv = context.kv("user").unwrap();
            // search for the username in the kv
            if kv.get(&data.username).await?.is_some() {
                return Response::error("Invalid username", 403);
            }

            // gen salt
            let salt = SaltString::generate(&mut OsRng);
            // hash password
            let hash = Pbkdf2.hash_password(&data.password.as_bytes(), &salt).unwrap();

            // save into kv
            kv.put(&data.username, hash.to_string()).unwrap().execute().await.unwrap();

            // return 200 OK
            Response::ok("Created")
        }
        Err(error) => Response::error(format!("Err while parsing json: {}", error), 400)
    }
}

/// POST /auth/login
/// start new jwt session
pub async fn post_login(mut request: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // parse body as json
    match request.json::<AuthUser>().await {
        Ok(data) => {
            // get auth kv
            let kv = context.kv("user").unwrap();
            // create auth obj
            let auth = UserAuthentication::new(kv, data);

            // verify the login
            match auth.verify_login().await {
                Ok(()) => {
                    // sign new jwt
                    let secret = context.var("SECRET").unwrap();
                    let token = auth.generate_token(&secret.to_string());

                    // return as json
                    Response::from_json(&token)
                },
                Err(_) => Response::error("Unauthorized", 401)
            }
        }
        Err(error) => Response::error(format!("Err while parsing json: {}", error), 400)
    }
}
