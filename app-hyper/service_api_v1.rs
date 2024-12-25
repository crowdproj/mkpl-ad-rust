use api_v1::{
    AdCreateResponse,
    AdDeleteResponse,
    AdOffersResponse,
    AdReadResponse,
    AdSearchResponse,
    AdUpdateResponse,
    Api,
    SwaggerResponse,
    models,
};
use async_trait::async_trait;
use log::info;
use swagger_ui::{swagger_spec_file, Spec, Config, DefaultModelRendering, DocExpansion, Filter, UrlObject};
use crate::server::Server;
use swagger::{ApiError, XSpanIdString, Has};

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
        // Err(ApiError("Generic failure".into()))
        let rs = models::AdCreateResponse {
            response_type: ad_create_request.request_type,
            request_id: ad_create_request.request_id,
            result: Some(models::ResponseResult::Success),
            ad: match ad_create_request.ad {
                Some(ad) => Some(models::AdResponseObject {
                    title: ad.title.clone(),
                    description: ad.description.clone(),
                    ..models::AdResponseObject::new()
                }),
                None => None,
            },
            ..models::AdCreateResponse::new()
        };
        Ok(AdCreateResponse::Success(rs))
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
        Err(ApiError("Generic failure".into()))
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
        Err(ApiError("Generic failure".into()))
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
        Err(ApiError("Generic failure".into()))
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
        Err(ApiError("Generic failure".into()))
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
        Err(ApiError("Generic failure".into()))
    }

    /// Swagger
    async fn swagger(&self, context: &C) -> Result<SwaggerResponse, ApiError> {
        info!("swagger() - X-Span-ID: {:?}", context.get().0.clone());
        let _spec: Spec = swagger_spec_file!("../../specs/spec-crowdproj-ad-v1.yaml");
        let _config: Config = Config {
            url: "".to_string(),
            urls: vec![UrlObject {
                url: "v1/spec.yaml".to_string(),
                name: "API V1".to_string(),
            }],
            deep_linking: false,
            display_operation_id: false,
            default_models_expand_depth: 0,
            default_model_expand_depth: 0,
            default_model_rendering: DefaultModelRendering::Example,
            display_request_duration: false,
            doc_expansion: DocExpansion::List,
            filter: Filter::Bool(false),
            max_displayed_tags: 0,
            show_extensions: false,
            show_common_extensions: false,
        };

        Ok(SwaggerResponse::Success(
            "<html><body><b>Hello World!</b></body></html>".to_string(),
        ))
        // Err(ApiError("Generic failure".into()))
    }
}
