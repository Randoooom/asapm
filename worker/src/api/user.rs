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

use worker::{Request, RouteContext, Response, Result};
use serde::{Deserialize, Serialize};
use crate::DataContext;

#[derive(Deserialize, Serialize)]
struct Data {
    data: String,
}

/// POST /user/data
///
/// update saved data
pub async fn push_data(mut request: Request, context: RouteContext<DataContext>) -> Result<Response> {
    // parse json
    match request.json::<Data>().await {
        Ok(data) => {
            // get the user
            let mut user = context.data().clone().unwrap();
            // get the kv
            let kv = context.kv("user")?;

            // update the data
            user.update_data(data.data);
            // save user
            user.save(kv).await?;

            Response::ok("Updated")
        },
        // invalid json
        Err(error) => Response::error(error.to_string(), 400)
    }
}

/// GET /user/data
///
/// get the saved data
pub async fn get_data(_: Request, context: RouteContext<DataContext>) -> Result<Response> {
    // get the user
    let user = context.data().clone().unwrap();

    // return as json
    Response::from_json(&Data  {
        data: user.data()
    })
}

/// DELETE /user
/// delete an user
pub async fn delete_user(_: Request, context: RouteContext<DataContext>) -> Result<Response> {
    // get the user
    let user = context.data().clone().unwrap();
    // get the kv
    let kv = context.kv("user")?;

    // delete the user
    kv.delete(user.uuid().as_str()).await?;

    Response::ok("Deleted")
}
