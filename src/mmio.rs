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
use voladdress::{read_only::ROVolAddress, write_only::WOVolAddress, VolAddress};

/// The core display control register.
///
/// Sets the visual mode as well as which layers to display.
pub const DISPCNT: VolAddress<DisplayControlSetting> = unsafe { VolAddress::new(0x4000000) };

/// Display status and interrupt control register.
///
/// This is only partly read/write, some fields are read only, see the setting
/// type for more info.
pub const DISPSTAT: VolAddress<DisplayStatusSetting> = unsafe { VolAddress::new(0x4000004) };

/// The current scanline being processed.
///
/// 160 and above are virtual scanlines used during VBlank.
pub const VCOUNT: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x4000006) };

/// Configuration for BG0
pub const BG0CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new(0x4000008) };

/// Configuration for BG1
pub const BG1CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new(0x400000A) };

/// Configuration for BG2
pub const BG2CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new(0x400000C) };

/// Configuration for BG3
pub const BG3CNT: VolAddress<BackgroundControlSetting> = unsafe { VolAddress::new(0x400000E) };

/// Leftmost visible BG0 pixel (9-bits, tiled mode only).
pub const BG0HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000010) };

/// Topmost visible BG0 pixel (9-bits, tiled mode only).
pub const BG0VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000012) };

/// Leftmost visible BG1 pixel (9-bits, tiled mode only).
pub const BG1HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000014) };

/// Topmost visible BG1 pixel (9-bits, tiled mode only).
pub const BG1VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000016) };

/// Leftmost visible BG2 pixel (9-bits, tiled mode only).
pub const BG2HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000018) };

/// Topmost visible BG2 pixel (9-bits, tiled mode only).
pub const BG2VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400001A) };

/// Leftmost visible BG3 pixel (9-bits, tiled mode only).
pub const BG3HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400001C) };

/// Topmost visible BG3 pixel (9-bits, tiled mode only).
pub const BG3VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400001E) };

pub const BG2PA: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000020) };

pub const BG2PB: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000022) };

pub const BG2PC: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000024) };

pub const BG2PD: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000026) };

pub const BG2X: WOVolAddress<u32> = unsafe { WOVolAddress::new(0x4000028) };

pub const BG2Y: VolAddress<u32> = unsafe { VolAddress::new(0x400002C) };

pub const BG3PA: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000030) };

pub const BG3PB: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000032) };

pub const BG3PC: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000034) };

pub const BG3PD: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000036) };

pub const BG3X: WOVolAddress<u32> = unsafe { WOVolAddress::new(0x4000038) };

pub const BG3Y: WOVolAddress<u32> = unsafe { WOVolAddress::new(0x400003C) };

pub const WIN0H: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000040) };

pub const WIN1H: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000042) };

pub const WIN0V: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000044) };

pub const WIN1V: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000046) };

pub const WININ: VolAddress<u16> = unsafe { VolAddress::new(0x4000048) };

pub const WINOUT: VolAddress<u16> = unsafe { VolAddress::new(0x400004A) };

/// Allows control of the mosaic effect.
pub const MOSAIC: WOVolAddress<MosaicSetting> = unsafe { WOVolAddress::new(0x400004C) };

pub const BLDCNT: VolAddress<u16> = unsafe { VolAddress::new(0x4000050) };

pub const BLDALPHA: VolAddress<u16> = unsafe { VolAddress::new(0x4000052) };

pub const BLDY: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4000054) };
