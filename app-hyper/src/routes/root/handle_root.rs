use api_common::ApiError;
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};

pub async fn handle_root(_req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    Ok(Response::builder()
        .status(200)
        .body(Full::new(Bytes::from("Hello, World!")))
        .unwrap())
}
