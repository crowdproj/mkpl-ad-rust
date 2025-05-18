pub struct StringFieldConverter;

impl StringFieldConverter {
    pub fn from_api(value: &Option<String>) -> String {
        value.clone().unwrap_or_default()
    }

    pub fn to_api(value: &String) -> Option<String> {
        if value.is_empty() {
            None
        } else {
            Some(value.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_from_api() {
        assert_eq!("", StringFieldConverter::from_api(&None));
        assert_eq!(
            "hello",
            StringFieldConverter::from_api(&Some("hello".to_string()))
        );
    }

    #[test]
    fn string_to_api() {
        assert_eq!(None, StringFieldConverter::to_api(&"".to_string()));
        assert_eq!(
            Some("hello".to_string()),
            StringFieldConverter::to_api(&"hello".to_string())
        );
    }
}
