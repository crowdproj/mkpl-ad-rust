use hyper::Request;

pub trait RequestParser<T> {
    // fn parse_request(&self, data: &[u8]) -> Result<T, ApiError>;
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str>;
}

// use crate::ApiError; // Импорт вашего типа ошибок

// // api_common/src/request_parser.rs
// use hyper::{Request, body::Incoming};
// use http_body_util::BodyExt; // 1. Добавляем необходимый трейт
// use std::collections::HashMap;

// pub struct RequestParser {
//     pub path_params: HashMap<String, String>,
//     pub query_params: HashMap<String, String>,
//     pub headers: HashMap<String, String>,
//     pub body: Vec<u8>,
// }

// impl RequestParser {
//     pub async fn from_request(req: Request<Incoming>) -> Result<Self, ApiError> {
//         let (parts, body) = req.into_parts();

//         // 2. Используем BodyExt для чтения тела
//         let collected = body.collect().await?;
//         let body_bytes = collected.to_bytes();

//         // 3. Исправляем обработку query параметров
//         let query_params = parts.uri.query()
//             .map(|q| {
//                 url::form_urlencoded::parse(q.as_bytes())
//                     .map(|(k, v)| (k.into_owned(), v.into_owned())) // Преобразуем Cow в String
//                     .collect()
//             })
//             .unwrap_or_default();

//         // 4. Обработка заголовков
//         let headers = parts.headers.iter()
//             .filter_map(|(name, value)| {
//                 value.to_str()
//                     .ok()
//                     .map(|val| (name.to_string(), val.to_string()))
//             })
//             .collect();

//         Ok(Self {
//             path_params: HashMap::new(),
//             query_params,
//             headers,
//             body: body_bytes.to_vec(),
//         })
//     }
// }
