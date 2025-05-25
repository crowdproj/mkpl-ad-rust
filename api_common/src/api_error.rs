use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Hyper processing error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("HTTP processing error: {0}")]
    HyperHttp(#[from] hyper::http::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Invalid header: {0}")]
    InvalidHeader(String),

    #[error("Custom error ({code}): {message}")]
    Custom { code: StatusCode, message: String },

    #[error("Internal server error")]
    Internal,
}

impl ApiError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        ApiError::Custom {
            code,
            message: message.into(),
        }
    }

    pub fn into_response(&self) -> Response<Full<Bytes>> {
        let body = self.to_string();
        Response::builder()
            .status(self.status_code())
            .body(Full::new(Bytes::from(body)))
            .unwrap()
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Hyper(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::HyperHttp(_) => StatusCode::BAD_REQUEST,
            ApiError::Serde(_) => StatusCode::BAD_REQUEST,
            ApiError::InvalidHeader(_) => StatusCode::BAD_REQUEST,
            ApiError::Custom { code, .. } => *code,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
