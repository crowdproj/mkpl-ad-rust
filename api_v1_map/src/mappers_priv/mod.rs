pub mod mkpl_ad_request_id;
pub mod mkpl_ad_type;
pub mod mkpl_ad_visibility;
pub mod mkpl_lock;
pub mod mkpl_product_id;
pub mod string_fields;

// Реэкспорт для удобства
pub use mkpl_ad_type::AdTypeConverter;
pub use mkpl_ad_visibility::VisibilityConverter;
pub use mkpl_lock::LockConverter;
pub use mkpl_product_id::ProductIdConverter;
pub use string_fields::StringFieldConverter;
