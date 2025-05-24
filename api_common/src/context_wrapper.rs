#[derive(Clone)]
pub struct ContextWrapper<T, C> {
    pub inner: T,
    pub context: C,
    // pub x_span_id: String,
}

impl<T, C> ContextWrapper<T, C> {
    pub fn new(inner: T, context: C) -> Self {
        Self { inner, context }
    }
    pub fn api(&self) -> &T {
        &self.inner
    }
}

// Реализация Deref для доступа к inner
impl<T, C> std::ops::Deref for ContextWrapper<T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
