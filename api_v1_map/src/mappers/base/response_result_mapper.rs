use api_v1::models::ResponseResult;
use biz_common::MkplAdCtx;
use cor::{CorContext, CorStatus};

#[derive(Debug)]
pub struct ResponseResultMapper;

impl ResponseResultMapper {
    // pub fn from_api(value: &Option<ResponseResult>) -> MkplAd {}
    // pub fn from_api(value: &Option<String>) -> String {
    //     value.clone().unwrap_or_default()
    // }

    pub fn to_api(ctx: &MkplAdCtx) -> Option<ResponseResult> {
        match ctx.status() {
            CorStatus::Running => Some(ResponseResult::Success),
            CorStatus::Finishing => Some(ResponseResult::Success),
            CorStatus::Failing => Some(ResponseResult::Error),
            CorStatus::None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_api() {
        let mut ctx_1 = MkplAdCtx::new().apply(|c| {
            c.set_status(CorStatus::Running);
        });
        // to_api
        assert_eq!(
            ResponseResult::Success,
            ResponseResultMapper::to_api(&mut ctx_1).unwrap()
        );

        let mut ctx_2 = MkplAdCtx::new().apply(|c| {
            c.set_status(CorStatus::Finishing);
        });
        // to_api
        assert_eq!(
            ResponseResult::Success,
            ResponseResultMapper::to_api(&mut ctx_2).unwrap()
        );

        let mut ctx_3 = MkplAdCtx::new().apply(|c| {
            c.set_status(CorStatus::Failing);
        });
        // to_api
        assert_eq!(
            ResponseResult::Error,
            ResponseResultMapper::to_api(&mut ctx_3).unwrap()
        );

        let mut ctx_4 = MkplAdCtx::new().apply(|c| {
            c.set_status(CorStatus::None);
        });
        // to_api
        assert_eq!(None, ResponseResultMapper::to_api(&mut ctx_4));
    }
}
