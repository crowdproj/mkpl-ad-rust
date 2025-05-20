//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use swagger::{Has, XSpanIdString};
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use api_v1::models;
use api_v1_map::AdCreateRequestMapper;
use biz_common::{models::mkpl_ad_command::MkplAdCommand, MkplAdCtx};

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service = api_v1::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    info!("Starting a server (over http, so no TLS)");
    // Using HTTP
    hyper::server::Server::bind(&addr)
        .serve(service)
        .await
        .unwrap()
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server {
            marker: PhantomData,
        }
    }
}

use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use swagger::auth::Authorization;
// use crate::server_auth;

use api_v1::server::MakeService;
use api_v1::{
    AdCreateResponse, AdDeleteResponse, AdOffersResponse, AdReadResponse, AdSearchResponse,
    AdUpdateResponse, Api, SwaggerResponse,
};
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Send + Sync,
{
    /// Create ad
    async fn ad_create(
        &self,
        ad_create_request: models::AdCreateRequest,
        x_request_id: Option<String>,
        context: &C,
    ) -> Result<AdCreateResponse, ApiError> {
        info!(
            "ad_create({:?}, {:?}) - X-Span-ID: {:?}",
            ad_create_request,
            x_request_id,
            context.get().0.clone()
        );
        let mut ctx: MkplAdCtx = MkplAdCtx::new();
        let mut ctx_mapper = AdCreateRequestMapper::new(&mut ctx);
        ctx_mapper.from_api(&ad_create_request);

        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
        Ok(AdCreateResponse::Success(models::AdCreateResponse::new()))
    }

    /// Delete ad
    async fn ad_delete(
        &self,
        ad_delete_request: models::AdDeleteRequest,
        x_request_id: Option<String>,
        context: &C,
    ) -> Result<AdDeleteResponse, ApiError> {
        info!(
            "ad_delete({:?}, {:?}) - X-Span-ID: {:?}",
            ad_delete_request,
            x_request_id,
            context.get().0.clone()
        );
        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
        Ok(AdDeleteResponse::Success(models::AdDeleteResponse::new()))
    }

    /// Search offers
    async fn ad_offers(
        &self,
        ad_offers_request: models::AdOffersRequest,
        x_request_id: Option<String>,
        context: &C,
    ) -> Result<AdOffersResponse, ApiError> {
        info!(
            "ad_offers({:?}, {:?}) - X-Span-ID: {:?}",
            ad_offers_request,
            x_request_id,
            context.get().0.clone()
        );
        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
        Ok(AdOffersResponse::Success(models::AdOffersResponse::new()))
    }

    /// Read ad
    async fn ad_read(
        &self,
        ad_read_request: models::AdReadRequest,
        x_request_id: Option<String>,
        context: &C,
    ) -> Result<AdReadResponse, ApiError> {
        info!(
            "ad_read({:?}, {:?}) - X-Span-ID: {:?}",
            ad_read_request,
            x_request_id,
            context.get().0.clone()
        );
        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
        Ok(AdReadResponse::Success(models::AdReadResponse::new()))
    }

    /// Search ad
    async fn ad_search(
        &self,
        ad_search_request: models::AdSearchRequest,
        x_request_id: Option<String>,
        context: &C,
    ) -> Result<AdSearchResponse, ApiError> {
        info!(
            "ad_search({:?}, {:?}) - X-Span-ID: {:?}",
            ad_search_request,
            x_request_id,
            context.get().0.clone()
        );
        Err(ApiError("Api-Error: Operation is NOT implemented".into()))
        // Ok(AdSearchResponse::new())
    }

    /// Update ad
    async fn ad_update(
        &self,
        ad_update_request: models::AdUpdateRequest,
        x_request_id: Option<String>,
        context: &C,
    ) -> Result<AdUpdateResponse, ApiError> {
        info!(
            "ad_update({:?}, {:?}) - X-Span-ID: {:?}",
            ad_update_request,
            x_request_id,
            context.get().0.clone()
        );
        let response = models::AdUpdateResponse {
            response_type: Some("update".to_string()),
            request_id: Some("12345".to_string()),
            result: Some(models::ResponseResult::Success),
            errors: None,
            ad: Some(models::AdResponseObject {
                id: Some("123".to_string()),
                title: Some("Updated Ad".to_string()),
                description: Some("Updated Ad Description".to_string()),
                ad_type: Some(models::DealSide::Supply),
                visibility: Some(models::AdVisibility::Public),
                product_id: None,
                owner_id: Some("owner-1234".to_string()),
                lock: Some("lock-1234".to_string()),
                permissions: Some(Vec::new()),
            }),
        };
        Ok(AdUpdateResponse::Success(response))
    }

    /// Swagger
    async fn swagger(&self, context: &C) -> Result<SwaggerResponse, ApiError> {
        info!("swagger() - X-Span-ID: {:?}", context.get().0.clone());
        // Err(ApiError("Api-Error: Operation is NOT implemented".into()))
        Ok(SwaggerResponse::Success("Hello, world".into()))
    }
}
