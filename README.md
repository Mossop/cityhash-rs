Provides rust bindings for the [CityHash algorithm](https://github.com/google/cityhash).

[![Build Status](https://travis-ci.org/Mossop/cityhash-rs.svg?branch=master)](https://travis-ci.org/Mossop/cityhash-rs)
[![Crates.io](https://img.shields.io/crates/v/cityhash.svg)](https://crates.io/crates/cityhash)
[![Released API docs](https://docs.rs/cityhash/badge.svg)](https://docs.rs/cityhash)

This crate contains multiple versions of the CityHash algorithm since the
hash returned varies depending on the version used. Each version is in a
versioned sub-module with `.` characters replaced by `_`.

If you want what you're building to have a stable hashing function then use
the functions from one of the versioned sub-modules. Otherwise the top-level
module will always expose the functions from the latest included version.

Currently only the 64-bit and 128-bit hashing functions with no seeds are
exposed, but it would be trivial to add them should someone feel like it!

This crate also contains a small command line interface for hashing strings.
The help text is fairly explanatory.
