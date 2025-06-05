use crate::api_handler;
use api_common::ApiError;
use api_v1::models::{AdUpdateRequest, AdUpdateResponse};
use api_v1_map::{AdUpdateRequestMapperV1, AdUpdateResponseMapperV1};

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};
use serde_json;

pub async fn handle_update(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    let handler = api_handler!(
        AdUpdateRequest,  // Тип запроса
        AdUpdateResponse, // Тип ответа
        |ctx: &mut MkplAdCtx, req: &AdUpdateRequest| {
            // Маппинг запроса
            AdUpdateRequestMapperV1::from_api(ctx, req);
        },
        |ctx: &mut MkplAdCtx| {
            // Бизнес-логика
            ctx.ad_response = ctx.ad_request.clone();
            AdUpdateResponseMapperV1::to_api(ctx)
        }
    );

    handler(req).await
}
