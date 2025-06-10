use crate::mappers::base::AdIdMapper;
use api_v1::models::AdOffersObject;
use common::*;

#[derive(Debug)]
pub struct AdOffersObjectMapper;

impl AdOffersObjectMapper {
    pub fn from_api(value: &Option<AdOffersObject>) -> MkplAd {
        match value {
            Some(api_obj) => MkplAd {
                id: AdIdMapper::from_api(&api_obj.id),
                ..MkplAd::new()
            },
            None => MkplAd::none(),
        }
    }

    pub fn to_api(mkpl_ad: &MkplAd) -> Option<AdOffersObject> {
        Some(AdOffersObject {
            id: AdIdMapper::to_api(&mkpl_ad.id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stubs::StubsMkplAdFilter;

    #[test]
    fn test_full_conversion_cycle() {
        let mkpl = StubsMkplAd::case1();
        let api = AdOffersObjectMapper::to_api(&mkpl).unwrap();

        assert_eq!(mkpl.id.get(), api.id.as_ref().unwrap());

        let converted_back = AdOffersObjectMapper::from_api(&Some(api));

        assert_eq!(mkpl.id, converted_back.id.into());
    }
}
