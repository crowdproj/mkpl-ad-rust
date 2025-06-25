use async_trait::async_trait;
use crate::{CorContext, CorStep, CtxWrapper};

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