use super::handle_create;
use super::handle_error;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};

pub async fn route(req: Request<hyper::body::Incoming>) -> Response<Full<Bytes>> {
    match req.uri().path() {
        "/v2/create" => handle_create(req).await.unwrap_or_else(handle_error),
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::new()))
            .unwrap(),
    }
}
