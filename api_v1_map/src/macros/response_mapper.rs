#[macro_export]
macro_rules! gen_response_mapper {
    // Сокращенная ветка для CRUD операций
    (
        $mapper_name:ident,
        $response_type:path,
        $discriminator:expr,
        $command:expr
    ) => {
        gen_response_mapper!(
            $mapper_name,
            $response_type,
            $discriminator,
            $command,
            from_api: {
                ad_response: |api_obj: &$response_type| crate::mappers::base::AdResponseObjectMapper::from_api(&api_obj.ad)
            },
            to_api: {
                ad: |ctx: &MkplAdCtx| crate::mappers::base::AdResponseObjectMapper::to_api(&ctx.ad_response)
            }
        );
    };

    // Полная ветка с гибким определением полей
    (
        $mapper_name:ident,
        $response_type:path,
        $discriminator:expr,
        $command:expr,
        from_api: {
            $($from_field:ident: $from_expr:expr),* $(,)?
        },
        to_api: {
            $($to_field:ident: $to_expr:expr),* $(,)?
        }
    ) => {
        use crate::mappers::base::{RequestIdConverter, ResponseResultMapper};
        use biz_common::MkplAdCtx;

        #[derive(Debug)]
        pub struct $mapper_name;

        impl $mapper_name {
            pub fn from_api(ctx: &mut MkplAdCtx, api_obj: &$response_type) {
                ctx.request_id = RequestIdConverter::from_api(&api_obj.request_id);
                ctx.command = $command;

                // Обработка пользовательских полей
                $(
                    let value = $from_expr(api_obj);
                    ctx.$from_field = value;
                )*
            }

            pub fn to_api(ctx: &mut MkplAdCtx) -> $response_type {
                let mut response = <$response_type>::new();

                // Устанавливаем обязательные поля
                response.request_id = RequestIdConverter::to_api(&ctx.request_id);
                response.response_type = Some($discriminator.to_string());
                response.result = ResponseResultMapper::to_api(&ctx);

                // Установка пользовательских полей
                $(
                    response.$to_field = $to_expr(ctx);
                )*

                response
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use stubs::StubsMkplAd;
            use cor::CorContext;

            #[test]
            fn test_full_conversion_cycle() {
                let test_obj = StubsMkplAd::case1();
                let test_objs = vec![StubsMkplAd::case1()];
                let mut source_ctx = MkplAdCtx::new().apply(|c| {
                    c.ad_response = test_obj.clone();
                    c.ads_response = test_objs.clone();
                });

                // // Подготовка исходного контекста
                // $(
                //     source_ctx.$from_field = test_obj;
                // )*

                // Проверяем to_api
                let res = $mapper_name::to_api(&mut source_ctx);

                // Проверяем from_api
                let mut target_ctx = MkplAdCtx::new();
                $mapper_name::from_api(&mut target_ctx, &res);

                // Проверяем, что данные совпадают
                $(
                    assert_eq!(source_ctx.$from_field, target_ctx.$from_field);
                )*
            }
        }
    };
}
