//! This project has not been documented yet
mod bindings {
    #![allow(
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        dead_code
    )]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use crate::bindings::root::OpenMM::*;
