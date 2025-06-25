use cor::{CorContext, CtxWrapper, CorStep};
use async_trait::async_trait;
use super::context::*;

pub struct InitializeHandler;

#[async_trait]
impl CorStep<TestContext> for InitializeHandler {
    async fn execute(&self, wctx: CtxWrapper<TestContext>) {
        CorContext::with(wctx, |ctx| {
            ctx.set_status(crate::CorStatus::Running);
        });
    }
}