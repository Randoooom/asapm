#![feature(explicit_generic_args_with_impl_trait)]

use worker::*;
use worker::kv::KvStore;
use crate::user::auth::{UserAuthentication};
use crate::user::user::User;

mod utils;
mod user;
mod api;

pub type DataContext = Option<User>;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::console::set_panic_hook();

    // handle preflights
    if &req.method() == &Method::Options {
        let mut headers = Headers::new();
        headers.append("Access-Control-Allow-Methods", "POST, GET, DELETE, OPTIONS, PUT")?;
        headers.append("Access-Control-Max-Age", "86400")?;
        headers.append("Access-Control-Allow-Origin", "*")?;
        headers.append("Access-Control-Allow-Headers", "*")?;

        return Ok(Response::empty()?.with_headers(headers).with_status(204));
    }

    // check for auth required
    let requires_auth = match req.url()?.path().to_lowercase().as_str() {
        "/auth/login" => false,
        "/auth/signup" => false,
        _ => true
    };

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router: Router<DataContext> = {
        match requires_auth {
            true => {
                // get token
                match req.headers().get("Authorization")? {
                    Some(token) => {
                        // verify the token
                        match UserAuthentication::verify_token(&token, &env.var("SECRET")?.to_string()) {
                            Ok(claims) => {
                                // get kv's
                                let password_kv: KvStore = env.kv("password")?;
                                let account_kv: KvStore = env.kv("user")?;

                                // build user
                                let user = User::new(claims.custom.username(), password_kv, account_kv);
                                // create router
                                Router::with_data(Some(user))
                            }
                            Err(_) => return Response::error("Unauthorized", 401)
                        }
                    }
                    None => return Response::error("Unauthorized", 401)
                }
            }
            false => Router::with_data(None)
        }
    };

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    match router
        // auth
        .post_async("/auth/signup", api::auth::post_signup)
        .post_async("/auth/login", api::auth::post_login)

        // user
        .delete_async("/user", api::user::delete_user)
        .get_async("/user/password", api::user::get_passwords)
        .post_async("/user/password", api::user::post_password)
        .put_async("/user/password", api::user::edit_password)
        .delete_async("/user/password", api::user::delete_password)

        .run(req, env)
        .await {
        Ok(response) => {
            // append cors headers
            let mut headers = response.headers().clone();
            headers.append("Access-Control-Allow-Methods", "POST, GET, DELETE, OPTIONS, PUT")?;
            headers.append("Access-Control-Max-Age", "86400")?;
            headers.append("Access-Control-Allow-Origin", "*")?;
            headers.append("Access-Control-Allow-Headers", "*")?;

            // return response
            Ok(response.with_headers(headers))
        }
        Err(error) => Err(error),
    }
}