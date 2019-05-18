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
use voladdress::{read_only::ROVolAddress, write_only::WOVolAddress, VolAddress, VolBlock};

mod lcd;
pub use lcd::*;

mod sound;
pub use sound::*;

mod dma;
pub use dma::*;

mod timers;
pub use timers::*;
