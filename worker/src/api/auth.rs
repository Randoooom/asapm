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

use worker::{Request, RouteContext, Result, Response};
use serde::{Deserialize, Serialize};
use crate::{DataContext, User};

#[derive(Deserialize, Serialize)]
struct Login {
    uuid: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct SignUp {
    password: String,
}

/// POST /auth/login
///
/// start new auth session
pub async fn post_auth_login(mut request: Request, context: RouteContext<DataContext>) -> Result<Response> {
    // parse json
    match request.json::<Login>().await {
        Ok(data) => {
            // get the kv
            let kv = context.kv("user")?;
            // verify login
            match User::new_from_login(kv, data.uuid, data.password).await {
                // return jwt key
                Ok(user) => Response::from_json(&user.sign_key(context.var("SECRET").unwrap().to_string())),
                // return 401
                Err(_) => Response::error("Unauthorized", 401)
            }
        }
        // invalid json
        Err(error) => Response::error(error.to_string(), 400)
    }
}

/// POST /auth/signup
///
/// create new backup user account
pub async fn post_signup(mut request: Request, context: RouteContext<DataContext>) -> Result<Response> {
    // parse json
    match request.json::<SignUp>().await {
        Ok(data) => {
            // get the kv
            let kv = context.kv("user")?;
            // create the new user
            match User::create_new(kv, data.password).await {
                Ok(user) => Response::from_json(&user),
                Err(_) => Response::error("KvError while creating user", 500)
            }
        }
        // invalid json
        Err(error) => Response::error(error.to_string(), 400)
    }
}
