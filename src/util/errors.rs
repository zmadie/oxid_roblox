use reqwest::Response;
use serde::Deserialize;

use super::{api_helper, responses::ErrorResponse};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    pub user_facing_message: Option<String>,
}

pub(crate) async fn get_api_errors_from_response(response: Response) -> Vec<ApiError> {
    api_helper::deserialize_body::<ErrorResponse>(response)
        .await
        .errors
}

pub type RobloxResult<T> = Result<T, Vec<ApiError>>;
