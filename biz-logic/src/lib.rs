use biz_common::{MkplAdCtx};
use cor::*;

// Основной класс, управляющий обработкой рекламы
pub struct MkplAdProcessor {
    // handlers: Vec<Box<dyn AdHandler + Send + Sync>>,
    chain: Cor<MkplAdCtx>,
}

impl MkplAdProcessor {
    // Настройка цепочки обработчиков
    pub fn new() -> Self {
        MkplAdProcessor {
            chain: cor_chain!(
                "Test Chain", 
                MkplAdCtx,
                {
                    cor_handler!("Init", |wctx| async move {
                        CorContext::with(wctx, |ctx: &mut MkplAdCtx| {
                            if ctx.status() == CorStatus::None {
                                ctx.set_status(CorStatus::Running);
                            }
                        });
                    })
                }
            ),
       }
    }

    // Метод обработки контекста
    pub async fn process(&self, wctx: CtxWrapper<MkplAdCtx>) {
        self.chain.process(wctx).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biz_common::*;

    #[tokio::test]
    async fn test_mkpl_ad_processor() {
        let processor = MkplAdProcessor::new();

        let wctx: CtxWrapper<MkplAdCtx> = MkplAdCtx::new()
            .apply(|ctx| ctx.command = MkplAdCommand::Read)
            .shared();

        processor.process(wctx.clone()).await;

        let final_command = CorContext::access(wctx, |ctx| ctx.command);
        assert_eq!(final_command, MkplAdCommand::Read);
    }
}
