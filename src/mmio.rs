//! The addresses of the GBA's Memory Mapped IO (MMIO).
//!
//! This module is only generated if you're compiling for the GBA or if you
//! enable a special override to force it into being (which is how it shows up
//! on docs.rs).
//!
//! # Safety
//!
//! If your program isn't running on a GBA, it's Undefined Behavior to actually
//! read or write using any address listed in this module.

use crate::data::*;
use typenum::consts::{U128, U256, U32, U8};
use voladdress::{
  read_only::ROVolAddress, write_only::WOVolAddress, VolAddress, VolBlock, VolSeries,
};

mod lcd;
pub use lcd::*;

mod sound;
pub use sound::*;

mod dma;
pub use dma::*;

mod timers;
pub use timers::*;

mod keypad;
pub use keypad::*;

mod irq;
pub use irq::*;

/// Background colors.
///
/// The 0th slot of this is the "backdrop" color, which is displayed in a pixel
/// if nothing else displays in a pixel.
pub const PALRAM_BG: VolBlock<Color, U256> = unsafe { VolBlock::new(0x500_0000) };

/// Object colors.
pub const PALRAM_OBJ: VolBlock<Color, U256> = unsafe { VolBlock::new(0x500_0200) };

/// OBJ Attribute 0 data series.
pub const OBG_ATTRS_0: VolSeries<OBJAttr0, U128, U8> = unsafe { VolSeries::new(0x700_0000) };

/// OBJ Attribute 1 data series.
pub const OBG_ATTRS_1: VolSeries<OBJAttr1, U128, U8> = unsafe { VolSeries::new(0x700_0002) };

/// OBJ Attribute 2 data series.
pub const OBG_ATTRS_2: VolSeries<OBJAttr2, U128, U8> = unsafe { VolSeries::new(0x700_0004) };

/// Affine Parameter A data series.
pub const AFFINE_PARAMS_A: VolSeries<i16, U32, U32> = unsafe { VolSeries::new(0x700_0006) };

/// Affine Parameter B data series.
pub const AFFINE_PARAMS_B: VolSeries<i16, U32, U32> = unsafe { VolSeries::new(0x700_000E) };

/// Affine Parameter C data series.
pub const AFFINE_PARAMS_C: VolSeries<i16, U32, U32> = unsafe { VolSeries::new(0x700_0016) };

/// Affine Parameter D data series.
pub const AFFINE_PARAMS_D: VolSeries<i16, U32, U32> = unsafe { VolSeries::new(0x700_001E) };

/// The start of VRAM.
///
/// Depending on what display mode is currently set there's different ways that
/// your program should interpret the VRAM space. Accordingly, we give the raw
/// value as just being a `usize`. Specific video mode types then wrap this as
/// being the correct thing.
pub const VRAM_BASE_USIZE: usize = 0x600_0000;

/// If a mode has more than one page, add this much to the base value to get the
/// page 1 address.
pub const PAGE1_OFFSET: usize = 0xA000;
