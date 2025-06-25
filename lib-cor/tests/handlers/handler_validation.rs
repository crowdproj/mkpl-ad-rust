use cor::{CorContext, CtxWrapper, CorStep};
use async_trait::async_trait;
use super::context::*;

pub struct ValidateHandler;

#[async_trait]
impl CorStep<TestContext> for ValidateHandler {
    async fn execute(&self, wctx: CtxWrapper<TestContext>) {
        let should_fail = CorContext::access(wctx.clone(), |ctx| ctx.id == 0);
        
        if should_fail {
            CorContext::with(wctx.clone(), |ctx| {
                ctx.fail("Invalid ID", ErrorCode::ValidationFailed, FieldName::Title);
            });
        } else {
            CorContext::with(wctx.clone(), |ctx| {
                ctx.set_status(crate::CorStatus::Finishing);
            });
        }
    }
}
