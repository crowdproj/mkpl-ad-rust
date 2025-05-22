use api_v1::models::ResponseResult;
use biz_common::{BizStatus, MkplAdCtx};

#[derive(Debug)]
pub struct ResponseResultMapper;

impl ResponseResultMapper {
    // pub fn from_api(value: &Option<ResponseResult>) -> MkplAd {}
    // pub fn from_api(value: &Option<String>) -> String {
    //     value.clone().unwrap_or_default()
    // }

    pub fn to_api(ctx: &MkplAdCtx) -> Option<ResponseResult> {
        match ctx.state {
            BizStatus::Running => Some(ResponseResult::Success),
            BizStatus::Finishing => Some(ResponseResult::Success),
            BizStatus::Failing => Some(ResponseResult::Error),
            BizStatus::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_api() {
        let mut ctx_1 = MkplAdCtx {
            state: BizStatus::Running,
            ..MkplAdCtx::new()
        };
        // to_api
        assert_eq!(
            ResponseResult::Success,
            ResponseResultMapper::to_api(&mut ctx_1).unwrap()
        );

        let mut ctx_2 = MkplAdCtx {
            state: BizStatus::Finishing,
            ..MkplAdCtx::new()
        };
        // to_api
        assert_eq!(
            ResponseResult::Success,
            ResponseResultMapper::to_api(&mut ctx_2).unwrap()
        );

        let mut ctx_3 = MkplAdCtx {
            state: BizStatus::Failing,
            ..MkplAdCtx::new()
        };
        // to_api
        assert_eq!(
            ResponseResult::Error,
            ResponseResultMapper::to_api(&mut ctx_3).unwrap()
        );

        let mut ctx_4 = MkplAdCtx {
            state: BizStatus::None,
            ..MkplAdCtx::new()
        };
        // to_api
        assert_eq!(None, ResponseResultMapper::to_api(&mut ctx_4));
    }
}
