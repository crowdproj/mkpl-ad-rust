use std::future::Future;
use std::pin::Pin;
use futures::future::join_all;
use std::sync::{Arc, Mutex};

pub type CtxWrapper<C> = Arc<Mutex<C>>;
// pub type StepFn<C> = dyn Fn(CtxWrapper<C>) -> Pin<Box<dyn Future<Output = ()> + 'static>> + Send + Sync;
pub type StepFn<C> = dyn Fn(CtxWrapper<C>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>> + Send + Sync + 'static;

// Внутренняя структура для шагов обработки
struct CorInternal<C> {
    steps: Vec<Box<StepFn<C>>>,
}

// Экспортируемый тип
pub struct Cor<C>(CorInternal<C>);

impl<C> Cor<C> {
    pub fn new(steps: Vec<Box<StepFn<C>>>) -> Self {
        Cor(CorInternal { steps })
    }

    pub async fn parallel(&self, ctx: CtxWrapper<C>) {
        let futures: Vec<_> = self.0.steps.iter().map(|step| {
            let ctx_clone = CtxWrapper::clone(&ctx);
            step(ctx_clone)
        }).collect();
        join_all(futures).await;
    }

    pub async fn process(&self, ctx: CtxWrapper<C>) {
        for step in &self.0.steps {
            let ctx_clone = CtxWrapper::clone(&ctx);
            step(ctx_clone).await;
        }
    }
}
