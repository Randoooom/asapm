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

use serde::{Deserialize, Serialize};
use worker::{Request, Response, RouteContext};
use crate::DataContext;
use crate::user::user::Password;

#[derive(Deserialize, Serialize)]
struct PasswordData {
    data: String,
}

#[derive(Deserialize, Serialize)]
struct PasswordIdentifier {
    uuid: String,
}

/// GET /user/password
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

/// POST /user/password
/// insert new password
pub async fn post_password(mut request: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // parse json
    match request.json::<PasswordData>().await {
        Ok(password) => {
            // get the user
            let mut user = context.data().clone().unwrap();
            // insert the password

            match user.create_password(password.data).await {
                Ok(()) => Response::ok("Created"),
                Err(_) => Response::error("KvError while inserting the password", 500)
            }
        }
        Err(error) => Response::error(format!("Err while parsing json: {}", error), 400)
    }
}

/// PUT /user/password
pub async fn edit_password(mut request: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // parse json
    match request.json::<Password>().await {
        Ok(password) => {
            // get the user
            let mut user = context.data().clone().unwrap();

            // update the password
            match user.edit_password(password).await {
                Ok(()) => Response::ok("Updated"),
                Err(_) => Response::error("KvError while updating the password", 500)
            }
        }
        Err(error) => Response::error(format!("Err while parsing json: {}", error), 400)
    }
}

/// DELETE /user/password
pub async fn delete_password(mut request: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // parse json
    match request.json::<PasswordIdentifier>().await {
        Ok(password) => {
            // get the user
            let mut user = context.data().clone().unwrap();

            // delete the password
            match user.delete_password(password.uuid).await {
                Ok(()) => Response::ok("Deleted"),
                Err(_) => Response::error("KvError while deleting the password", 500)
            }
        }
        Err(error) => Response::error(format!("Err while parsing json: {}", error), 400)
    }
}

/// DELETE /user
pub async fn delete_user(_: Request, context: RouteContext<DataContext>) -> worker::Result<Response> {
    // get the user
    let mut user = context.data().clone().unwrap();

    // delete the password
    match user.delete_user().await {
        Ok(()) => Response::ok("Deleted"),
        Err(_) => Response::error("KvError while deleting the user", 500)
    }
}
