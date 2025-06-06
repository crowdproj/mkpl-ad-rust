use crate::gen_response_mapper;

gen_response_mapper!(
    AdDeleteResponseMapper,
    api_v1::models::AdDeleteResponse,
    crate::mappers::base::DISCRIMINATOR_DELETE,
    biz_common::models::MkplAdCommand::Delete
);
