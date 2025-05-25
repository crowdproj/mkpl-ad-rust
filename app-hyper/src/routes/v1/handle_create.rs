use api_common::ApiError;
use api_v1::models::{AdCreateRequest, AdCreateResponse};
use api_v1_map::{AdCreateRequestMapperV1, AdCreateResponseMapperV1};
use biz_common::{BizStatus, MkplAdCtx};
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::{Request, Response, StatusCode};
use serde_json;

pub async fn handle_create(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, ApiError> {
    // Разделяем запрос на части и тело
    let (_parts, body) = req.into_parts();

    // Читаем всё тело запроса
    let collected = body.collect().await.map_err(|e| ApiError::Hyper(e))?;

    let body_bytes = collected.to_bytes();

    // Парсинг JSON
    let request: AdCreateRequest =
        serde_json::from_slice(&body_bytes).map_err(|e| ApiError::Serde(e))?;

    // Логика обработки
    let mut ctx = MkplAdCtx {
        state: BizStatus::Running,
        ..MkplAdCtx::new()
    };

    // Request
    let mut mapper = AdCreateRequestMapperV1::new(&mut ctx);
    mapper.from_api(&request);

    // Обработка бизнес-логики
    ctx.ad_response = ctx.ad_request.clone();

    let mapper = AdCreateResponseMapperV1::new(&mut ctx);

    let response: AdCreateResponse = mapper.to_api();

    // Сериализация ответа
    let json = serde_json::to_vec(&response).map_err(|e| ApiError::Serde(e))?;

    // Сборка ответа
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(json)))
        .map_err(|e| ApiError::HyperHttp(e))?)
}
