//! Data types for the GBA's MMIO and such.
//!
//! These data types can actually be used on any machine you like, not just the
//! GBA. They're probably not very useful on other machines, but who knows.

use super::{newtype, newtype_enum};
use gba_proc_macro::phantom_fields;

newtype_enum! {
  /// The six display modes available on the GBA.
  DisplayMode = u16,
  /// * Affine: No
  /// * Layers: 0/1/2/3
  /// * Size(px): 256x256 to 512x512
  /// * Tiles: 1024
  /// * Palette Modes: 4bpp or 8bpp
  Mode0 = 0,
  /// * BG0 / BG1: As Mode0
  /// * BG2: As Mode2
  Mode1 = 1,
  /// * Affine: Yes
  /// * Layers: 2/3
  /// * Size(px): 128x128 to 1024x1024
  /// * Tiles: 256
  /// * Palette Modes: 8bpp
  Mode2 = 2,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 240x160 (1 page)
  /// * Bitmap
  /// * Full Color
  Mode3 = 3,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 240x160 (2 pages)
  /// * Bitmap
  /// * Palette Modes: 8bpp
  Mode4 = 4,
  /// * Affine: Yes
  /// * Layers: 2
  /// * Size(px): 160x128 (2 pages)
  /// * Bitmap
  /// * Full Color
  Mode5 = 5,
}

newtype!(
  /// Setting for the display control register.
  ///
  /// * 0-2: `DisplayMode`
  /// * 3: CGB mode flag
  /// * 4: Display frame 1 (Modes 4/5 only)
  /// * 5: "hblank interval free", allows full access to OAM during hblank
  /// * 6: Object tile memory 1-dimensional
  /// * 7: Force vblank
  /// * 8: Display bg0 layer
  /// * 9: Display bg1 layer
  /// * 10: Display bg2 layer
  /// * 11: Display bg3 layer
  /// * 12: Display objects layer
  /// * 13: Window 0 display
  /// * 14: Window 1 display
  /// * 15: Object window
  DisplayControlSetting,
  u16
);
#[allow(missing_docs)]
impl DisplayControlSetting {
  phantom_fields! {
    self.0: u16,
    mode: 0-2=DisplayMode<Mode0, Mode1, Mode2, Mode3, Mode4, Mode5>,
    frame1: 4,
    hblank_interval_free: 5,
    oam_memory_1d: 6,
    force_vblank: 7,
    bg0: 8,
    bg1: 9,
    bg2: 10,
    bg3: 11,
    obj: 12,
    win0: 13,
    win1: 14,
    obj_window: 15,
  }
}

newtype!(
  /// Display Status and interrupt settings.
  ///
  /// * 0: currently in vblank (read only)
  /// * 1: currently in hblank (read only)
  /// * 2: vcounter matches current setting (read only)
  /// * 3: vblank interrupt enable
  /// * 4: hblank interrupt enable
  /// * 5: vcounter interrupt enable
  /// * 8-15: vcount setting.
  ///
  /// "Although the drawing time is only 960 cycles (240*4), the H-Blank flag is
  /// "0" for a total of 1006 cycles." -gbatek
  DisplayStatusSetting,
  u16
);
#[allow(missing_docs)]
impl DisplayStatusSetting {
  phantom_fields! {
    self.0: u16,
    vblank_flag: 0,
    hblank_flag: 1,
    vcounter_flag: 2,
    vblank_irq_enable: 3,
    hblank_irq_enable: 4,
    vcounter_irq_enable: 5,
    vcount_setting: 8-15,
  }
}

newtype! {
  /// Allows configuration of a background layer.
  ///
  /// Bits 0-1: BG Priority (lower number is higher priority, like an index)
  /// Bits 2-3: Character Base Block (0 through 3, 16k each)
  /// Bit 6: Mosaic mode
  /// Bit 7: is 8bpp
  /// Bit 8-12: Screen Base Block (0 through 31, 2k each)
  /// Bit 13: Display area overflow wraps (otherwise transparent, affine BG only)
  /// Bit 14-15: Screen Size (details depend on Text/Affine mode)
  BackgroundControlSetting, u16
}
#[allow(missing_docs)]
impl BackgroundControlSetting {
  phantom_fields! {
    self.0: u16,
    bg_priority: 0-1,
    char_base_block: 2-3,
    mosaic: 6,
    is_8bpp: 7,
    screen_base_block: 8-12,
    affine_display_overflow_wrapping: 13,
    size: 14-15,
  }
}

newtype! {
  /// Allows control of the Mosaic effect.
  ///
  /// Values are the _increase_ for each top-left pixel to be duplicated in the
  /// final result. If you want to duplicate some other pixel than the top-left,
  /// you can offset the background or object by an appropriate amount.
  ///
  /// 0) No effect (1+0)
  /// 1) Each pixel becomes 2 pixels (1+1)
  /// 2) Each pixel becomes 3 pixels (1+2)
  /// 3) Each pixel becomes 4 pixels (1+3)
  ///
  /// * Bits 0-3: BG mosaic horizontal increase
  /// * Bits 4-7: BG mosaic vertical increase
  /// * Bits 8-11: Object mosaic horizontal increase
  /// * Bits 12-15: Object mosaic vertical increase
  MosaicSetting, u16
}
#[allow(missing_docs)]
impl MosaicSetting {
  phantom_fields! {
    self.0: u16,
    bg_horizontal_inc: 0-3,
    bg_vertical_inc: 4-7,
    obj_horizontal_inc: 8-11,
    obj_vertical_inc: 12-15,
  }
}

newtype! {
  /// A two-byte fixed point value.
  ///
  /// * Signed
  /// * 7 integral bits
  /// * 8 fractional bits (x/256)
  #[allow(bad_style)]
  FP_I_7_8, pub i16
}

newtype! {
  /// A four-byte fixed point value.
  ///
  /// * Signed
  /// * 19 integral bits (bits above are ignored)
  /// * 8 fractional bits (x/256)
  #[allow(bad_style)]
  FP_I_19_8, pub i32
}

newtype! {
  /// Horizontal control for Window effect.
  ///
  /// * x1: Leftmost window edge (8-bit)
  /// * x2: Rightmost window edge +1 (8-bit)
  WindowHorizontalSetting, u16
}
#[allow(missing_docs)]
impl WindowHorizontalSetting {
  phantom_fields! {
    self.0: u16,
    x1: 8-15,
    x2: 0-7,
  }
}

newtype! {
  /// Vertical control for Window effect.
  ///
  /// * y1: Leftmost window edge (8-bit)
  /// * y2: Rightmost window edge +1 (8-bit)
  WindowVerticalSetting, u16
}
#[allow(missing_docs)]
impl WindowVerticalSetting {
  phantom_fields! {
    self.0: u16,
    y1: 8-15,
    y2: 0-7,
  }
}

newtype! {
  /// Controls the interior of windows 0 and 1.
  ///
  /// * 0-3: Win0 BG0 through BG3 enable
  /// * 4: Win0 OBJ enable
  /// * 5: Win0 color special effect enable
  /// * 8-11: Win0 BG0 through BG3 enable
  /// * 12: Win0 OBJ enable
  /// * 13: Win0 color special effect enable
  WindowInSetting, u16
}
#[allow(missing_docs)]
impl WindowInSetting {
  phantom_fields! {
    self.0: u16,
    win0_bg0: 0,
    win0_bg1: 1,
    win0_bg2: 2,
    win0_bg3: 3,
    win0_obj: 4,
    win0_color_effect: 5,
    win1_bg0: 8,
    win1_bg1: 9,
    win1_bg2: 10,
    win1_bg3: 11,
    win1_obj: 12,
    win1_color_effect: 13,
  }
}

newtype! {
  /// Controls outside of the windows and the OBJ window.
  ///
  /// * 0-3: Outside Window BG0 through BG3 enable
  /// * 4: Outside Window OBJ enable
  /// * 5: Outside Window color special effect enable
  /// * 8-11: OBJ Window BG0 through BG3 enable
  /// * 12: OBJ Window OBJ enable
  /// * 13: OBJ Window color special effect enable
  WindowOutSetting, u16
}
#[allow(missing_docs)]
impl WindowOutSetting {
  phantom_fields! {
    self.0: u16,
    outside_bg0: 0,
    outside_bg1: 1,
    outside_bg2: 2,
    outside_bg3: 3,
    outside_obj: 4,
    outside_color_effect: 5,
    win_obj_bg0: 8,
    win_obj_bg1: 9,
    win_obj_bg2: 10,
    win_obj_bg3: 11,
    win_obj_obj: 12,
    win_obj_color_effect: 13,
  }
}

newtype_enum! {
  /// What color special effect to apply to selected pixels.
  ColorSpecialEffect = u16,
  /// No color effect.
  None = 0,
  /// 1st and 2nd targets mixed.
  AlphaBlend = 1,
  /// 1st target moves toward white
  BrightnessIncrease = 2,
  /// 2nd target moves toward black
  BrightnessDecrease = 3,
}

newtype! {
  /// Controls the color special effect
  ///
  /// * 0-5: 1st target pixel selections
  /// * 6-7: color special effect
  /// * 8-13: 2nd target pixel selections
  ///
  /// "1st target" picks the top-most pixel among the selections, and then "2nd
  /// target" will pick the pixel underneath that. The "2nd target" only matters
  /// if you've selected the AlphaBlend effect.
  ///
  /// For the names, note that BD = Backdrop (the color when nothing at all is
  /// drawn from any background or object).
  BlendControlSetting, u16
}
#[allow(missing_docs)]
impl BlendControlSetting {
  phantom_fields! {
    self.0: u16,
    bg0_1st_target: 0,
    bg1_1st_target: 1,
    bg2_1st_target: 2,
    bg3_1st_target: 3,
    obj_1st_target: 4,
    bd_1st_target: 5,
    color_special_effect: 6-7=ColorSpecialEffect<None, AlphaBlend, BrightnessIncrease, BrightnessDecrease>,
    bg0_2nd_target: 8,
    bg1_2nd_target: 9,
    bg2_2nd_target: 10,
    bg3_2nd_target: 11,
    obj_2nd_target: 12,
    bd_2nd_target: 13,
  }
}

newtype! {
  /// Controls alpha blend.
  ///
  /// Both fields are x/16, but values above 16 are capped at 16/16.
  ///
  /// For each color channel of an affected 1st target pixel with a valid 2nd
  /// target pixel the final channel value is as follows:
  ///
  /// * `I = min(31, I_1st * EVA + I_2nd * EVB)`
  ///
  /// If the blend mode isn't set to `AlphaBlend`, or if the 1st target and 2nd
  /// target combination isn't valid for this pixel location then this register
  /// has no effect at all.
  BlendAlphaSetting, u16
}
#[allow(missing_docs)]
impl BlendAlphaSetting {
  phantom_fields! {
    self.0: u16,
    eva_coefficient: 0-4,
    evb_coefficient: 8-12,
  }
}

newtype! {
  /// Controls brightness blend.
  ///
  /// This is x/16, but values above 16 are capped at 16/16.
  ///
  /// For each color channel of an affected 1st target pixel the final channel
  /// value is as follows:
  ///
  /// * **Increase:** `I = I_1st + (31 - I_1st) * EVY`
  /// * **Decrease:** `I = I_1st - I_1st * EVY`
  ///
  /// If the blend mode isn't set to `BrightnessIncrease` or
  /// `BrightnessDecrease` this register has no effect at all.
  BlendBrightnessSetting, u16
}
#[allow(missing_docs)]
impl BlendBrightnessSetting {
  phantom_fields! {
    self.0: u16,
    evy_coefficient: 0-4,
  }
}

newtype! {
  /// Controls the sweep effect (Pulse A only).
  ///
  /// * 0-2: Shift number
  /// * 3: Sweep is decreasing or increasing
  /// * 4-6: Time per sweep. Units of `x/128` sec aka `x * 7.8` ms
  ///
  /// If sweep is disabled by setting sweep time to 0, the sweep should also be
  /// set to decreasing mode.
  SweepSetting, u8
}
#[allow(missing_docs)]
impl SweepSetting {
  phantom_fields! {
    self.0: u8,
    shift_num: 0-2,
    decreasing: 3,
    timer: 4-6,
  }
}

newtype_enum! {
  /// How much of the pulse wave should be the "active" value.
  PulseDutyPattern = u16,
  /// 1/8th (12.5%)
  Eighth = 0,
  /// 1/4th (25%)
  Quarter = 1,
  /// 1/2 (50%)
  Half = 2,
  /// 3/4ths (75%). This sounds the same as the 25% mode.
  ThreeQuarters = 3,
}

newtype! {
  /// Combines the main pulse voice effects into a single setting.
  ///
  /// * 0-5 (wo): Sound length, `(64-n)/256` seconds
  /// * 6-7: Pulse Duty
  /// * 8-10: Time per envelope step: `x/64` sec, or 0 for no envelope.
  /// * 11: If the envelope is increasing or decreasing.
  /// * 12-15: Initial envelope volume (0 = no sound)
  DutyLenEnvelopeSetting, u16
}
#[allow(missing_docs)]
impl DutyLenEnvelopeSetting {
  phantom_fields! {
    self.0: u16,
    length: 0-5,
    duty: 6-7=PulseDutyPattern<Eighth, Quarter, Half, ThreeQuarters>,
    envelope_time: 8-10,
    envelope_increasing: 11,
    initial_volume: 12-15,
  }
}

newtype! {
  /// Frequency and master control settings
  ///
  /// * 0-10 (wo): Frequency `131072/(2048-n)` Hz
  /// * 14: Stop output when length expires
  /// * 15 (wo): Initialize/restart this sound
  PulseFrequencyControlSetting, u16
}
#[allow(missing_docs)]
impl PulseFrequencyControlSetting {
  phantom_fields! {
    self.0: u16,
    frequency: 0-10,
    timeout_enabled: 14,
    init_restart: 15,
  }
}

newtype! {
  /// Controls how the Wave RAM is accessed, and if the sound plays at all.
  ///
  /// * 5: true, playback runs through both banks as a 64 digit loop. false,
  ///   playback runs though just a single bank as a 32 digit loop.
  /// * 6: true selects bank 1 for playback, false selects bank 0.
  /// * 7: Wave sound is enabled.
  WaveInitRAMControl, u8
}
#[allow(missing_docs)]
impl WaveInitRAMControl {
  phantom_fields! {
    self.0: u8,
    use_both_banks: 5,
    use_bank_1: 6,
    playback: 7,
  }
}

newtype_enum! {
  /// How loudly the Wave output should play
  WaveVolume = u16,
  /// No sound
  Zero = 0,
  /// 100% sound
  Full = 1,
  /// 50% sound
  Half = 2,
  /// 25% sound
  Quarter = 3,
}

newtype! {
  /// Length and Volume controls for the Wave output.
  ///
  /// * 0-7 (wo): Sound Length: `(256-n)/256` seconds
  /// * 13-14: Volume Mode
  /// * 15: Override above and use 75%
  WaveLengthVolumeSetting, u16
}
#[allow(missing_docs)]
impl WaveLengthVolumeSetting {
  phantom_fields! {
    self.0: u16,
    length: 0-7,
    volume: 13-14=WaveVolume<Zero, Full, Half, Quarter>,
    override_75percent: 15,
  }
}

newtype! {
  /// Wave output frequency and master control settings.
  ///
  /// * 0-10 (wo): Sample Rate `2097152/(2048-n)` Hz
  /// * 14: Stop output when length expires
  /// * 15 (wo): Initialize / restart the sound.
  WaveFrequencyControl, u16
}
#[allow(missing_docs)]
impl WaveFrequencyControl {
  phantom_fields! {
    self.0: u16,
    sample_rate: 0-10,
    use_timeout: 14,
    initialize: 15,
  }
}

newtype! {
  /// Length and envelope controls for the Noise output
  ///
  /// * 0-5 (wo): sound length `(64-n)/256` seconds
  /// * 8-10: Envelope step time, `n/64` seconds, 0 for off.
  /// * 11: Envelope increasing
  /// * 12-15: Envelope initial volume
  LengthEnvelopeSetting, u16
}
#[allow(missing_docs)]
impl LengthEnvelopeSetting {
  phantom_fields! {
    self.0: u16,
    length: 0-5,
    step_time: 8-10,
    envelope_increasing: 11,
    initial_volume: 12-15,
  }
}

newtype! {
  /// Noise channel frequency and master control setting
  ///
  /// Frequency is `524288 / r / 2^(s+1)` Hz. For r=0 assume r=0.5 instead.
  ///
  /// * 0-2: divide ratio `r`
  /// * 3: Use a 7-bit counter (true) or 15-bit counter (false)
  /// * 4-7: shift clock frequency `s`
  /// * 14: Stop output when length expires
  /// * 15 (wo): Initialize / restart the sound.
  NoiseFrequencyControl, u16
}
#[allow(missing_docs)]
impl NoiseFrequencyControl {
  phantom_fields! {
    self.0: u16,
    divide_ratio: 0-2,
    counter_is_7bit: 3,
    shift_clock_frequency: 4-7,
    length_flag: 14,
    initialize: 15,
  }
}
