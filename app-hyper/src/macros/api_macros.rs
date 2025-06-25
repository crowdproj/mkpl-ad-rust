#[macro_export]
macro_rules! api_handler {
    // Вариант для методов с телом (CREATE, UPDATE)
    ($req_type:ty, $resp_type:ty, $mapper:expr, $logic:expr) => {
        |req: ::hyper::Request<::hyper::body::Incoming>| async move {
            use api_common::ApiError;
            use biz_common::{BizStatus, MkplAdCtx};
            use bytes::Bytes;
            use http_body_util::{BodyExt, Full};
            use hyper::header::CONTENT_TYPE;
            use hyper::{Response, StatusCode};
            use serde_json;
            use cor::CorContext;

            // Чтение тела
            let body_bytes = {
                let (_, body) = req.into_parts();
                let collected = body
                    .collect()
                    .await
                    .map_err(|e| ApiError::Hyper(e.into()))?;
                collected.to_bytes()
            };

            // Парсинг JSON
            let request: $req_type =
                serde_json::from_slice(&body_bytes).map_err(|e| ApiError::Serde(e.into()))?;

            // Инициализация контекста
            let mut ctx = MkplAdCtx::new().apply(|c| c.set_status(BizStatus::Running));

            // Маппинг запроса
            $mapper(&mut ctx, &request);

            // Бизнес-логика
            let response: $resp_type = $logic(&mut ctx);

            // Сериализация и формирование ответа
            let json = serde_json::to_vec(&response).map_err(|e| ApiError::Serde(e.into()))?;

            Response::builder()
                .status(StatusCode::CREATED)
                .header(CONTENT_TYPE, "application/json")
                .body(Full::new(Bytes::from(json)))
                .map_err(|e| ApiError::HyperHttp(e.into()))
        }
    };
}
