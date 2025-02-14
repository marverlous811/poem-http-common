use poem::{http::StatusCode, Error};
use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Object, Clone)]
pub struct StatusResponse {
    pub status: bool,
}

#[derive(Serialize, Deserialize, Debug, Object, Clone)]
pub struct RedirectResponse {
    pub url: String,
}

pub fn build_error_response(code: StatusCode, msg: &str) -> Error {
    let err: HttpApiResponse<StatusResponse> = match code {
        StatusCode::BAD_REQUEST => HttpApiResponse::BadRequest(ErrorResponse::from_str(msg).to_json()),
        StatusCode::UNAUTHORIZED => HttpApiResponse::Unauthorized(ErrorResponse::from_str(msg).to_json()),
        StatusCode::FORBIDDEN => HttpApiResponse::Forbidden(ErrorResponse::from_str(msg).to_json()),
        StatusCode::TOO_MANY_REQUESTS => HttpApiResponse::TooManyRequest(ErrorResponse::from_str(msg).to_json()),
        StatusCode::NOT_FOUND => HttpApiResponse::NotFound(ErrorResponse::from_str(msg).to_json()),
        StatusCode::INTERNAL_SERVER_ERROR => {
            HttpApiResponse::InternalServerError(ErrorResponse::from_str(msg).to_json())
        }
        StatusCode::BAD_GATEWAY => HttpApiResponse::BadGateway(ErrorResponse::from_str(msg).to_json()),
        StatusCode::SERVICE_UNAVAILABLE => HttpApiResponse::ServiceUnavailable(ErrorResponse::from_str(msg).to_json()),
        _ => HttpApiResponse::InternalServerError(ErrorResponse::from_str(msg).to_json()),
    };

    Error::from(err)
}

#[derive(Serialize, Deserialize, Debug, Object, Clone)]
pub struct ErrorResponse {
    pub msg: String,
}

impl ErrorResponse {
    pub fn from_str(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }

    pub fn to_json(&self) -> Json<Self> {
        Json(self.clone())
    }
}

#[derive(ApiResponse)]
pub enum HttpApiResponse<T>
where
    T: ParseFromJSON + ToJSON + Send + Sync,
{
    #[oai(status = 200)]
    Ok(Json<T>),
    #[oai(status = 201)]
    Created(Json<T>),
    #[oai(status = 301)]
    MovedPermanently(Json<RedirectResponse>),
    #[oai(status = 302)]
    Found(Json<RedirectResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 429)]
    TooManyRequest(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorResponse>),
    #[oai(status = 502)]
    BadGateway(Json<ErrorResponse>),
    #[oai(status = 503)]
    ServiceUnavailable(Json<ErrorResponse>),
}
