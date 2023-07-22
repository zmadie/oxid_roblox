use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use reqwest::{header::HeaderMap, Client, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::{get_api_errors_from_response, RobloxResult};

lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
    static ref HEADERS: Arc<Mutex<HeaderMap>> = Arc::new(Mutex::new({
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "Roblox/WinInet".parse().unwrap());
        headers.insert("Referer", "www.roblox.com".parse().unwrap());
        headers
    }));
}

pub(crate) fn set_roblosecurity(roblosecurity: &str) {
    HEADERS.clone().lock().unwrap().insert(
        "Cookie",
        format!(".ROBLOSECURITY={};", roblosecurity)
            .parse()
            .unwrap(),
    );
}

pub(crate) async fn deserialize_body<T: DeserializeOwned>(response: Response) -> T {
    response.json::<T>().await.unwrap()
}

async fn request(verb: Method, url: String, body: Option<Value>) -> RobloxResult<Response> {
    let arc_ref = HEADERS.clone();
    let mut headers = arc_ref.lock().unwrap();

    let response = HTTP_CLIENT
        .request(verb.clone(), url.clone())
        .headers(headers.clone())
        .json(&body)
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => Ok(response),
        StatusCode::UNAUTHORIZED => panic!(
            "A valid .ROBLOSECURITY with sufficient permissions is required for this action."
        ),
        StatusCode::FORBIDDEN => {
            // Get the x-csrf-token here because get_api_errors_from_response consumes the response
            let x_csrf_token = response.headers().get("x-csrf-token").cloned();

            let errors = get_api_errors_from_response(response).await;
            // Some endpoints return 403 for domain logic errors, so only handle the x-csrf-token if this is a Token Validation Failed (code 0)
            if errors.iter().any(|error| error.code == 0) {
                headers.insert("x-csrf-token", x_csrf_token.unwrap().to_owned());

                Ok(HTTP_CLIENT
                    .request(verb, url)
                    .headers(headers.clone())
                    .json(&body)
                    .send()
                    .await
                    .unwrap())
            } else {
                Err(errors)
            }
        }
        _ => Err(get_api_errors_from_response(response).await),
    }
}

pub async fn get(url: String) -> RobloxResult<Response> {
    request(Method::GET, url, None).await
}

pub async fn delete(url: String) -> RobloxResult<Response> {
    request(Method::DELETE, url, None).await
}

pub async fn post(url: String, body: Value) -> RobloxResult<Response> {
    request(Method::POST, url, Some(body)).await
}

pub async fn patch(url: String, body: Value) -> RobloxResult<Response> {
    request(Method::PATCH, url, Some(body)).await
}
