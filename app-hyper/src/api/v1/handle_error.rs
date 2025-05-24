// error_handler.rs
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode};

pub fn handle_error(error: ApiError) -> Response<Full<Bytes>> {
    let status = match error.0.as_str() {
        "Invalid JSON" => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(format!(
            r#"{{"error":"{}"}}"#,
            error.0
        ))))
        .unwrap()
}
