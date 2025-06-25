use async_trait::async_trait;
use crate::{CtxWrapper, CorContext};

#[async_trait]
pub trait CorStep<C>: Send + Sync
where
    C: CorContext + 'static,
{
    async fn execute(&self, ctx: CtxWrapper<C>);
}
