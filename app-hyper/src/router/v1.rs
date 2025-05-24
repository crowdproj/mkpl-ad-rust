// router/v1.rs
use super::handle_error;
use crate::api::v1::create;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};

pub async fn route(req: Request<hyper::body::Incoming>) -> Response<Full<Bytes>> {
    match req.uri().path() {
        "/v1/create" => create::handle(req).await.unwrap_or_else(handle_error),
        // ... другие обработчики
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::new()))
            .unwrap(),
    }
}
