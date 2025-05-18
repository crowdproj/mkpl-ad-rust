use crate::gen_enum_converter;

gen_enum_converter! {
    api_v1::models::AdVisibility => common::mkpl_ad::MkplAdVisibility,
    converter: VisibilityConverter,
    mappings: {
        api_v1::models::AdVisibility::Public => common::mkpl_ad::MkplAdVisibility::Public,
        api_v1::models::AdVisibility::OwnerOnly => common::mkpl_ad::MkplAdVisibility::Owner,
        api_v1::models::AdVisibility::RegisteredOnly => common::mkpl_ad::MkplAdVisibility::Group
        // None => common::mkpl_ad::MkplAdVisibility::None
    },
    default: common::mkpl_ad::MkplAdVisibility::None
}
