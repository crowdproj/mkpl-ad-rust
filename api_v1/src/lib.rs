// pub mod ad_models {
//     // include!(concat!(env!("OUT_DIR"), "/generates/src/lib.rs"));
//     // include!(concat!(env!("OUT_DIR"), "/generates/src/server/mod.rs"));
//     // include!(concat!(env!("OUT_DIR"), "/generates/src/lib.rs"));
//     include!(concat!(env!("CARGO_MANIFEST_DIR"), "/.generated/src/models.rs"));
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
