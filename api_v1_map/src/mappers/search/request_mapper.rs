use api_v1::models::AdSearchRequest;

use super::object_mapper::AdSearchFilterMapper;
use crate::mappers::base::{RequestIdConverter, DISCRIMINATOR_SEARCH};
use biz_common::{models::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdSearchRequestMapper;

impl AdSearchRequestMapper {
    pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &AdSearchRequest) {
        ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        ctx.ad_filter_request = AdSearchFilterMapper::from_api(&api_obj.ad_filter);
        ctx.command = MkplAdCommand::Search;
    }

    pub fn to_api(ctx: &MkplAdCtx) -> AdSearchRequest {
        AdSearchRequest {
            request_id: RequestIdConverter::to_api(&ctx.request_id),
            request_type: Some(DISCRIMINATOR_SEARCH.to_string()),
            ad_filter: AdSearchFilterMapper::to_api(&ctx.ad_filter_request),
            debug: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cor::CorContext;
    use stubs::StubsMkplAdFilter;

    #[test]
    fn test_full_conversion_cycle() {
        let test_obj = StubsMkplAdFilter::case1();
        let mut ctx = MkplAdCtx::new().apply(|c| {
            c.ad_filter_request = test_obj.clone();
        });

        // to_api
        let res = AdSearchRequestMapper::to_api(&mut ctx);

        assert_eq!(
            test_obj.query,
            res.ad_filter.as_ref().unwrap().search_string
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        AdSearchRequestMapper::from_api(&mut new_ctx, &res);

        assert_eq!(test_obj.query, new_ctx.ad_filter_request.query);
    }
}
