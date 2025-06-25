use cor::{cor_create, cor_handler, CorContext, CorError, CorStatus, CtxWrapper};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum ErrorCode {
    ValidationFailed,
    InternalError,
}

#[derive(Debug)]
enum FieldName {
    Title,
    Description,
}

// Минимальная реализация контекста для тестов
struct TestContext {
    id: u32,
    status: CorStatus,
    errors: Vec<CorError<ErrorCode, FieldName>>,
}

impl Debug for TestContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{id={}, status={:?}, errors={:?}}}",
            self.id, self.status, self.errors
        )
    }
}

impl CorContext<ErrorCode, FieldName> for TestContext {
    fn new() -> Self {
        Self {
            id: 0,
            status: CorStatus::None,
            errors: vec![],
        }
    }

    fn push_error(&mut self, err: CorError<ErrorCode, FieldName>) {
        self.errors.push(err);
    }

    fn set_status(&mut self, status: CorStatus) {
        self.status = status;
    }
}

// Тестируем простую инициализацию и проверку валидности
#[tokio::test]
async fn test_successful_validation() {
    let chain = cor_create!(
        "Simple Validation", TestContext,
        {
            // Используем &mut TestContext вместо CtxWrapper
            cor_handler!("Initialize", TestContext, |wctx: CtxWrapper<TestContext>| async move {
                let mut ctx = wctx.lock().unwrap();
                ctx.set_status(CorStatus::Running);
            }),
            // Аналогично исправляем здесь
            cor_handler!("Validate", TestContext, |wctx: CtxWrapper<TestContext>| async move {
                let mut ctx = wctx.lock().unwrap();
                if ctx.id != 0 {
                    ctx.set_status(CorStatus::Finishing);
                } else {
                    let err = CorError {
                        msg: "Invalid ID".to_string(),
                        code: ErrorCode::ValidationFailed,
                        field: FieldName::Title,
                        error: None,
                    };
                    // Используем метод fail из CorContext
                    ctx.fail(err);
                }
            })
        }
    );

    let wctx = Arc::new(Mutex::new(TestContext {
        id: 1,
        status: CorStatus::None,
        errors: vec![],
    }));

    chain.process(wctx.clone()).await;

    let ctx = wctx.lock().unwrap();
    assert_eq!(ctx.status, CorStatus::Finishing);
    assert!(ctx.errors.is_empty());
}

// // Тестируем случай, когда возникает ошибка
// #[tokio::test]
// async fn test_failed_validation() {
//     let chain = cor_create!(
//         "Simple Validation", TestContext,
//         {
//             cor_handler!("Initialize", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 ctx.set_status(CorStatus::Running);
//             }),
//             cor_handler!("Validate", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 if ctx.id == 0 {
//                     let err = CorError {
//                         msg: "Invalid ID".to_string(),
//                         code: ErrorCode::ValidationFailed,
//                         field: FieldName::Title,
//                         error: None,
//                     };
//                     ctx.fail(err);
//                 }
//             })
//         }
//     );

//     let wctx = Arc::new(Mutex::new(TestContext {
//         id: 0,
//         status: CorStatus::None,
//         errors: vec![],
//     }));

//     chain.process(wctx.clone()).await;

//     let ctx = wctx.lock().unwrap();
//     assert_eq!(ctx.status, CorStatus::Failing);
//     assert!(!ctx.errors.is_empty());
//     assert_eq!(ctx.errors.len(), 1);
//     assert_eq!(ctx.errors[0].msg, "Invalid ID");
// }

// // Тестируем составную обработку
// #[tokio::test]
// async fn test_complex_processing() {
//     let chain = cor_create!(
//         "Complex Processing", TestContext,
//         {
//             cor_handler!("Initialize", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 ctx.set_status(CorStatus::Running);
//             }),
//             cor_handler!("Validate", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 if ctx.id == 0 {
//                     let err = CorError {
//                         msg: "Invalid ID".to_string(),
//                         code: ErrorCode::ValidationFailed,
//                         field: FieldName::Title,
//                         error: None,
//                     };
//                     ctx.fail(err);
//                 }
//             }),
//             cor_handler!("Finalize", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 ctx.set_status(CorStatus::Finishing);
//             })
//         }
//     );

//     let wctx = Arc::new(Mutex::new(TestContext {
//         id: 1,
//         status: CorStatus::None,
//         errors: vec![],
//     }));

//     chain.process(wctx.clone()).await;

//     let ctx = wctx.lock().unwrap();
//     assert_eq!(ctx.status, CorStatus::Finishing);
//     assert!(ctx.errors.is_empty());
// }

// // Дополнительный тест для ситуаций с несколькими ошибками
// #[tokio::test]
// async fn test_multiple_errors() {
//     let chain = cor_create!(
//         "Multiple Errors", TestContext,
//         {
//             cor_handler!("Initialize", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 ctx.set_status(CorStatus::Running);
//             }),
//             cor_handler!("Validate", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 if ctx.id == 0 {
//                     let err = CorError {
//                         msg: "Invalid ID".to_string(),
//                         code: ErrorCode::ValidationFailed,
//                         field: FieldName::Title,
//                         error: None,
//                     };
//                     ctx.fail(err);
//                 }
//             }),
//             cor_handler!("CheckPrice", TestContext, |wctx: CtxWrapper<TestContext>| async move {
//                 let mut ctx = wctx.lock().unwrap();
//                 if ctx.id % 2 == 0 {
//                     let err = CorError {
//                         msg: "Even IDs are not allowed".to_string(),
//                         code: ErrorCode::InternalError,
//                         field: FieldName::Description,
//                         error: None,
//                     };
//                     ctx.fail(err);
//                 }
//             })
//         }
//     );

//     let wctx = Arc::new(Mutex::new(TestContext {
//         id: 0,
//         status: CorStatus::None,
//         errors: vec![],
//     }));

//     chain.process(wctx.clone()).await;

//     let ctx = wctx.lock().unwrap();
//     assert_eq!(ctx.status, CorStatus::Failing);
//     assert_eq!(ctx.errors.len(), 2);
//     assert_eq!(ctx.errors[0].msg, "Invalid ID");
//     assert_eq!(ctx.errors[1].msg, "Even IDs are not allowed");
// }
