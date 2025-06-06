use common::mkpl_ad::{MkplAd, MkplAdType, MkplAdVisibility};
use common::mkpl_ad_id::MkplAdId;
use common::mkpl_ad_lock::MkplAdLock;
use common::mkpl_ad_product_id::MkplAdProductId;

pub struct StubsMkplAd;

#[allow(dead_code)]
impl StubsMkplAd {
    pub fn case1() -> MkplAd {
        MkplAd {
            id: MkplAdId::from("ad-1234567"),
            title: "title1".to_string(),
            description: "description1".to_string(),
            ad_type: MkplAdType::Supply,
            visibility: MkplAdVisibility::Public,
            product_id: MkplAdProductId::from("product-123456"),
            lock: MkplAdLock::from("lock-1234"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stubs_mkpl_ad() {
        let ad = StubsMkplAd::case1();
        assert_eq!(ad.id.get(), "ad-1234567");
        assert_eq!(ad.title, "title1");
        assert_eq!(ad.description, "description1");
        assert_eq!(ad.ad_type, MkplAdType::Supply);
        assert_eq!(ad.visibility, MkplAdVisibility::Public);
        assert_eq!(ad.product_id.get(), "product-123456");
        assert_eq!(ad.lock.get(), "lock-1234");
    }
}
