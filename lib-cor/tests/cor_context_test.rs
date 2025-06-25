use cor::{cor_context, CorContext, CorError, CorStatus};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorCode {
    ValidationFailed,
    MissingField,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldName {
    Title,
    Price,
    Description,
}

// Используем правильный синтаксис без фигурных скобок
cor_context!(
    TestContext,
    ErrorCode,
    FieldName,
    id: u32 = 1,
    name: String = "Test".to_string()
);

#[test]
fn test_cor_context_macro_and_trait() {
    let ctx = TestContext::new();
    let wctx = ctx.shared();

    TestContext::with(wctx.clone(), |inner| {
        let err = CorError {
            msg: "Invalid ID".to_string(),
            code: ErrorCode::ValidationFailed,
            field: FieldName::Title,
            error: None,
        };
        inner.push_error(err);
        inner.set_status(CorStatus::Failing);
    });

    TestContext::with(wctx, |inner| {
        assert_eq!(inner.status, CorStatus::Failing);
        assert!(!inner.errors.is_empty());

        let error = &inner.errors[0];
        assert_eq!(error.code, ErrorCode::ValidationFailed);
        assert_eq!(error.field, FieldName::Title);
        assert_eq!(error.msg, "Invalid ID");

        // Проверяем дополнительные поля
        assert_eq!(inner.id, 1);
        assert_eq!(inner.name, "Test");
    });
}
