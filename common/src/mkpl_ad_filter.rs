use crate::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct MkplAdFilter {
    pub query: Option<String>,
    /// Идентификатор объявления
    pub ad_ids: HashSet<MkplAdId>,
    /// Идентификатор владельца объявления
    pub owner_ids: HashSet<MkplAdUserId>,
    /// Заголовок объявления
    pub title: Option<String>,
    /// Описание объявления
    pub description: Option<String>,
    pub ad_type: Option<MkplAdType>,
    pub visibilities: HashSet<MkplAdVisibility>,
    /// Идентификатор модели продукта, к которому относится объявление
    pub product_ids: HashSet<MkplAdProductId>,
}

impl MkplAdFilter {
    pub fn new() -> MkplAdFilter {
        Self::none()
    }

    pub fn none() -> MkplAdFilter {
        MkplAdFilter {
            query: None,
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_mkpl_ad_none() {
//         assert_eq!(MKPL_AD_NONE.id.get(), "");
//     }
// }
