use api_v1::models::AdDeleteRequest;

use crate::mappers::base::*;
use crate::mappers::delete::object_mapper::AdDeleteObjectMapper;
use biz_common::{models::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdDeleteRequestMapper;

impl AdDeleteRequestMapper {
    pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &AdDeleteRequest) {
        ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        ctx.ad_request = AdDeleteObjectMapper::from_api(&api_obj.ad);
        ctx.command = MkplAdCommand::Create;
    }

    pub fn to_api(ctx: &MkplAdCtx) -> AdDeleteRequest {
        AdDeleteRequest {
            request_id: RequestIdConverter::to_api(&ctx.request_id),
            request_type: Some(DISCRIMINATOR_READ.to_string()),
            ad: AdDeleteObjectMapper::to_api(&ctx.ad_request),
            debug: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stubs::StubsMkplAd;

    #[test]
    fn test_full_conversion_cycle() {
        let test_obj = StubsMkplAd::case1();
        let ctx = MkplAdCtx {
            ad_request: test_obj.clone(),
            ..MkplAdCtx::new()
        };

        // to_api
        let res = AdDeleteRequestMapper::to_api(&ctx);

        assert_eq!(
            test_obj.id.get(),
            res.ad.as_ref().unwrap().id.as_ref().unwrap()
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        AdDeleteRequestMapper::from_api(&mut new_ctx, &res);

        assert_eq!(test_obj.id, new_ctx.ad_request.id);
    }
}
