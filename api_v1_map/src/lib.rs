#[macro_use]
mod macros;

mod mappers;

pub use crate::mappers::create::request_mapper::AdCreateRequestMapper as AdCreateRequestMapperV1;
pub use crate::mappers::create::response_mapper::AdCreateResponseMapper as AdCreateResponseMapperV1;
pub use crate::mappers::read::request_mapper::AdReadRequestMapper as AdReadRequestMapperV1;
pub use crate::mappers::read::response_mapper::AdReadResponseMapper as AdReadResponseMapperV1;
