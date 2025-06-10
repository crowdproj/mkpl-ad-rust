use crate::api_handler;
use api_common::ApiError;
use api_v1::models::{AdOffersRequest, AdOffersResponse};
use api_v1_map::{AdOffersRequestMapperV1, AdOffersResponseMapperV1};

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};
use serde_json;

pub async fn handle_offers(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    let handler = api_handler!(
        AdOffersRequest,  // Тип запроса
        AdOffersResponse, // Тип ответа
        |ctx: &mut MkplAdCtx, req: &AdOffersRequest| {
            // Маппинг запроса
            AdOffersRequestMapperV1::from_api(ctx, req);
        },
        |ctx: &mut MkplAdCtx| {
            // Бизнес-логика
            ctx.ad_response = ctx.ad_request.clone();
            AdOffersResponseMapperV1::to_api(ctx)
        }
    );

    handler(req).await
}
