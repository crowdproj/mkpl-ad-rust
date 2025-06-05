use crate::api_handler;
use api_common::ApiError;
use api_v1::models::{AdReadRequest, AdReadResponse};
use api_v1_map::{AdReadRequestMapperV1, AdReadResponseMapperV1};

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};
use serde_json;

pub async fn handle_read(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    let handler = api_handler!(
        AdReadRequest,  // Тип запроса
        AdReadResponse, // Тип ответа
        |ctx: &mut MkplAdCtx, req: &AdReadRequest| {
            // Маппинг запроса
            AdReadRequestMapperV1::from_api(ctx, req);
        },
        |ctx: &mut MkplAdCtx| {
            // Бизнес-логика
            ctx.ad_response = ctx.ad_request.clone();
            AdReadResponseMapperV1::to_api(ctx)
        }
    );

    handler(req).await
}
