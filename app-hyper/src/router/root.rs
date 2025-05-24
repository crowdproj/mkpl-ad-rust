use http_body_util::Full;
use hyper::{body::Bytes, Response};

pub async fn handle() -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::builder()
        .status(200)
        .body(Full::new(Bytes::from("Hello, World!")))
        .unwrap())
}
