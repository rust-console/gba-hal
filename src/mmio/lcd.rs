
use super::*;

/// The core display control register.
///
/// Sets the visual mode as well as which layers to display.
pub const DISPCNT: VolAddress<DisplayControl> = unsafe { VolAddress::new(0x400_0000) };

/// Display status and interrupt control register.
///
/// This is only partly read/write, some fields are read only, see the setting
/// type for more info.
pub const DISPSTAT: VolAddress<DisplayStatus> = unsafe { VolAddress::new(0x400_0004) };

/// The current scanline being processed.
///
/// 160 and above are virtual scanlines used during VBlank.
pub const VCOUNT: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x400_0006) };

/// Configuration for BG0
pub const BG0CNT: VolAddress<BackgroundControl> = unsafe { VolAddress::new(0x400_0008) };

/// Configuration for BG1
pub const BG1CNT: VolAddress<BackgroundControl> = unsafe { VolAddress::new(0x400_000A) };

/// Configuration for BG2
pub const BG2CNT: VolAddress<BackgroundControl> = unsafe { VolAddress::new(0x400_000C) };

/// Configuration for BG3
pub const BG3CNT: VolAddress<BackgroundControl> = unsafe { VolAddress::new(0x400_000E) };

/// Leftmost visible BG0 pixel (9-bits, tiled mode only).
pub const BG0HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_0010) };

/// Topmost visible BG0 pixel (9-bits, tiled mode only).
pub const BG0VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_0012) };

/// Leftmost visible BG1 pixel (9-bits, tiled mode only).
pub const BG1HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_0014) };

/// Topmost visible BG1 pixel (9-bits, tiled mode only).
pub const BG1VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_0016) };

/// Leftmost visible BG2 pixel (9-bits, tiled mode only).
pub const BG2HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_0018) };

/// Topmost visible BG2 pixel (9-bits, tiled mode only).
pub const BG2VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_001A) };

/// Leftmost visible BG3 pixel (9-bits, tiled mode only).
pub const BG3HOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_001C) };

/// Topmost visible BG3 pixel (9-bits, tiled mode only).
pub const BG3VOFS: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x400_001E) };

/// BG2 Affine Parameter A
pub const BG2PA: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0020) };

/// BG2 Affine Parameter B
pub const BG2PB: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0022) };

/// BG2 Affine Parameter C
pub const BG2PC: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0024) };

/// BG2 Affine Parameter D
pub const BG2PD: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0026) };

/// BG2 Reference X coordinate
pub const BG2X: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x400_0028) };

/// BG2 Reference Y coordinate
pub const BG2Y: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x400_002C) };

/// BG3 Affine Parameter A
pub const BG3PA: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0030) };

/// BG3 Affine Parameter B
pub const BG3PB: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0032) };

/// BG3 Affine Parameter C
pub const BG3PC: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0034) };

/// BG3 Affine Parameter D
pub const BG3PD: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x400_0036) };

/// BG3 Reference X coordinate
pub const BG3X: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x400_0038) };

/// BG3 Reference Y coordinate
pub const BG3Y: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x400_003C) };

/// Horizontal position of Win0
pub const WIN0H: WOVolAddress<WindowHorizontal> = unsafe { WOVolAddress::new(0x400_0040) };

/// Horizontal position of Win1
pub const WIN1H: WOVolAddress<WindowHorizontal> = unsafe { WOVolAddress::new(0x400_0042) };

/// Vertical position of Win0
pub const WIN0V: WOVolAddress<WindowVertical> = unsafe { WOVolAddress::new(0x400_0044) };

/// Vertical position of Win1
pub const WIN1V: WOVolAddress<WindowVertical> = unsafe { WOVolAddress::new(0x400_0046) };

/// Controls the appearance inside of Window0 and Window1
pub const WININ: VolAddress<WindowIn> = unsafe { VolAddress::new(0x400_0048) };

/// Controls the appearance Outside of Windows and of the OBJ Window
pub const WINOUT: VolAddress<WindowOut> = unsafe { VolAddress::new(0x400_004A) };

/// Allows control of the mosaic effect.
pub const MOSAIC: WOVolAddress<Mosaic> = unsafe { WOVolAddress::new(0x400_004C) };

/// Overall configuration of color blending (see also `BLDALPHA` and `BLDY`)
pub const BLDCNT: VolAddress<BlendControl> = unsafe { VolAddress::new(0x400_0050) };

/// Configures alpha blending.
pub const BLDALPHA: VolAddress<BlendAlpha> = unsafe { VolAddress::new(0x400_0052) };

/// Configures brightness blending.
pub const BLDY: WOVolAddress<BlendBrightness> = unsafe { WOVolAddress::new(0x400_0054) };
