use api_v1::models::AdReadObject;
use common::*;

use crate::mappers::base::AdIdMapper;

#[derive(Debug)]
pub struct AdReadObjectMapper;

impl AdReadObjectMapper {
    pub fn from_api(value: &Option<AdReadObject>) -> MkplAd {
        match value {
            Some(api_obj) => MkplAd {
                id: AdIdMapper::from_api(&api_obj.id),
                ..MkplAd::new()
            },
            None => MkplAd::none(),
        }
    }

    pub fn to_api(mkpl_ad: &MkplAd) -> Option<AdReadObject> {
        Some(AdReadObject {
            id: AdIdMapper::to_api(&mkpl_ad.id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stubs::StubsMkplAd;

    #[test]
    fn test_full_conversion_cycle() {
        let mkpl = StubsMkplAd::case1();
        let api = AdReadObjectMapper::to_api(&mkpl).unwrap();

        assert_eq!(mkpl.id.get(), api.id.as_ref().unwrap());

        let converted_back = AdReadObjectMapper::from_api(&Some(api));

        assert_eq!(mkpl.id, converted_back.id.into());
    }
}
