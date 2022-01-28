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

#![feature(explicit_generic_args_with_impl_trait)]

extern crate core;

mod utils;
mod user;
mod api;

use worker::*;
use worker::kv::KvStore;
use user::User;

pub type DataContext = Option<User>;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::console::set_panic_hook();

    // handle preflights
    if &req.method() == &Method::Options {
        // set cors headers
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
                        // get the kv
                        let kv: KvStore = env.kv("user")?;
                        // verify the token
                        match User::new_from_token(kv, token, env.var("SECRET")?.to_string()).await {
                            Ok(user) => Router::with_data(Some(user)),
                            Err(()) => return Response::error("Unauthorized", 401)
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
        .post_async("/auth/login", api::auth::post_auth_login)

        // user
        .delete_async("/user", api::user::delete_user)
        .get_async("/user/data", api::user::get_data)
        .post_async("/user/data", api::user::push_data)

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
