use api_common::ApiError;
use bytes::Bytes;
use http_body_util::Full;
use hyper::Response;

pub fn handle_error(error: ApiError) -> Response<Full<Bytes>> {
    error.into_response()
}
