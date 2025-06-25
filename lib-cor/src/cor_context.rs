use crate::{ContextTypes, CorError, CorStatus};
use std::sync::{Arc, Mutex};

pub trait CorContext: ContextTypes + Sized + Send + Sync + 'static {
    fn new() -> Self;
    fn push_error(&mut self, err: CorError<Self::ErrorCode, Self::FieldName>);
    fn set_status(&mut self, status: CorStatus);
    fn errors(&self) -> &Vec<CorError<Self::ErrorCode, Self::FieldName>>;
    fn status(&self) -> CorStatus;

    fn apply(self, f: impl FnOnce(&mut Self)) -> Self {
        let mut value = self;
        f(&mut value);
        value
    }

    fn fail(&mut self, err: CorError<Self::ErrorCode, Self::FieldName>) {
        self.push_error(err);
        self.set_status(CorStatus::Failing);
    }

    fn shared(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(self))
    }

    fn with<F, R>(ctx: Arc<Mutex<Self>>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let mut guard = ctx.lock().unwrap();
        f(&mut *guard)
    }

    fn access<F, R>(ctx: Arc<Mutex<Self>>, f: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        let guard = ctx.lock().unwrap();
        f(&*guard)
    }
}
