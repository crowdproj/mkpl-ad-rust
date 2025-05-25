use http_body_util::Full;
use hyper::{body::Bytes, Request, Response};

pub async fn handle_create(
    _req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::builder()
        .status(200)
        .body(Full::new(Bytes::from("v1 create")))
        .unwrap())
}
