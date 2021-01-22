# openmm-bindings

[![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/yoshanuikabundi/openmm?label=tag&logo=github&sort=semver)](https://github.com/yoshanuikabundi/openmm)
[![Crates.io](https://img.shields.io/crates/v/openmm.svg)](https://crates.io/crates/openmm)
[![Docs.rs](https://docs.rs/openmm/badge.svg)](https://docs.rs/openmm)

Unsafe Rust bindings to the C++ OpenMM library

## bindgen

Bindings are generated automatically by the [bindgen](https://crates.io/crates/bindgen) crate
when the crate is built.

### Notes
- The `-fkeep-inline-functions` flag is passed the the C++ compiler so that inline functions
can be used by Rust code. This may cause performance problems — we'll have to see.
- The `improper_ctypes` lint is disabled, as several functions return opaque hidden types that
appear to Rust as arrays.
