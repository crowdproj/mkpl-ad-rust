use crate::{gen_response_mapper, mappers::base::AdResponseObjectMapper};
use api_v1::models::{AdResponseObject, AdSearchResponse};

gen_response_mapper!(
    AdSearchResponseMapper,
    AdSearchResponse,
    crate::mappers::base::DISCRIMINATOR_SEARCH,
    biz_common::models::MkplAdCommand::Search,
    from_api: {
        ads_response: |api_obj: &AdSearchResponse| {
            api_obj.ads.as_ref()
                .map(
                    |ads: &Vec<AdResponseObject>| ads.iter()
                    .map(|ad| AdResponseObjectMapper::from_api(&Some(ad.clone())))
                    .collect()
                )
                .unwrap_or_default()
        }
    },
    to_api: {
        ads: |ctx: &MkplAdCtx| Some(
            ctx.ads_response.iter()
                .filter_map(|res| AdResponseObjectMapper::to_api(&res))
                .collect()
        )
    }
);
