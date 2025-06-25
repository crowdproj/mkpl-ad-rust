#[macro_export]
// Создание цепочки обработчиков
macro_rules! cor_create {
    ($name:expr, $ctx_type:ty, {$($h:expr),*}) => {{
        let steps = vec![
            $(Box::new($h) as Box<$crate::StepFn<$ctx_type>>),*
        ];
        cor::Cor::new(steps)
    }}
}

#[macro_export]
macro_rules! cor_handler {
    ($desc:expr, $ctx_type:ty, $f:expr) => {{
        move |wrapped_ctx: cor::CtxWrapper<$ctx_type>| -> std::pin::Pin<
            Box<dyn std::future::Future<Output = ()> + Send + Sync + 'static>,
        > { Box::pin($f(wrapped_ctx)) }
    }};
}

#[macro_export]
macro_rules! cor_context {
    ($name:ident, $($field:ident: $ty:ty = $init:expr),* $(,)?) => {
        use std::sync::{Arc, Mutex};

        #[derive(std::fmt::Debug)]
        pub struct $name {
            errors: Vec<cor::CorError<ErrorCode, FieldName>>,
            status: cor::CorStatus,
            $($field: $ty,)*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    errors: Vec::new(),
                    status: cor::CorStatus::None,
                    $($field: $init,)*
                }
            }

            pub fn push_error(&mut self, err: cor::CorError<ErrorCode, FieldName>) {
                self.errors.push(err);
            }

            pub fn set_status(&mut self, status: cor::CorStatus) {
                self.status = status;
            }

            // Добавляем удобный метод для ошибок
            pub fn fail(&mut self, err: cor::CorError<ErrorCode, FieldName>) {
                self.push_error(err);
                self.set_status(cor::CorStatus::Failing);
            }
        }

        impl cor::CorContext<ErrorCode, FieldName> for $name {
            fn new() -> Self {
                Self::new()
            }
            fn push_error(&mut self, err: cor::CorError<ErrorCode, FieldName>) {
                self.push_error(err);
            }

            fn set_status(&mut self, status: cor::CorStatus) {
                self.set_status(status);
            }
        }

        impl $name {
            pub fn shared() -> Arc<Mutex<Self>> {
                Arc::new(Mutex::new(Self::new()))
            }

            pub fn with<F, R>(ctx: Arc<Mutex<Self>>, f: F) -> R
            where
                F: FnOnce(&mut Self) -> R,
            {
                let mut guard = ctx.lock().unwrap();
                f(&mut guard)
            }
        }
    };
}
