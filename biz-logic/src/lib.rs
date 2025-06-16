use async_trait::async_trait;
use biz_common::{MkplAdCommand, MkplAdCtx};
use std::sync::{Arc, Mutex};

// Абстрактный trait для обработчиков
#[async_trait]
trait AdHandler {
    async fn handle(&self, ctx: Arc<Mutex<MkplAdCtx>>);
}

// Конкретный обработчик A
struct HandlerA {}

#[async_trait]
impl AdHandler for HandlerA {
    async fn handle(&self, ctx: Arc<Mutex<MkplAdCtx>>) {
        let mut guard = ctx.lock().unwrap();
        guard.command = MkplAdCommand::Create;
        println!("HandlerA modified context to: {:?}", guard.command);
    }
}

// Конкретный обработчик B
struct HandlerB {}

#[async_trait]
impl AdHandler for HandlerB {
    async fn handle(&self, ctx: Arc<Mutex<MkplAdCtx>>) {
        let mut guard = ctx.lock().unwrap();
        guard.command = MkplAdCommand::Read;
        println!("HandlerB modified context to: {:?}", guard.command);
    }
}

// Основной класс, управляющий обработкой рекламы
pub struct MkplAdProcessor {
    handlers: Vec<Box<dyn AdHandler + Send + Sync>>,
}

impl MkplAdProcessor {
    // Настройка цепочки обработчиков
    pub fn new() -> Self {
        let handler_a = Box::new(HandlerA {});
        let handler_b = Box::new(HandlerB {});
        MkplAdProcessor {
            handlers: vec![handler_a, handler_b],
        }
    }

    // Метод обработки контекста
    pub async fn process(&self, ctx: Arc<Mutex<MkplAdCtx>>) {
        for handler in self.handlers.iter() {
            handler.handle(ctx.clone()).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mkpl_ad_processor() {
        let processor = MkplAdProcessor::new();

        let ctx: Arc<Mutex<MkplAdCtx>> = Arc::new(Mutex::new(MkplAdCtx::new()));

        processor.process(ctx.clone()).await;

        let final_command = ctx.lock().unwrap().command.clone();
        assert_eq!(final_command, MkplAdCommand::Read);
    }
}
