use super::*;
use http_body_util::Full;
use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use hyper::{body::Bytes, Response, StatusCode};
use serde::Serialize;

#[derive(Debug, Default)]
pub struct SwaggerResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Option<Bytes>,
    content_type: Option<String>,
}

impl SwaggerResponse {
    // Конструкторы для стандартных статусов
    pub fn ok() -> Self {
        Self::new(StatusCode::OK)
    }

    pub fn created() -> Self {
        Self::new(StatusCode::CREATED)
    }

    pub fn bad_request() -> Self {
        Self::new(StatusCode::BAD_REQUEST)
    }

    pub fn internal_error() -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR)
    }

    // Основной конструктор
    pub fn new(status: StatusCode) -> Self {
        SwaggerResponse {
            status,
            headers: HeaderMap::new(),
            body: None,
            content_type: None,
        }
    }

    // Добавление тела ответа
    pub fn with_body<T: Serialize>(mut self, body: &T) -> Result<Self, ApiError> {
        let json = serde_json::to_vec(body).map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(), // Преобразуем ошибку в строку
            )
        })?;

        self.body = Some(Bytes::from(json));
        self.content_type = Some("application/json".to_string());
        Ok(self)
    }

    // Добавление сырых данных
    pub fn with_raw_body(mut self, body: impl Into<Bytes>) -> Self {
        self.body = Some(body.into());
        self.content_type = Some("text/plain".to_string());
        self
    }

    pub fn with_json_body<T: Serialize>(mut self, body: &T) -> Result<Self, ApiError> {
        let json = serde_json::to_vec(body).map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(), // Явное преобразование ошибки в String
            )
        })?;

        self.body = Some(json.into());
        self.content_type = Some("application/json".to_string());
        Ok(self)
    }

    pub fn content_type(mut self, ct: &str) -> Self {
        self.headers.insert(
            hyper::header::CONTENT_TYPE,
            // Исправляем обработку ошибок парсинга
            ct.parse()
                .unwrap_or_else(|_| HeaderValue::from_static("text/plain")),
        );
        self
    }

    // Добавление заголовков
    pub fn with_header(mut self, name: &str, value: &str) -> Result<Self, ApiError> {
        let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(), // Преобразуем ошибку в строку
            )
        })?;

        let header_value = HeaderValue::from_str(value).map_err(|e| {
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                e.to_string(), // Преобразуем ошибку в строку
            )
        })?;

        self.headers.insert(header_name, header_value);
        Ok(self)
    }

    // Добавление span ID из контекста
    pub fn with_span_id(mut self, context: &impl Has<XSpanIdString>) -> Self {
        if let Ok(header_value) = HeaderValue::from_str(&context.get().0) {
            self.headers.insert("x-span-id", header_value);
        }
        self
    }

    // Преобразование в Hyper Response
    pub fn into_hyper_response(self) -> Response<Full<Bytes>> {
        let mut response = Response::builder().status(self.status);

        // Устанавливаем заголовки
        for (name, value) in self.headers.iter() {
            response = response.header(name, value);
        }

        // Устанавливаем Content-Type
        if let Some(ct) = self.content_type {
            response = response.header("Content-Type", ct);
        }

        // Тело ответа
        response
            .body(Full::new(self.body.unwrap_or_default()))
            .unwrap_or_else(|_| Response::new(Full::new(Bytes::from("Internal Server Error"))))
    }

    pub fn add_header(&mut self, name: &str, value: &str) -> Result<(), ApiError> {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let header_value = HeaderValue::from_str(value)
            .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        self.headers.insert(header_name, header_value);
        Ok(())
    }

    // Добавляем метод для установки тела
    pub fn set_body(&mut self, body: Bytes) {
        self.body = Some(body);
    }
}

impl From<ApiError> for SwaggerResponse {
    fn from(error: ApiError) -> Self {
        SwaggerResponse::new(error.status_code())
            .with_raw_body(error.to_string())
            .content_type("text/plain")
    }
}
