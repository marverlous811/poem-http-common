use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};

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
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorResponse>),
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorResponse>),
}
