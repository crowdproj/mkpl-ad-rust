#[macro_export]
macro_rules! cor_chain {
    ($name:expr, $ctx_type:ty, {$($step:expr),*}) => {{
        let steps = vec![
            $(Box::new($step) as Box<dyn $crate::CorStep<$ctx_type>>),*
        ];
        $crate::Cor::new($name, steps)
    }};
}

#[macro_export]
macro_rules! cor_handler {
    ($name:expr, $f:expr) => {{
        $crate::CorClosureStep::new($name, $f)
    }};
}

#[macro_export]
macro_rules! cor_context {
    ($name:ident, $ec:ty, $fld:ty, $($field:ident: $ty:ty = $init:expr),* $(,)?) => {
        use std::sync::{Arc, Mutex};

        #[derive(Debug)]
        pub struct $name {
            errors: Vec<$crate::CorError<$ec, $fld>>,
            status: $crate::CorStatus,
            $(pub $field: $ty,)*
        }

        impl $crate::ContextTypes for $name {
            type ErrorCode = $ec;
            type FieldName = $fld;
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    errors: Vec::new(),
                    status: $crate::CorStatus::None,
                    $($field: $init,)*
                }
            }

            pub fn shared(self) -> Arc<Mutex<Self>> {
                Arc::new(Mutex::new(self))
            }

            pub fn fail(&mut self, msg: &str, code: $ec, field: $fld) {
                self.push_error($crate::CorError {
                    msg: msg.to_string(),
                    code,
                    field,
                    error: None,
                });
                self.set_status($crate::CorStatus::Failing);
            }
        }

        impl $crate::CorContext for $name {
            fn new() -> Self {
                Self::new()
            }

            fn push_error(&mut self, err: $crate::CorError<Self::ErrorCode, Self::FieldName>) {
                self.errors.push(err);
            }

            fn set_status(&mut self, status: $crate::CorStatus) {
                self.status = status;
            }

            fn errors(&self) -> &Vec<$crate::CorError<Self::ErrorCode, Self::FieldName>> {
                &self.errors
            }

            fn status(&self) -> $crate::CorStatus {
                self.status
            }
        }
    };
}
