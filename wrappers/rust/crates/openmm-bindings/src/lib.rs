//! Unsafe Rust bindings to the C++ OpenMM library
//!
//! # bindgen
//!
//! Bindings are generated automatically by the [bindgen](https://crates.io/crates/bindgen) crate
//! when the crate is built.
//!
//! ## Notes
//! - The `-fkeep-inline-functions` flag is passed the the C++ compiler so that inline functions
//! can be used by Rust code. This may cause performance problems — we'll have to see.
//! - The `improper_ctypes` lint is disabled, as several functions return opaque hidden types that
//! appear to Rust as arrays.

// use cxx::{type_id, ExternType};

pub mod c_bindings {
    mod bindings {
        #![allow(
            non_upper_case_globals,
            non_camel_case_types,
            non_snake_case,
            dead_code,
            improper_ctypes
        )]

        include!(concat!(env!("OUT_DIR"), "/c_bindings.rs"));
    }

    pub use bindings::root::*;
}

pub mod cpp_bindings {
    mod bindings {
        #![allow(
            non_upper_case_globals,
            non_camel_case_types,
            non_snake_case,
            dead_code,
            improper_ctypes
        )]

        include!(concat!(env!("OUT_DIR"), "/cpp_bindings.rs"));
    }

    pub use bindings::root::OpenMM::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_linked() {
        unsafe {
            let mut sys = System::new();
            let idx = sys.addParticle(1.008);
            assert_eq!(1.008, sys.getParticleMass(idx));
            sys.destruct();
        }
    }
}
