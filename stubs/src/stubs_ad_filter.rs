use common::*;
use std::collections::HashSet;

pub struct StubsMkplAdFilter;

#[allow(dead_code)]
impl StubsMkplAdFilter {
    pub fn case1() -> MkplAdFilter {
        MkplAdFilter {
            query: Some("title1".to_string()),
            title: None,
            description: None,
            ad_type: None,
            visibilities: HashSet::new(),
            product_ids: HashSet::new(),
            ad_ids: HashSet::new(),
            owner_ids: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stubs_mkpl_ad() {
        let filter = StubsMkplAdFilter::case1();
        assert_eq!(filter.query.unwrap(), "title1");
        assert_eq!(filter.title, None);
        assert_eq!(filter.description, None);
        assert_eq!(filter.ad_type, None);
        assert!(filter.visibilities.is_empty());
        assert!(filter.owner_ids.is_empty());
        assert!(filter.ad_ids.is_empty());
    }
}
