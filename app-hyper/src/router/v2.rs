use crate::api::v2;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};

pub async fn route(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match req.uri().path() {
        "/v2/create" => v2::create::handle(req).await,
        // "/v2/read" => v2::read::handle(req).await,
        // "/v2/update" => v2::update::handle(req).await,
        // "/v2/delete" => v2::delete::handle(req).await,
        // "/v2/search" => v2::search::handle(req).await,
        _ => Ok(Response::builder()
            .status(404)
            .body(Full::new(Bytes::new()))
            .unwrap()),
    }
}
