use api_v1::models::AdUpdateRequest;

use super::object_mapper::AdUpdateObjectMapper;
use crate::mappers::base::{RequestIdConverter, DISCRIMINATOR_CREATE};
use biz_common::{models::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdUpdateRequestMapper;

impl AdUpdateRequestMapper {
    pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &AdUpdateRequest) {
        ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        ctx.ad_request = AdUpdateObjectMapper::from_api(&api_obj.ad);
        ctx.command = MkplAdCommand::Update;
    }

    pub fn to_api(ctx: &MkplAdCtx) -> AdUpdateRequest {
        AdUpdateRequest {
            request_id: RequestIdConverter::to_api(&ctx.request_id),
            request_type: Some(DISCRIMINATOR_CREATE.to_string()),
            ad: AdUpdateObjectMapper::to_api(&ctx.ad_request),
            debug: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cor::CorContext;
    use stubs::StubsMkplAd;

    #[test]
    fn test_full_conversion_cycle() {
        let test_obj = StubsMkplAd::case1();
        let mut ctx = MkplAdCtx::new().apply(|c| {
            c.ad_request = test_obj.clone();
        });

        // to_api
        let res = AdUpdateRequestMapper::to_api(&mut ctx);

        assert_eq!(
            test_obj.title,
            *res.ad.as_ref().unwrap().title.as_ref().unwrap()
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        AdUpdateRequestMapper::from_api(&mut new_ctx, &res);

        assert_eq!(test_obj.title, new_ctx.ad_request.title);
    }
}
