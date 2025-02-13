// @dfns-sdk-rs/src/utils/fetch.rs

use crate::{
    error::{DfnsError, PolicyPendingError},
    models::generic::DfnsBaseApiOptions,
    utils::nonce::generate_nonce,
};
use reqwest::{Client, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

const DEFAULT_DFNS_BASE_URL: &str = "https://api.dfns.io";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl From<HttpMethod> for Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => Method::GET,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
            HttpMethod::DELETE => Method::DELETE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchOptions<T> {
    pub method: HttpMethod,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<Value>,
    pub api_options: T,
}

pub type FetchResult = Result<Response, DfnsError>;

pub trait Fetch {
    #[allow(async_fn_in_trait)]
    async fn execute(&self, url: &str, options: FetchOptions<DfnsBaseApiOptions>) -> FetchResult;
}

pub struct DfnsFetch {
    client: Client,
}

impl Clone for DfnsFetch {
    fn clone(&self) -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl std::fmt::Debug for DfnsFetch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DfnsFetch")
            .field("client", &"Client")
            .finish()
    }
}

impl PartialEq for DfnsFetch {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl DfnsFetch {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    #[allow(async_fn_in_trait)]
    #[allow(dead_code)]
    async fn handle_response(&self, response: Response) -> FetchResult {
        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let body: Value = response.json().await.unwrap_or_default();

            if status == StatusCode::ACCEPTED {
                Err(PolicyPendingError::new(Some(body)).into())
            } else {
                let message = body
                    .get("error")
                    .and_then(|e| e.get("message"))
                    .or_else(|| body.get("message"))
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();

                Err(DfnsError::new(status.as_u16(), message, Some(body)))
            }
        }
    }
}

impl Fetch for DfnsFetch {
    async fn execute(
        &self,
        resource: &str,
        options: FetchOptions<DfnsBaseApiOptions>,
    ) -> FetchResult {
        let base_url = options
            .api_options
            .base_url
            .unwrap_or_else(|| DEFAULT_DFNS_BASE_URL.to_string());
        let url = Url::parse(&base_url)?.join(resource)?;

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("x-dfns-appid", options.api_options.app_id.parse()?);
        headers.insert("x-dfns-nonce", generate_nonce().parse()?);
        headers.insert("x-dfns-sdk-version", VERSION.parse()?);

        if let Some(app_secret) = options.api_options.app_secret {
            headers.insert("x-dfns-appsecret", app_secret.parse()?);
        }

        if let Some(auth_token) = options.api_options.auth_token {
            headers.insert("authorization", format!("Bearer {}", auth_token).parse()?);
        }

        if let Some(custom_headers) = options.headers {
            for (key, value) in custom_headers {
                let key_str: &'static str = Box::leak(key.into_boxed_str());
                headers.insert(key_str, value.parse()?);
            }
        }

        let mut request = self
            .client
            .request(options.method.into(), url)
            .headers(headers);

        if let Some(body) = options.body {
            request = request
                .header("content-type", "application/json")
                .json(&body);
        }

        request.send().await.map_err(|e| e.into())
    }
}

pub async fn simple_fetch<T: Serialize + DeserializeOwned>(
    resource: &str,
    options: FetchOptions<DfnsBaseApiOptions>,
) -> Result<T, DfnsError> {
    let fetch = DfnsFetch::new();
    let response = fetch.execute(resource, options).await?;
    response.json::<T>().await.map_err(|e| e.into())
}
