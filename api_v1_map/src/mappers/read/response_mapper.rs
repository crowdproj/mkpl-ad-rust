use crate::gen_response_mapper;

gen_response_mapper!(
    AdReadResponseMapper,
    api_v1::models::AdReadResponse,
    crate::mappers::base::DISCRIMINATOR_READ,
    biz_common::models::MkplAdCommand::Read
);
