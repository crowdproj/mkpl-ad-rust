#[macro_export]
macro_rules! gen_id_type_mapper {
    ($type:ty, $converter:ident) => {
        pub struct $converter;

        impl $converter {
            pub fn from_api(value: &Option<String>) -> $type {
                value.as_ref().map(<$type>::from).unwrap_or(<$type>::none())
            }

            pub fn to_api(value: &$type) -> Option<String> {
                if *value == <$type>::none() {
                    None
                } else {
                    Some(value.get().to_string())
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            const TEST_VALUE: &str = "test_value";

            #[test]
            fn test_from_api() {
                let test_value = Some(TEST_VALUE.to_string());
                let internal_value: $type = $converter::from_api(&test_value);
                assert_eq!(<$type>::from(TEST_VALUE), internal_value);
            }

            #[test]
            fn test_to_api() {
                let api_obj = $converter::to_api(&<$type>::from(TEST_VALUE));
                assert_eq!(Some(TEST_VALUE.to_string()), api_obj);
            }

            #[test]
            fn test_none_conversion() {
                let api_obj: $type = $converter::from_api(&None);
                assert_eq!(<$type>::none(), api_obj);
            }
        }
    };
}

// // Пример использования для MkplAdLock
// generate_converter!(common::mkpl_ad_lock::MkplAdLock, LockConverter);

// // Пример использования для другого типа
// generate_converter!(common::mkpl_ad_id::MkplAdId, IdConverter);
