use api_v1::models::AdDeleteObject;
use common::*;

use crate::mappers::base::{AdIdMapper, AdLockMapper};

#[derive(Debug)]
pub struct AdDeleteObjectMapper;

impl AdDeleteObjectMapper {
    pub fn from_api(value: &Option<AdDeleteObject>) -> MkplAd {
        match value {
            Some(api_obj) => MkplAd {
                id: AdIdMapper::from_api(&api_obj.id),
                lock: AdLockMapper::from_api(&api_obj.lock),
                ..MkplAd::new()
            },
            None => MkplAd::none(),
        }
    }

    pub fn to_api(mkpl_ad: &MkplAd) -> Option<AdDeleteObject> {
        Some(AdDeleteObject {
            id: AdIdMapper::to_api(&mkpl_ad.id),
            lock: AdLockMapper::to_api(&mkpl_ad.lock),
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
        let api = AdDeleteObjectMapper::to_api(&mkpl).unwrap();

        assert_eq!(mkpl.id.get(), api.id.as_ref().unwrap());
        assert_eq!(mkpl.lock.get(), api.lock.as_ref().unwrap());

        let converted_back = AdDeleteObjectMapper::from_api(&Some(api));

        assert_eq!(mkpl.id, converted_back.id.into());
        assert_eq!(mkpl.lock, converted_back.lock.into());
    }
}
