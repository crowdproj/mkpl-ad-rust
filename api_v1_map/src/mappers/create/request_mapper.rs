use api_v1::models::AdCreateRequest;

use crate::mappers::base::{RequestIdConverter, DISCRIMINATOR_CREATE};
use crate::mappers::create::object_mapper::AdCreateObjectMapper;
use biz_common::{models::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdCreateRequestMapper;

impl AdCreateRequestMapper {
    pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &AdCreateRequest) {
        ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        ctx.ad_request = AdCreateObjectMapper::from_api(&api_obj.ad);
        ctx.command = MkplAdCommand::Create;
    }

    pub fn to_api(ctx: &MkplAdCtx) -> AdCreateRequest {
        AdCreateRequest {
            request_id: RequestIdConverter::to_api(&ctx.request_id),
            request_type: Some(DISCRIMINATOR_CREATE.to_string()),
            ad: AdCreateObjectMapper::to_api(&ctx.ad_request),
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
        let res = AdCreateRequestMapper::to_api(&mut ctx);

        assert_eq!(
            test_obj.title,
            *res.ad.as_ref().unwrap().title.as_ref().unwrap()
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        AdCreateRequestMapper::from_api(&mut new_ctx, &res);

        assert_eq!(test_obj.title, new_ctx.ad_request.title);
    }
}
