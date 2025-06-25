use async_trait::async_trait;
use crate::{CorStep,CtxWrapper, CorContext};
use std::future::Future;
use std::pin::Pin;

pub struct CorClosureStep<C> {
    name: &'static str,
    f: Box<dyn Fn(CtxWrapper<C>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>,
}

impl<C> CorClosureStep<C> {
    pub fn new<Fut, F>(name: &'static str, f: F) -> Self
    where
        F: Fn(CtxWrapper<C>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        Self {
            name,
            f: Box::new(move |ctx| Box::pin(f(ctx))),
        }
    }
}

#[async_trait]
impl<C> CorStep<C> for CorClosureStep<C>
where
    C: CorContext + 'static,
{
    async fn execute(&self, ctx: CtxWrapper<C>) {
        println!("Executing step: {}", self.name);
        (self.f)(ctx).await;
    }
}

