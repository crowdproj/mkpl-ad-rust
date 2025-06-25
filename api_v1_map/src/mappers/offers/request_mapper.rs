use api_v1::models::AdOffersRequest;

use crate::mappers::base::*;
use crate::mappers::offers::object_mapper::AdOffersObjectMapper;
use biz_common::{models::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdOffersRequestMapper;

impl AdOffersRequestMapper {
    pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &AdOffersRequest) {
        ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        ctx.ad_request = AdOffersObjectMapper::from_api(&api_obj.ad);
        ctx.command = MkplAdCommand::Create;
    }

    pub fn to_api(ctx: &MkplAdCtx) -> AdOffersRequest {
        AdOffersRequest {
            request_id: RequestIdConverter::to_api(&ctx.request_id),
            request_type: Some(DISCRIMINATOR_OFFERS.to_string()),
            ad: AdOffersObjectMapper::to_api(&ctx.ad_request),
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
        let res = AdOffersRequestMapper::to_api(&ctx);

        assert_eq!(
            test_obj.id.get(),
            res.ad.as_ref().unwrap().id.as_ref().unwrap()
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        AdOffersRequestMapper::from_api(&mut new_ctx, &res);

        assert_eq!(test_obj.id, new_ctx.ad_request.id);
    }
}
