#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod libusbgx_sys {
    use super::*;
    use std::mem;

    #[test]
    fn create_empty_instance() {
        assert_eq!("TODO: Not implemented", "");
    }

    #[test]
    fn create_configuration_0() {
        assert_eq!("TODO: Not implemented", "");
    }

    #[test]
    fn create_configuration_1() {
        assert_eq!("TODO: Not implemented", "");
    }

    #[test]
    fn create_configuration_from_file() {
        assert_eq!("TODO: Not implemented", "");
    }
}
