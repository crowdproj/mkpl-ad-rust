use crate::api_handler;
use api_common::ApiError;
use api_v1::models::{AdCreateRequest, AdCreateResponse};
use api_v1_map::{AdCreateRequestMapperV1, AdCreateResponseMapperV1};

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};
use serde_json;

pub async fn handle_create(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    let handler = api_handler!(
        AdCreateRequest,  // Тип запроса
        AdCreateResponse, // Тип ответа
        |ctx: &mut MkplAdCtx, req: &AdCreateRequest| {
            // Маппинг запроса
            AdCreateRequestMapperV1::from_api(ctx, req);
        },
        |ctx: &mut MkplAdCtx| {
            // Бизнес-логика
            ctx.ad_response = ctx.ad_request.clone();
            AdCreateResponseMapperV1::to_api(ctx)
        }
    );

    handler(req).await
}
