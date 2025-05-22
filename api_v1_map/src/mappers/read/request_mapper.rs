use api_v1::models::AdReadRequest;

use crate::mappers::base::*;
use crate::mappers::read::object_mapper::AdReadObjectMapper;
use biz_common::{models::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdReadRequestMapper<'a>(&'a mut MkplAdCtx);

impl<'a> AdReadRequestMapper<'a> {
    pub fn new(ctx: &'a mut MkplAdCtx) -> Self {
        AdReadRequestMapper(ctx)
    }

    pub fn from_api(&mut self, api_obj: &AdReadRequest) {
        self.0.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        self.0.ad_request = AdReadObjectMapper::from_api(&api_obj.ad);
        self.0.command = MkplAdCommand::Create;
    }

    pub fn to_api(&self) -> AdReadRequest {
        AdReadRequest {
            request_id: RequestIdConverter::to_api(&self.0.request_id),
            request_type: Some(DISCRIMINATOR_READ.to_string()),
            ad: AdReadObjectMapper::to_api(&self.0.ad_request),
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
        let mut ctx = MkplAdCtx {
            ad_request: test_obj.clone(),
            ..MkplAdCtx::new()
        };

        // to_api
        let mapper_out = AdReadRequestMapper(&mut ctx);
        let res = mapper_out.to_api();

        assert_eq!(
            test_obj.id.get(),
            res.ad.as_ref().unwrap().id.as_ref().unwrap()
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        let mut mapper_in = AdReadRequestMapper(&mut new_ctx);
        mapper_in.from_api(&res);

        assert_eq!(test_obj.id, new_ctx.ad_request.id);
    }
}
