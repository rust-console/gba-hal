
use super::*;

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

/// BG2 Affine Parameter A
pub const BG2PA: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000020) };

/// BG2 Affine Parameter B
pub const BG2PB: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000022) };

/// BG2 Affine Parameter C
pub const BG2PC: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000024) };

/// BG2 Affine Parameter D
pub const BG2PD: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000026) };

/// BG2 Reference X coordinate
pub const BG2X: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x4000028) };

/// BG2 Reference Y coordinate
pub const BG2Y: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x400002C) };

/// BG3 Affine Parameter A
pub const BG3PA: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000030) };

/// BG3 Affine Parameter B
pub const BG3PB: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000032) };

/// BG3 Affine Parameter C
pub const BG3PC: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000034) };

/// BG3 Affine Parameter D
pub const BG3PD: WOVolAddress<FP_I_7_8> = unsafe { WOVolAddress::new(0x4000036) };

/// BG3 Reference X coordinate
pub const BG3X: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x4000038) };

/// BG3 Reference Y coordinate
pub const BG3Y: WOVolAddress<FP_I_19_8> = unsafe { WOVolAddress::new(0x400003C) };

/// Horizontal position of Win0
pub const WIN0H: WOVolAddress<WindowHorizontalSetting> = unsafe { WOVolAddress::new(0x4000040) };

/// Horizontal position of Win1
pub const WIN1H: WOVolAddress<WindowHorizontalSetting> = unsafe { WOVolAddress::new(0x4000042) };

/// Vertical position of Win0
pub const WIN0V: WOVolAddress<WindowVerticalSetting> = unsafe { WOVolAddress::new(0x4000044) };

/// Vertical position of Win1
pub const WIN1V: WOVolAddress<WindowVerticalSetting> = unsafe { WOVolAddress::new(0x4000046) };

/// Controls the appearance inside of Window0 and Window1
pub const WININ: VolAddress<WindowInSetting> = unsafe { VolAddress::new(0x4000048) };

/// Controls the appearance Outside of Windows and of the OBJ Window
pub const WINOUT: VolAddress<WindowOutSetting> = unsafe { VolAddress::new(0x400004A) };

/// Allows control of the mosaic effect.
pub const MOSAIC: WOVolAddress<MosaicSetting> = unsafe { WOVolAddress::new(0x400004C) };

/// Overall configuration of color blending (see also `BLDALPHA` and `BLDY`)
pub const BLDCNT: VolAddress<BlendControlSetting> = unsafe { VolAddress::new(0x4000050) };

/// Configures alpha blending.
pub const BLDALPHA: VolAddress<BlendAlphaSetting> = unsafe { VolAddress::new(0x4000052) };

/// Configures brightness blending.
pub const BLDY: WOVolAddress<BlendBrightnessSetting> = unsafe { WOVolAddress::new(0x4000054) };
