use crate::gen_response_mapper;

gen_response_mapper!(
    AdCreateResponseMapper,
    api_v1::models::AdCreateResponse,
    crate::mappers::base::DISCRIMINATOR_CREATE,
    biz_common::models::MkplAdCommand::Create
);
