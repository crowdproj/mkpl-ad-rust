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
        pub struct $mapper_name;

        impl $mapper_name {
            pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &$response_type) {
                ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
                ctx.ad_response = AdResponseObjectMapper::from_api(&api_obj.ad);
                ctx.command = $command;
            }

            pub fn to_api(ctx: &mut MkplAdCtx) -> $response_type {
                $response_type {
                    request_id: RequestIdConverter::to_api(&ctx.request_id),
                    response_type: Some($discriminator.to_string()),
                    result: ResponseResultMapper::to_api(&ctx),
                    ad: AdResponseObjectMapper::to_api(&ctx.ad_response),
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
                let res = $mapper_name::to_api(&mut ctx);

                assert_eq!(
                    test_obj.title,
                    *res.ad.as_ref().unwrap().title.as_ref().unwrap()
                );

                // from_api
                let mut new_ctx = MkplAdCtx::new();
                $mapper_name::from_api(&mut new_ctx, &res);

                assert_eq!(test_obj.title, new_ctx.ad_response.title);
            }
        }
    };
}
