use crate::api_handler;
use api_common::ApiError;
use api_v1::models::{AdSearchRequest, AdSearchResponse};
use api_v1_map::{AdSearchRequestMapperV1, AdSearchResponseMapperV1};

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};
use serde_json;

pub async fn handle_search(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    let handler = api_handler!(
        AdSearchRequest,  // Тип запроса
        AdSearchResponse, // Тип ответа
        |ctx: &mut MkplAdCtx, req: &AdSearchRequest| {
            // Маппинг запроса
            AdSearchRequestMapperV1::from_api(ctx, req);
        },
        |ctx: &mut MkplAdCtx| {
            // Бизнес-логика
            ctx.ad_response = ctx.ad_request.clone();
            AdSearchResponseMapperV1::to_api(ctx)
        }
    );

    handler(req).await
}
