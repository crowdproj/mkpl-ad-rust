use crate::models::{AdCreateRequest, AdCreateResponse, ApiError};
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};
use serde_json;

pub async fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    // Парсинг JSON из тела запроса
    let body_bytes = hyper::body::to_bytes(req.into_body())
        .await
        .map_err(|_| ApiError("Failed to read body".into()))?;

    let request: AdCreateRequest =
        serde_json::from_slice(&body_bytes).map_err(|_| ApiError("Invalid JSON".into()))?;

    // Логика обработки (пример)
    let response = AdCreateResponse {
        id: "123".to_string(),
        status: "created".to_string(),
    };

    // Сериализация ответа в JSON
    let json =
        serde_json::to_string(&response).map_err(|_| ApiError("Serialization error".into()))?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(json)))
        .unwrap())
}
