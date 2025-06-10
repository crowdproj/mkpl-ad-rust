use crate::{gen_response_mapper, mappers::base::AdResponseObjectMapper};
use api_v1::models::{AdOffersResponse, AdResponseObject};

gen_response_mapper!(
    AdOffersResponseMapper,
    AdOffersResponse,
    crate::mappers::base::DISCRIMINATOR_OFFERS,
    biz_common::models::MkplAdCommand::Offers,
    from_api: {
        ad_response: |api_obj: &AdOffersResponse| crate::mappers::base::AdResponseObjectMapper::from_api(&api_obj.ad),
        ads_response: |api_obj: &AdOffersResponse| {
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
        ad: |ctx: &MkplAdCtx| crate::mappers::base::AdResponseObjectMapper::to_api(&ctx.ad_response),
        ads: |ctx: &MkplAdCtx| Some(
            ctx.ads_response.iter()
                .filter_map(|res| AdResponseObjectMapper::to_api(&res))
                .collect()
        )
    }
);
