use serde::{Deserialize, Serialize};
use worker::{Request, Response, RouteContext};
use crate::DataContext;

#[derive(Deserialize, Serialize)]
struct Password {
    password: String
}

/// return all passwords as json
pub async fn get_passwords(_: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // get the user
    let user = context.data().clone().unwrap();

    // return passwords
    match user.get_passwords().await {
        Some(passwords) => Response::from_json(&passwords),
        None => Response::error("Not found", 400)
    }
}

/// insert new password
pub async fn post_password(mut request: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // parse json
    match request.json::<Password>().await {
        Ok(password) => {
            // get the user
            let mut user = context.data().clone().unwrap();
            // insert the password

            match user.save_password(password.password).await {
                Ok(()) => Response::ok("Created"),
                Err(_) => Response::error("KvError while inserting the password", 500)
            }
        },
        Err(error) => Response::error(format!("Err while parsing json: {}", error), 400)
    }
}
