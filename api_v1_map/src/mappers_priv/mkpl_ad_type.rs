use crate::gen_enum_converter;

gen_enum_converter! {
    api_v1::models::DealSide => common::mkpl_ad::MkplAdType,
    converter: AdTypeConverter,
    mappings: {
        api_v1::models::DealSide::Supply => common::mkpl_ad::MkplAdType::Supply,
        api_v1::models::DealSide::Demand => common::mkpl_ad::MkplAdType::Demand
    },
    default: common::mkpl_ad::MkplAdType::None
}
