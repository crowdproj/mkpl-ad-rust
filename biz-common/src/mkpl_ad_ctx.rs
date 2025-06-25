use crate::models::*;
use common::*;
use cor::{cor_context, CorContext};

cor_context!(
    MkplAdCtx,
    MkplErrorCode,
    MkplFieldName,

    command: MkplAdCommand = MkplAdCommand::None,
    request_id: MkplAdRequestId = MkplAdRequestId::none(),

    ad_request: MkplAd = MkplAd::new(),
    ad_filter_request: MkplAdFilter = MkplAdFilter::new(),

    ad_response: MkplAd = MkplAd::new(),
    ads_response: Vec::<MkplAd> = Vec::<MkplAd>::new(),
);
