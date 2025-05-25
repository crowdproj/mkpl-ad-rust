mod root;
mod v1;
// mod v2;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response};

pub async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let path = req.uri().path();

    Ok(match path {
        "/" => root::route(req).await,
        path if path.starts_with("/v1/") => v1::route(req).await,
        // path if path.starts_with("/v2/") => v2::route(req).await,
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::new()))
            .unwrap(),
    })
}
