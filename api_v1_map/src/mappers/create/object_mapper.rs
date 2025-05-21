use api_v1::models::{AdCreateObject, AdVisibility, DealSide};
use common::mkpl_ad::*;
use common::{mkpl_ad_id::MkplAdId, mkpl_ad_lock::MkplAdLock, mkpl_ad_product_id::MkplAdProductId};

use crate::mappers::base::{
    AdTypeConverter, ProductIdConverter, StringFieldConverter, VisibilityConverter,
};

#[derive(Debug)]
pub struct AdCreateObjectMapper;

impl AdCreateObjectMapper {
    pub fn from_api(value: &Option<AdCreateObject>) -> MkplAd {
        match value {
            Some(api_obj) => MkplAd {
                id: MkplAdId::none(),
                title: StringFieldConverter::from_api(&api_obj.title),
                description: StringFieldConverter::from_api(&api_obj.description),
                ad_type: AdTypeConverter::from_api(&api_obj.ad_type),
                visibility: VisibilityConverter::from_api(&api_obj.visibility),
                product_id: ProductIdConverter::from_api(&api_obj.product_id),
                lock: MkplAdLock::none(),
            },
            None => MkplAd::none(),
        }
    }

    pub fn to_api(mkpl_ad: &MkplAd) -> Option<AdCreateObject> {
        Some(AdCreateObject {
            title: StringFieldConverter::to_api(&mkpl_ad.title),
            description: StringFieldConverter::to_api(&mkpl_ad.description),
            ad_type: AdTypeConverter::to_api(&mkpl_ad.ad_type),
            visibility: VisibilityConverter::to_api(&mkpl_ad.visibility),
            product_id: ProductIdConverter::to_api(&mkpl_ad.product_id),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    const TEST_TITLE: &str = "Test Ad";
    const TEST_DESCRIPTION: &str = "Test Description";
    const TEST_PRODUCT_ID: &str = "TestProductId";

    static API_OBJ: Lazy<AdCreateObject> = Lazy::new(|| AdCreateObject {
        title: Some(TEST_TITLE.to_string()),
        description: Some(TEST_DESCRIPTION.to_string()),
        ad_type: Some(DealSide::Supply),
        visibility: Some(AdVisibility::Public),
        product_id: Some(TEST_PRODUCT_ID.to_string()),
    });

    #[test]
    fn test_full_conversion_cycle() {
        let create_obj: AdCreateObject = API_OBJ.clone();
        let mkpl = AdCreateObjectMapper::from_api(&Some(create_obj));

        assert_eq!(TEST_TITLE, mkpl.title);
        assert_eq!(TEST_DESCRIPTION, mkpl.description);
        assert_eq!(MkplAdType::Supply, mkpl.ad_type);
        assert_eq!(MkplAdVisibility::Public, mkpl.visibility);
        assert_eq!(MkplAdLock::none(), mkpl.lock);
        assert_eq!(MkplAdProductId::from(TEST_PRODUCT_ID), mkpl.product_id);

        let converted_back = AdCreateObjectMapper::to_api(&mkpl).unwrap();

        assert_eq!(TEST_TITLE, converted_back.title.unwrap());
        assert_eq!(TEST_DESCRIPTION, converted_back.description.unwrap());
        assert_eq!(API_OBJ.ad_type, converted_back.ad_type);
        assert_eq!(API_OBJ.visibility, converted_back.visibility);
        assert_eq!(API_OBJ.product_id, converted_back.product_id);
    }
}
