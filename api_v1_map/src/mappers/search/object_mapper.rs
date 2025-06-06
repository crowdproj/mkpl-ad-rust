use api_v1::models::AdSearchFilter;
use common::*;

#[derive(Debug)]
pub struct AdSearchFilterMapper;

impl AdSearchFilterMapper {
    pub fn from_api(value: &Option<AdSearchFilter>) -> MkplAdFilter {
        match value {
            Some(api_obj) => MkplAdFilter {
                query: api_obj
                    .search_string
                    .as_ref()
                    .map_or(None, |s| Some(s.to_string())),
                ..MkplAdFilter::none()
            },
            None => MkplAdFilter::none(),
        }
    }

    pub fn to_api(mkpl_ad: &MkplAdFilter) -> Option<AdSearchFilter> {
        Some(AdSearchFilter {
            search_string: mkpl_ad.query.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stubs::StubsMkplAdFilter;

    #[test]
    fn test_full_conversion_cycle() {
        let mkpl = StubsMkplAdFilter::case1();
        let api = AdSearchFilterMapper::to_api(&mkpl).unwrap();

        assert_eq!("title1", api.search_string.as_ref().unwrap().as_str());

        let converted_back = AdSearchFilterMapper::from_api(&Some(api));

        assert_eq!(mkpl.query.unwrap(), converted_back.query.unwrap());
    }
}
