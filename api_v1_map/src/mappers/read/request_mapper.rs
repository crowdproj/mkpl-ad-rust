use api_v1::models::AdReadRequest;

use crate::mappers::base::*;
use crate::mappers::read::object_mapper::AdReadObjectMapper;
use biz_common::{models::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdReadRequestMapper;

impl AdReadRequestMapper {
    pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &AdReadRequest) {
        ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        ctx.ad_request = AdReadObjectMapper::from_api(&api_obj.ad);
        ctx.command = MkplAdCommand::Create;
    }

    pub fn to_api(ctx: &MkplAdCtx) -> AdReadRequest {
        AdReadRequest {
            request_id: RequestIdConverter::to_api(&ctx.request_id),
            request_type: Some(DISCRIMINATOR_READ.to_string()),
            ad: AdReadObjectMapper::to_api(&ctx.ad_request),
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
        let ctx = MkplAdCtx::new().apply(|c| {
            c.ad_request = test_obj.clone();
        });

        // to_api
        let res = AdReadRequestMapper::to_api(&ctx);

        assert_eq!(
            test_obj.id.get(),
            res.ad.as_ref().unwrap().id.as_ref().unwrap()
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        AdReadRequestMapper::from_api(&mut new_ctx, &res);

        assert_eq!(test_obj.id, new_ctx.ad_request.id);
    }
}
