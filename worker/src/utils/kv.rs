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

use serde::de::DeserializeOwned;
use serde::Serialize;
use worker::kv::{KvError, KvStore, KvValue, PutOptionsBuilder, ToRawKvValue};

#[derive(Clone)]
pub struct KvStoreWrapper {
    kv: KvStore,
}

impl KvStoreWrapper {
    /// simple put
    pub async fn put<T: ToRawKvValue>(&self, name: impl Into<&str>, value: T) -> PutOptionsBuilder {
        self.kv.put(name.into(), value).unwrap()
    }

    /// simple get
    pub async fn get(&self, name: impl Into<&str>) -> Option<KvValue> {
        self.kv.get(name.into()).await.unwrap()
    }

    /// simple delete
    pub async fn delete(&self, name: impl Into<&str>) -> Result<(), KvError> {
        self.kv.delete(name.into()).await
    }

    /// encode a object to base64 string
    pub fn encode_base64<T: Serialize>(value: &T) -> String {
        // stringify
        let serialized = serde_json::to_string(value).unwrap();
        // encode
        base64::encode(serialized.as_bytes())
    }

    /// put value as base64
    pub async fn put_base64<T: Serialize>(&mut self, name: impl Into<&str>, value: &T) -> PutOptionsBuilder {
        // encode
        let encoded = Self::encode_base64(value);

        // put value
        self.put(name, encoded).await
    }

    /// get base64 encoded struct
    pub async fn get_base64<T: DeserializeOwned>(&self, name: impl Into<&str>) -> Option<T> {
        // get raw
        match self.get(name.into()).await {
            None => None,
            Some(raw) => {
                // decode
                let decoded = base64::decode(raw.as_bytes()).unwrap();
                let decoded = String::from_utf8(decoded).unwrap();

                // parse
                Some(serde_json::from_str(decoded.as_str()).unwrap())
            }
        }
    }
}

impl From<KvStore> for KvStoreWrapper {
    fn from(kv: KvStore) -> Self {
        Self {
            kv
        }
    }
}