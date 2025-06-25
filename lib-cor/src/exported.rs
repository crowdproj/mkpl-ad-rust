use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::{CorContext};

pub type CtxWrapper<C> = Arc<Mutex<C>>;

#[async_trait]
pub trait CorStep<C>: Send + Sync
where
    C: CorContext + 'static,
{
    async fn execute(&self, ctx: CtxWrapper<C>);
}

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

pub struct Cor<C> {
    name: &'static str,
    steps: Vec<Box<dyn CorStep<C>>>,
}

impl<C> Cor<C>
where
    C: CorContext + 'static,
{
    pub fn new(name: &'static str, steps: Vec<Box<dyn CorStep<C>>>) -> Self {
        Self { name, steps }
    }

    pub async fn process(&self, ctx: CtxWrapper<C>) {
        for step in &self.steps {
            step.execute(ctx.clone()).await;
        }
    }
}

#[async_trait]
impl<C> CorStep<C> for Cor<C>
where
    C: CorContext + 'static,
{
    async fn execute(&self, ctx: CtxWrapper<C>) {
        println!("Executing chain: {}", self.name);
        self.process(ctx).await;
    }
}