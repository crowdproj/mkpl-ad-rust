use api_v1::models::AdResponseObject;
use common::mkpl_ad::*;

use crate::mappers::base::*;

#[derive(Debug)]
pub struct AdResponseObjectMapper;

impl AdResponseObjectMapper {
    pub fn from_api(value: &Option<AdResponseObject>) -> MkplAd {
        match value {
            Some(api_obj) => MkplAd {
                id: AdIdMapper::from_api(&api_obj.id),
                title: StringFieldConverter::from_api(&api_obj.title),
                description: StringFieldConverter::from_api(&api_obj.description),
                ad_type: AdTypeConverter::from_api(&api_obj.ad_type),
                visibility: VisibilityConverter::from_api(&api_obj.visibility),
                product_id: ProductIdConverter::from_api(&api_obj.product_id),
                lock: AdLockMapper::from_api(&api_obj.lock),
            },
            None => MkplAd::none(),
        }
    }

    pub fn to_api(mkpl_ad: &MkplAd) -> Option<AdResponseObject> {
        Some(AdResponseObject {
            id: AdIdMapper::to_api(&mkpl_ad.id),
            title: StringFieldConverter::to_api(&mkpl_ad.title),
            description: StringFieldConverter::to_api(&mkpl_ad.description),
            ad_type: AdTypeConverter::to_api(&mkpl_ad.ad_type),
            visibility: VisibilityConverter::to_api(&mkpl_ad.visibility),
            product_id: ProductIdConverter::to_api(&mkpl_ad.product_id),
            owner_id: None,
            lock: AdLockMapper::to_api(&mkpl_ad.lock),
            permissions: None,
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
        let api = AdResponseObjectMapper::to_api(&mkpl).unwrap();

        assert_eq!(mkpl.id.get(), api.id.as_ref().unwrap());
        assert_eq!(mkpl.lock.get(), api.lock.as_ref().unwrap());
        assert_eq!(mkpl.title, *api.title.as_ref().unwrap());
        assert_eq!(mkpl.description, *api.description.as_ref().unwrap());
        // assert_eq!(mkpl.ad_type, api.ad_type.as_ref().unwrap());
        // assert_eq!(mkpl.visibility, api.visibility.as_ref().unwrap());
        assert_eq!(mkpl.product_id.get(), api.product_id.as_ref().unwrap());

        let converted_back = AdResponseObjectMapper::from_api(&Some(api));

        assert_eq!(mkpl.id, converted_back.id);
        assert_eq!(mkpl.title, converted_back.title);
        assert_eq!(mkpl.description, converted_back.description);
        assert_eq!(mkpl.ad_type, converted_back.ad_type);
        assert_eq!(mkpl.visibility, converted_back.visibility);
        assert_eq!(mkpl.lock, converted_back.lock);
        assert_eq!(mkpl.product_id, converted_back.product_id);
    }
}
