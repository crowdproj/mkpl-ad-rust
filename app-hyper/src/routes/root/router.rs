// router/v1.rs
use super::handle_error;
use super::handle_root;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};

pub async fn route(req: Request<hyper::body::Incoming>) -> Response<Full<Bytes>> {
    match req.uri().path() {
        "/" => handle_root(req).await.unwrap_or_else(handle_error),
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::new()))
            .unwrap(),
    }
}
