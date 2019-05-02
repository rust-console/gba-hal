[![License:Apache2](https://img.shields.io/badge/License-Apache2-green.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![travis.ci](https://travis-ci.org/rust-console/gba-hal.svg?branch=master)](https://travis-ci.org/rust-console/gba-hal)
[![crates.io](https://img.shields.io/crates/v/gba-hal.svg)](https://crates.io/crates/gba-hal)
[![docs.rs](https://docs.rs/gba-hal/badge.svg)](https://docs.rs/gba-hal/)

# gba-hal

This is a crate for the Game Boy Advance Hardware Abstraction Layer. It consists
of types that the memory mapped IO uses as well as the MMIO addresses that they
go with.

* The types can of course safely be used on any device.

* The MMIO addresses can only be safely used on a GBA, and any attempt to use
  them elsewhere is UB. The addresses are kept in a separate module that is only
  compiled in when compiling for GBA (using the compilation system suggested by
  the [gba](https://github.com/rust-console/gba) crate), though there is also a
  special override flag so that the address docs can be generated for docs.rs.
  You should not use the override flag otherwise.
