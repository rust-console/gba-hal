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
  /// Configuration for the display control register.
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
  DisplayControl,
  u16
);
#[allow(missing_docs)]
impl DisplayControl {
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
  DisplayStatus,
  u16
);
#[allow(missing_docs)]
impl DisplayStatus {
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
  BackgroundControl, u16
}
#[allow(missing_docs)]
impl BackgroundControl {
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
  Mosaic, u16
}
#[allow(missing_docs)]
impl Mosaic {
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
  WindowHorizontal, u16
}
#[allow(missing_docs)]
impl WindowHorizontal {
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
  WindowVertical, u16
}
#[allow(missing_docs)]
impl WindowVertical {
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
  WindowIn, u16
}
#[allow(missing_docs)]
impl WindowIn {
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
  WindowOut, u16
}
#[allow(missing_docs)]
impl WindowOut {
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
  BlendControl, u16
}
#[allow(missing_docs)]
impl BlendControl {
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
  BlendAlpha, u16
}
#[allow(missing_docs)]
impl BlendAlpha {
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
  BlendBrightness, u16
}
#[allow(missing_docs)]
impl BlendBrightness {
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
  Sweep, u8
}
#[allow(missing_docs)]
impl Sweep {
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
  DutyLenEnvelope, u16
}
#[allow(missing_docs)]
impl DutyLenEnvelope {
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
  PulseFrequencyControl, u16
}
#[allow(missing_docs)]
impl PulseFrequencyControl {
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
  WaveLengthVolume, u16
}
#[allow(missing_docs)]
impl WaveLengthVolume {
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
  LengthEnvelope, u16
}
#[allow(missing_docs)]
impl LengthEnvelope {
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

newtype! {
  /// Allows setting of the `SOUNDBIAS` register.
  ///
  /// * 1-9: Bias level, defaults to 0x100
  /// * 14-15: Amplitude resolution: 9 bits _minus_ this value.
  Soundbias, u16
}
#[allow(missing_docs)]
impl Soundbias {
  phantom_fields! {
    self.0: u16,
    bias_level: 1-9,
    amplitude_resolution: 14-15,
  }
}

newtype! {
  /// Controls left and right sound outputs.
  ///
  /// * 0-2: Master Right Volume
  /// * 4-6: Master Left Volume
  /// * 8: Pulse A right
  /// * 9: Pulse B right
  /// * 10: Wave right
  /// * 11: Noise right
  /// * 12: Pulse A left
  /// * 13: Pulse B left
  /// * 14: Wave left
  /// * 15: Noise left
  StereoControl, u16
}
#[allow(missing_docs)]
impl StereoControl {
  phantom_fields! {
    self.0: u16,
    volume_right: 0-2,
    volume_left: 4-6,
    pulse_a_right: 8,
    pulse_b_right: 9,
    wave_right: 10,
    noise_right: 11,
    pulse_a_left: 12,
    pulse_b_left: 13,
    wave_left: 14,
    noise_left: 15,
  }
}

newtype_enum! {
  /// How loudly the non-DMA sound should play
  NonDMASoundVolume = u16,
  /// 25% volume
  Quarter = 0,
  /// 50% volume
  Half = 1,
  /// 100% volume
  Full = 2,
}

newtype! {
  /// Controls DMA Sound mixing.
  ///
  /// * 0-1: non-dma sound level: quarter, half, or full.
  /// * 2: DMA sound A full (true) or half (false)
  /// * 3: DMA sound B full (true) or half (false)
  /// * 8: DMA sound A enabled right
  /// * 9: DMA sound A enabled left
  /// * 10: DMA sound A timer 1 (true) or timer 0 (false)
  /// * 11 (wo): Reset FIFO A
  /// * 12: DMA sound B enabled right
  /// * 13: DMA sound B enabled left
  /// * 14: DMA sound B timer 1 (true) or timer 0 (false)
  /// * 15 (wo): Reset FIFO B
  DMAMixer, u16
}
#[allow(missing_docs)]
impl DMAMixer {
  phantom_fields! {
    self.0: u16,
    non_dma_volume: 0-1=NonDMASoundVolume<Quarter,Half,Full>,
    dma_a_full: 2,
    dma_b_full: 3,
    dma_a_right: 8,
    dma_a_left: 9,
    dma_a_timer1: 10,
    dma_a_reset_fifo: 11,
    dma_b_right: 12,
    dma_b_left: 13,
    dma_b_timer1: 14,
    dma_b_reset_fifo: 15,
  }
}

newtype! {
  /// Allows setting of the `SOUND_STATUS_ENABLE` register.
  ///
  /// * 0 (ro): Pulse A is active
  /// * 1 (ro): Pulse B is active
  /// * 2 (ro): Wave is active
  /// * 3 (ro): Noise is active
  /// * 7: sound master enable
  SoundStatusMaster, u16
}
#[allow(missing_docs)]
impl SoundStatusMaster {
  phantom_fields! {
    self.0: u16,
    bias_level: 1-9,
    amplitude_resolution: 14-15,
  }
}

newtype_enum! {
  /// Controls the change in DMA destination address
  DestAddressControl = u16,
  #[allow(missing_docs)]
  Increment = 0,
  #[allow(missing_docs)]
  Decrement = 1,
  #[allow(missing_docs)]
  Fixed = 2,
  #[allow(missing_docs)]
  IncrementReload = 3,
}

#[allow(missing_docs)]
newtype_enum! {
  /// Controls the change in DMA source address
  SourceAddressControl = u16,
  #[allow(missing_docs)]
  Increment = 0,
  #[allow(missing_docs)]
  Decrement = 1,
  #[allow(missing_docs)]
  Fixed = 2,
}

newtype_enum! {
  /// Controls DMA starting time
  StartTiming = u16,
  /// This actually takes 2 clock cycles
  Immediately = 0,
  /// Trigger on VBlank
  VBlank = 1,
  /// Trigger on HBlank
  HBlank = 2,
  /// Varies by DMA unit:
  ///
  /// * 0: Prohibited
  /// * 1 or 2: Sound FIFO
  /// * 3: Video Capture
  Special = 3,
}

newtype! {
  /// Configures a DMA unit.
  ///
  /// * 5-6: destination address control
  /// * 7-8: source address control
  /// * 9: DMA repeat
  /// * 10: DMA transfer type
  /// * 12-13: DMA start timing
  /// * 14: Interrupt when DMA ends
  /// * 15: DMA Enable
  ///
  /// DRQ is only for special game pak units, and so it isn't supported by this
  /// type.
  DMAControl, u16
}
#[allow(missing_docs)]
impl DMAControl {
  phantom_fields! {
    self.0: u16,
    dest_address_ctrl: 5-6=DestAddressControl<Increment, Decrement, Fixed, IncrementReload>,
    src_address_ctrl: 7-8=SourceAddressControl<Increment, Decrement, Fixed>,
    dma_repeats: 9,
    dma_is_32bit: 10,
    dma_start_time: 12-13=StartTiming<Immediately, VBlank, HBlank, Special>,
    irq_at_end: 14,
    dma_enable: 15,
  }
}

newtype_enum! {
  /// Controls DMA starting time
  TimerTickRate = u8,
  /// Once every CPU cycle
  CPU1 = 0,
  /// Once per 64 CPU cycles
  CPU64 = 1,
  /// Once per 256 CPU cycles
  CPU256 = 2,
  /// Once per 1,024 CPU cycles
  CPU1024 = 3,
}

newtype! {
  /// Controls one of the four Timer units.
  ///
  /// * 0-1: Tick however many CPU cycles
  /// * 2: Instead of above, tick once per lower timer overflow (useless with
  ///   Timer0).
  /// * 6: Interrupt when this timer overflows
  /// * 7: Enable bit.
  ///
  /// Each timer also has a "reload" (WO) and "counter" (RO). A timer's "reload"
  /// value gets copied into the "counter" value every time that it overflows,
  /// or any time that the enable bit goes from 0 to 1.
  TimerControl, u8
}
#[allow(missing_docs)]
impl TimerControl {
  phantom_fields! {
    self.0: u8,
    tick_rate: 0-1=TimerTickRate<CPU1, CPU64, CPU256, CPU1024>,
    cascade: 2,
    overflow_irq: 6,
    enabled: 7,
  }
}

newtype! {
  /// This is the GBA's _native_ key reading value.
  ///
  /// The term "native" here means "low-active". In other words, `false` ==
  /// "pressed", and `true` is "released".
  ///
  /// You probably actually want to have "normal" style key data, where `false`
  /// == "released" and `true` == "pressed". To get this you must XOR the lower
  /// 10 bits of the inner value:
  ///
  /// ```
  /// # use gba_hal::data::*;
  /// # let key_input = KeyInput::default();
  /// let high_active_keys_value = key_input.0 ^ 0b11_1111_1111;
  /// ```
  ///
  /// That's not provided by this crate because this crate sticks to minimal
  /// declarations. You can of course use the [gba](https://docs.rs/gba) crate
  /// for this and other helpful higher level abstractions.
  ///
  /// The bits are as follows:
  ///
  /// * 0: a
  /// * 1: b
  /// * 2: select
  /// * 3: start
  /// * 4: right
  /// * 5: left
  /// * 6: up
  /// * 7: down
  /// * 8: r
  /// * 9: l
  KeyInput, pub u16
}
#[allow(missing_docs)]
impl KeyInput {
  phantom_fields! {
    self.0: u16,
    a_released: 0,
    b_released: 1,
    select_released: 2,
    start_released: 3,
    right_released: 4,
    left_released: 5,
    up_released: 6,
    down_released: 7,
    r_released: 8,
    l_released: 9,
  }
}

newtype! {
  /// Affects when a key interrupt is triggered.
  ///
  /// This is intended for bringing the GBA out of the very-low-power mode, not
  /// for general input.
  ///
  /// * There's one bit per button, matching the bits of the
  ///   [KeyInput](KeyInput) type.
  /// * 14: Key interrupts enabled
  /// * 15: `true` if all selected keys must be pressed to trigger the interrupt
  ///   (Logical AND), `false` if any selected key can be pressed to trigger the
  ///   interrupt (Logical OR).
  KeyControl, u16
}
#[allow(missing_docs)]
impl KeyControl {
  phantom_fields! {
    self.0: u16,
    a: 0,
    b: 1,
    select: 2,
    start: 3,
    right: 4,
    left: 5,
    up: 6,
    down: 7,
    r: 8,
    l: 9,
    irq_enabled: 14,
    require_all_selected_keys: 15,
  }
}

newtype! {
  /// This controls what types of interrupts can fire.
  ///
  /// Note that for each type of interrupt given here, there's also an interrupt
  /// flag in that item's IO control registers. For example, there's a VBlank
  /// interrupt flag here, and another VBlank interrupt flag in the
  /// [DisplayStatus](DisplayStatus). The locations vary by interrupt category.
  ///
  /// In any case, BOTH flags have to be set (as well as `IME`) for the
  /// interrupt to _actually_ happen at the appropriate time.
  ///
  /// * 0: VBlank
  /// * 1: HBlank
  /// * 2: VCounter Match
  /// * 3: Timer 0 Overflow
  /// * 4: Timer 1 Overflow
  /// * 5: Timer 2 Overflow
  /// * 6: Timer 3 Overflow
  /// * 7: Serial Communication
  /// * 8: DMA 0 Completed
  /// * 9: DMA 1 Completed
  /// * 10: DMA 2 Completed
  /// * 11: DMA 3 Completed
  /// * 12: Keypad
  /// * 13: Game Pak
  InterruptEnable, u16
}
#[allow(missing_docs)]
impl InterruptEnable {
  phantom_fields! {
    self.0: u16,
    vblank: 0,
    hblank: 1,
    vcounter: 2,
    timer0: 3,
    timer1: 4,
    timer2: 5,
    timer3: 6,
    serial: 7,
    dma0: 8,
    dma1: 9,
    dma2: 10,
    dma3: 11,
    keypad: 12,
    game_pak: 13,
  }
}

newtype! {
  /// Interrupt Requested Flags. GBATEK `IF` register.
  ///
  /// This has the same bit layout as the [InterruptEnable](InterruptEnable)
  /// type, but this type is used to either check what interrupts are pending
  /// (`IRQ_PENDING`) or acknowledge an interrupt as processed
  /// (`IRQ_ACKNOWLEDGE`).
  ///
  /// It may seem strange, but to acknowledge an interrupt as having been
  /// processed, you need to **set** that bit (and only that bit), even though
  /// it's already an active bit.
  ///
  /// The full story of how to do interrupt handling properly is beyond the
  /// scope of this comment, but that's the basics.
  InterruptRequestFlags, u16
}
#[allow(missing_docs)]
impl InterruptRequestFlags {
  phantom_fields! {
    self.0: u16,
    vblank: 0,
    hblank: 1,
    vcounter: 2,
    timer0: 3,
    timer1: 4,
    timer2: 5,
    timer3: 6,
    serial: 7,
    dma0: 8,
    dma1: 9,
    dma2: 10,
    dma3: 11,
    keypad: 12,
    game_pak: 13,
  }
}

newtype! {
  /// Color values on the GBA.
  Color, pub u16
}
#[allow(missing_docs)]
impl Color {
  phantom_fields! {
    self.0: u16,
    red: 0-4,
    green: 5-9,
    blue: 10-14,
  }
}

newtype! {
  /// 0th part of an object's attributes.
  ///
  /// * Bits 0-7: row-coordinate
  /// * Bits 8-9: Rendering style: Normal, Affine, Disabled, Double Area Affine
  /// * Bits 10-11: Object mode: Normal, SemiTransparent, Object Window
  /// * Bit 12: Mosaic
  /// * Bit 13: is 8bpp
  /// * Bits 14-15: Object Shape: Square, Horizontal, Vertical
  OBJAttr0, u16
}
#[allow(missing_docs)]
impl OBJAttr0 {
  phantom_fields! {
    self.0: u16,
    row_coordinate: 0-7,
    obj_rendering: 8-9=ObjectRender<Normal, Affine, Disabled, DoubleAreaAffine>,
    obj_mode: 10-11=ObjectMode<Normal, SemiTransparent, OBJWindow>,
    mosaic: 12,
    is_8bpp: 13,
    obj_shape: 14-15=ObjectShape<Square, Horizontal, Vertical>,
  }
}

/// What style of rendering for this object
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectRender {
  /// Standard, non-affine rendering
  Normal = 0,
  /// Affine rendering
  Affine = 1,
  /// Object disabled (saves cycles for elsewhere!)
  Disabled = 2,
  /// Affine with double render space (helps prevent clipping)
  DoubleAreaAffine = 3,
}

/// What mode to ues for the object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectMode {
  /// Show the object normally
  Normal = 0,
  /// The object becomes the "Alpha Blending 1st target" (see Alpha Blending)
  SemiTransparent = 1,
  /// Use the object's non-transparent pixels as part of a mask for the object
  /// window (see Windows).
  OBJWindow = 2,
}

/// What shape the object's appearance should be.
///
/// The specifics also depend on the `ObjectSize` set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectShape {
  /// Equal parts wide and tall
  Square = 0,
  /// Wider than tall
  Horizontal = 1,
  /// Taller than wide
  Vertical = 2,
}

newtype! {
  /// 1st part of an object's attributes.
  ///
  /// * Bits 0-8: column coordinate
  /// * Bits 9-13:
  ///   * Normal render: Bit 12 holds hflip and 13 holds vflip.
  ///   * Affine render: The affine parameter selection.
  /// * Bits 14-15: Object Size
  OBJAttr1, u16
}
#[allow(missing_docs)]
impl OBJAttr1 {
  phantom_fields! {
    self.0: u16,
    col_coordinate: 0-8,
    affine_index: 9-13,
    hflip: 12,
    vflip: 13,
    obj_size: 14-15=ObjectSize<Zero, One, Two, Three>,
  }
}

/// The object's size.
///
/// Also depends on the `ObjectShape` set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectSize {
  /// * Square: 8x8px
  /// * Horizontal: 16x8px
  /// * Vertical: 8x16px
  Zero = 0,
  /// * Square: 16x16px
  /// * Horizontal: 32x8px
  /// * Vertical: 8x32px
  One = 1,
  /// * Square: 32x32px
  /// * Horizontal: 32x16px
  /// * Vertical: 16x32px
  Two = 2,
  /// * Square: 64x64px
  /// * Horizontal: 64x32px
  /// * Vertical: 32x64px
  Three = 3,
}

newtype! {
  /// 2nd part of an object's attributes.
  ///
  /// * Bits 0-9: Base Tile Index (tile offset from CBB4)
  /// * Bits 10-11: Priority
  /// * Bits 12-15: Palbank (if using 4bpp)
  OBJAttr2, u16
}
#[allow(missing_docs)]
impl OBJAttr2 {
  phantom_fields! {
    self.0: u16,
    tile_id: 0-9,
    priority: 10-11,
    palbank: 12-15,
  }
}
