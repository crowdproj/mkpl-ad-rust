pub mod handlers;

use cor::{cor_chain, cor_handler, CorContext, CorStatus, CorStep};
use handlers::context::{TestContext, ErrorCode, FieldName};
use handlers::handler_init::InitializeHandler;
use handlers::handler_validation::ValidateHandler;

#[tokio::test]
async fn test_chain() {
    let chain = cor_chain!(
        "Test Chain", 
        TestContext,
        {
            cor_handler!("Init", |wctx| async move {
                CorContext::with(wctx, |ctx: &mut TestContext| {
                    ctx.set_status(CorStatus::Running);
                });
            }),
            cor_handler!("Validate", |wctx| async move {
                CorContext::with(wctx.clone(), |ctx: &mut TestContext| {
                    if ctx.id == 0 {
                        ctx.fail("Invalid ID", ErrorCode::ValidationFailed, FieldName::Title);
                    }
                });
                
                // Асинхронная операция
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                
                CorContext::with(wctx, |ctx: &mut TestContext| {
                    if ctx.status() != CorStatus::Failing {
                        ctx.set_status(CorStatus::Finishing);
                    }
                });
            })
        }
    );

    let mut ctx = TestContext::new();
    ctx.id =42;
    let wctx = ctx.shared();

    chain.process(wctx.clone()).await;

    let ctx = CorContext::with(wctx, |ctx: &mut TestContext| ctx.status());
    assert_eq!(ctx, CorStatus::Finishing);
}

#[tokio::test]
async fn test_chain_external() {
    let chain = cor_chain!(
        "Test Chain", 
        TestContext,
        {
            cor_handler!("Init", |wctx| async move {
                InitializeHandler.execute(wctx).await;
            }),
            cor_handler!("Validate", |wctx| async move {
                ValidateHandler.execute(wctx).await;
            })
        }
    );

    let wctx = TestContext::new()
        .apply(|ctx| ctx.id = 42)
        .shared();

    chain.process(wctx.clone()).await;

    let status = CorContext::with(wctx, |ctx: &mut TestContext| ctx.status());
    assert_eq!(status, CorStatus::Finishing);
}

#[tokio::test]
async fn test_failed_validation() {
    let chain = cor_chain!(
        "Simple Validation", 
        TestContext,
        {
            cor_handler!("Initialize", |wctx| async move {
                InitializeHandler.execute(wctx).await;
            }),
            cor_handler!("Validate", |wctx| async move {
                ValidateHandler.execute(wctx).await;
            })
        }
    );

    let wctx = TestContext::new().shared();
    wctx.lock().unwrap().id = 0;

    chain.process(wctx.clone()).await;

    CorContext::with(wctx, |ctx| {
        assert_eq!(ctx.status(), CorStatus::Failing);
        assert!(!ctx.errors().is_empty());
    });
}
