use crate::models::biz_status::BizStatus;
use crate::models::mkpl_ad_command::MkplAdCommand;
use common::mkpl_ad::*;
use common::mkpl_ad_request_id::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MkplAdCtx {
    pub command: MkplAdCommand,
    pub state: BizStatus,
    pub request_id: MkplAdRequestId,
    pub ad_request: MkplAd,
}

impl MkplAdCtx {
    pub fn new() -> MkplAdCtx {
        MkplAdCtx {
            command: MkplAdCommand::None,
            state: BizStatus::None,
            request_id: MkplAdRequestId::none(),

            ad_request: MkplAd::new(),
        }
    }
}

trait ToCtx<T> {
    fn to_ctx(&self, dat: &T);
}
