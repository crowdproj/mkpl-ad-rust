use crate::gen_response_mapper;

gen_response_mapper!(
    AdUpdateResponseMapper,
    api_v1::models::AdUpdateResponse,
    crate::mappers::base::DISCRIMINATOR_UPDATE,
    biz_common::models::MkplAdCommand::Update
);
