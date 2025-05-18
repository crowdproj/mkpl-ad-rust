#[macro_export]
macro_rules! impl_model_mapper {
    ($wrapper:ident, $target:ty, $($from:path => $to:expr),*) => {
        impl $wrapper {
            pub fn convert(&self) -> $target {
                <$target>::new()
                    $(.with_field($from, $to))*
            }
        }
    };
}
