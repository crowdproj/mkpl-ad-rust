use crate::mkpl_ad_id::MkplAdId;
use crate::mkpl_ad_lock::MkplAdLock;
use crate::mkpl_ad_product_id::MkplAdProductId;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
pub struct MkplAd {
    /// Идентификатор объявления
    pub id: MkplAdId,
    /// Заголовок объявления
    pub title: String,
    /// Описание объявления
    pub description: String,
    pub ad_type: MkplAdType,
    pub visibility: MkplAdVisibility,
    /// Идентификатор модели продукта, к которому относится объявление
    pub product_id: MkplAdProductId,
    /// Версия оптимистичной блокировки
    pub lock: MkplAdLock,
}

impl MkplAd {
    pub fn new() -> MkplAd {
        MKPL_AD_NONE.clone()
    }

    pub fn none() -> MkplAd {
        MKPL_AD_NONE.clone()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MkplAdType {
    None,
    Supply,
    Demand,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MkplAdVisibility {
    None,
    Owner,
    Group,
    Public,
}

lazy_static! {
    #[allow(dead_code)]
    pub static ref MKPL_AD_NONE: MkplAd = MkplAd {
        id: MkplAdId::none(),
        title: "".to_owned(),
        description: "".to_owned(),
        ad_type: MkplAdType::None,
        visibility: MkplAdVisibility::None,
        product_id: MkplAdProductId::none(),
        lock: MkplAdLock::none(),
    };
}

// impl Default for MkplAd {
//     fn default() -> MkplAd {
//         MkplAd {
//             title: None,
//             description: None,
//             ad_type: None,
//             visibility: None,
//             product_id: None,
//             id: None,
//             lock: None,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mkpl_ad_none() {
        assert_eq!(MKPL_AD_NONE.id.get(), "");
        // assert_eq!(MKPL_AD_NONE.title, "");
        // assert_eq!(MKPL_AD_NONE.description, "");
        // assert_eq!(MKPL_AD_NONE.ad_type, MkplAdType::None);
        // assert_eq!(MKPL_AD_NONE.visibility, MkplAdVisibility::None);
        // assert_eq!(MKPL_AD_NONE.product_id.val, "");
        // assert_eq!(MKPL_AD_NONE.lock.val, "");
    }
}
