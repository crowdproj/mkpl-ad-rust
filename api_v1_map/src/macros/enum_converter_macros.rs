#[macro_export]
macro_rules! gen_enum_converter {
    (
        $src_enum:path => $dest_enum:path,
        converter: $converter:ident,
        mappings: { $($src_variant:path => $dest_variant:path),+ },
        default: $default:expr
    ) => {
        #[derive(Debug, PartialEq)]
        pub struct $converter;

        impl $converter {
            pub fn from_api(api_value: &Option<$src_enum>) -> $dest_enum {
                match api_value {
                    $(Some($src_variant) => $dest_variant,)+
                    _ => $default,
                }
            }

            pub fn to_api(internal_value: &$dest_enum) -> Option<$src_enum> {
                match internal_value {
                    $($dest_variant => Some($src_variant),)+
                    _ => None,
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_from_known_variants() {
                $(
                    let input = Some($src_variant);
                    let result = <$converter>::from_api(&input);
                    assert_eq!(result, $dest_variant, "Failed converting {:?} to {:?}", input, $dest_variant);
                )+
            }

            #[test]
            fn test_to_known_variants() {
                $(
                    let input = $dest_variant;
                    let result = <$converter>::to_api(&input);
                    assert_eq!(result, Some($src_variant), "Failed converting {:?} back to {:?}", input, $src_variant);
                )+
            }

            #[test]
            fn test_from_unknown() {
                let input: Option<$src_enum> = None;
                let result = <$converter>::from_api(&input);
                assert_eq!(result, $default, "Failed using default value for unknown variant");
            }

            #[test]
            fn test_to_unknown() {
                let input = $default;
                let result = <$converter>::to_api(&input);
                assert_eq!(result, None, "Expected None when converting unknown target variant");
            }
        }
    };
}
