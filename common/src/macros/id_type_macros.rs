#[macro_export]
macro_rules! gen_id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn get(&self) -> &String {
                &self.0
            }

            pub fn none() -> Self {
                Self("".to_owned())
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&String> for $name {
            fn from(s: &String) -> Self {
                Self(String::from(s))
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_none() {
                assert_eq!($name::none().get(), "");
            }

            #[test]
            fn test_from_str() {
                assert_eq!($name::from("xx").get(), "xx");
            }

            #[test]
            fn test_from_string() {
                assert_eq!($name::from("xx".to_string()).get(), "xx");
            }

            #[test]
            fn test_from_refstring() {
                let s = "xx".to_string();
                assert_eq!($name::from(&s).get(), "xx");
            }
        }
    };
}
