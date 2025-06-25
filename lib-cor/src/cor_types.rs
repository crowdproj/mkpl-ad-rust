pub trait ContextTypes {
    type ErrorCode: std::fmt::Debug + Send + Sync + 'static;
    type FieldName: std::fmt::Debug + Send + Sync + 'static;
}
