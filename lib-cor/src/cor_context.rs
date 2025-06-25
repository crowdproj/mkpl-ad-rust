use crate::CorError;
use crate::CorStatus;
use std::sync::{Arc, Mutex};

pub trait CorContext<E, F>: Sized {
    // Методы для работы с контекстом
    fn new() -> Self;
    fn push_error(&mut self, err: CorError<E, F>);
    fn set_status(&mut self, status: CorStatus);
    fn fail(&mut self, err: CorError<E, F>) {
        self.push_error(err);
        self.set_status(CorStatus::Failing);
    }

    // Создание потокобезопасного контекста
    fn shared(self) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new()))
    }

    // Параллельное изменение данных
    fn with<T>(ctx: Arc<Mutex<Self>>, f: T)
    where
        T: FnOnce(&mut Self) + Send + 'static,
    {
        let mut guard = ctx.lock().unwrap();
        f(&mut *guard);
    }
}
