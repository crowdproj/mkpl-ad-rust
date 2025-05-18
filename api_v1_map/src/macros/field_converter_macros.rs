#[macro_export]
macro_rules! impl_string_field_converter {
    ($field:ident) => {
        pub fn $field(&self) -> String {
            self.0.$field.as_deref().unwrap_or_default().to_string()
        }
    };
    ($($field:ident),+) => {
        $(impl_string_field_converter!($field);)+
    };
}
