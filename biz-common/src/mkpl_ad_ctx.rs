use crate::models::biz_status::BizStatus;
use crate::models::mkpl_ad_command::MkplAdCommand;
use common::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MkplAdCtx {
    pub command: MkplAdCommand,
    pub state: BizStatus,
    pub request_id: MkplAdRequestId,
    pub ad_request: MkplAd,
    pub ad_filter_request: MkplAdFilter,

    pub ad_response: MkplAd,
    pub ads_response: Vec<MkplAd>,
}

impl MkplAdCtx {
    pub fn new() -> MkplAdCtx {
        MkplAdCtx {
            command: MkplAdCommand::None,
            state: BizStatus::None,
            request_id: MkplAdRequestId::none(),

            ad_request: MkplAd::new(),
            ad_filter_request: MkplAdFilter::new(),

            ad_response: MkplAd::new(),
            ads_response: Vec::<MkplAd>::new(),
        }
    }
}
