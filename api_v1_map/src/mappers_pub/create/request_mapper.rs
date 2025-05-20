use api_v1::models::AdCreateRequest;

use crate::mappers_priv::mkpl_ad_request_id::RequestIdConverter;
use crate::mappers_pub::create::object_mapper::AdCreateObjectMapper;
use biz_common::{models::mkpl_ad_command::MkplAdCommand, MkplAdCtx};

#[derive(Debug)]
pub struct AdCreateRequestMapper<'a>(&'a mut MkplAdCtx);

impl<'a> AdCreateRequestMapper<'a> {
    pub fn new(ctx: &'a mut MkplAdCtx) -> Self {
        AdCreateRequestMapper(ctx)
    }

    pub fn from_api(&mut self, api_obj: &AdCreateRequest) {
        self.0.request_id = RequestIdConverter::from_api(&api_obj.request_id);
        self.0.ad_request = AdCreateObjectMapper::from_api(&api_obj.ad);
        self.0.command = MkplAdCommand::Create;
    }

    pub fn to_api(&self) -> AdCreateRequest {
        AdCreateRequest {
            request_id: RequestIdConverter::to_api(&self.0.request_id),
            request_type: Some("create".to_string()),
            ad: AdCreateObjectMapper::to_api(&self.0.ad_request),
            debug: None,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate stubs;
    use self::stubs::StubsMkplAd;
    use super::*;

    #[test]
    fn test_full_conversion_cycle() {
        let test_obj = StubsMkplAd::case1();
        let mut ctx = MkplAdCtx {
            ad_request: test_obj.clone(),
            ..MkplAdCtx::new()
        };

        // to_api
        let mapper_out = AdCreateRequestMapper(&mut ctx);
        let res = mapper_out.to_api();

        assert_eq!(
            test_obj.title,
            *res.ad.as_ref().unwrap().title.as_ref().unwrap()
        );

        // from_api
        let mut new_ctx = MkplAdCtx::new();
        let mut mapper_in = AdCreateRequestMapper(&mut new_ctx);
        mapper_in.from_api(&res);

        assert_eq!(test_obj.title, new_ctx.ad_request.title);
    }
}
