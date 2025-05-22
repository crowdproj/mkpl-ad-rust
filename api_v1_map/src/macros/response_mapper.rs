#[macro_export]
macro_rules! gen_response_mapper {
    (
        $mapper_name:ident,
        $response_type:path,
        $discriminator:expr,
        $command:expr
    ) => {
        use crate::mappers::base::{
            AdResponseObjectMapper, RequestIdConverter, ResponseResultMapper,
        };
        use biz_common::MkplAdCtx;

        #[derive(Debug)]
        pub struct $mapper_name<'a>(&'a mut MkplAdCtx);

        impl<'a> $mapper_name<'a> {
            pub fn new(ctx: &'a mut MkplAdCtx) -> Self {
                Self(ctx)
            }

            pub fn from_api(&mut self, api_obj: &$response_type) {
                self.0.request_id = RequestIdConverter::from_api(&api_obj.request_id);
                self.0.ad_response = AdResponseObjectMapper::from_api(&api_obj.ad);
                self.0.command = $command;
            }

            pub fn to_api(&self) -> $response_type {
                $response_type {
                    request_id: RequestIdConverter::to_api(&self.0.request_id),
                    response_type: Some($discriminator.to_string()),
                    result: ResponseResultMapper::to_api(&self.0),
                    ad: AdResponseObjectMapper::to_api(&self.0.ad_response),
                    errors: None,
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
                    ad_response: test_obj.clone(),
                    ..MkplAdCtx::new()
                };

                // to_api
                let mapper_out = $mapper_name(&mut ctx);
                let res = mapper_out.to_api();

                assert_eq!(
                    test_obj.title,
                    *res.ad.as_ref().unwrap().title.as_ref().unwrap()
                );

                // from_api
                let mut new_ctx = MkplAdCtx::new();
                let mut mapper_in = $mapper_name(&mut new_ctx);
                mapper_in.from_api(&res);

                assert_eq!(test_obj.title, new_ctx.ad_response.title);
            }
        }
    };
}
