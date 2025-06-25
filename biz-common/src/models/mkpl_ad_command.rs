#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MkplAdCommand {
    None,
    Create,
    Read,
    Update,
    Delete,
    Search,
    Offers,
}
