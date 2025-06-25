use cor::cor_context;
use cor::CorContext;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    ValidationFailed,
    InternalError,
}

#[derive(Debug, Clone, Copy)]
pub enum FieldName {
    Title,
    Description,
}

cor_context!(
    TestContext,
    ErrorCode,
    FieldName,
    id: u32 = 0
);
