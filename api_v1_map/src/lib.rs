#[macro_use]
mod macros;

mod mappers;

pub use crate::mappers::create::request_mapper::AdCreateRequestMapper as AdCreateRequestMapperV1;
pub use crate::mappers::create::response_mapper::AdCreateResponseMapper as AdCreateResponseMapperV1;
pub use crate::mappers::delete::request_mapper::AdDeleteRequestMapper as AdDeleteRequestMapperV1;
pub use crate::mappers::delete::response_mapper::AdDeleteResponseMapper as AdDeleteResponseMapperV1;
pub use crate::mappers::offers::request_mapper::AdOffersRequestMapper as AdOffersRequestMapperV1;
pub use crate::mappers::offers::response_mapper::AdOffersResponseMapper as AdOffersResponseMapperV1;
pub use crate::mappers::read::request_mapper::AdReadRequestMapper as AdReadRequestMapperV1;
pub use crate::mappers::read::response_mapper::AdReadResponseMapper as AdReadResponseMapperV1;
pub use crate::mappers::search::request_mapper::AdSearchRequestMapper as AdSearchRequestMapperV1;
pub use crate::mappers::search::response_mapper::AdSearchResponseMapper as AdSearchResponseMapperV1;
pub use crate::mappers::update::request_mapper::AdUpdateRequestMapper as AdUpdateRequestMapperV1;
pub use crate::mappers::update::response_mapper::AdUpdateResponseMapper as AdUpdateResponseMapperV1;
