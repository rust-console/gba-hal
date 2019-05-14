
use super::*;
use typenum::consts::U8;

//

/// Pulse A: Sweep. GBATEK `SOUND1CNT_L`
pub const PULSE_A_SWEEP: VolAddress<Sweep> = unsafe { VolAddress::new(0x400_0060) };

/// Pulse A: Duty, Length, and Envelope. GBATEK `SOUND1CNT_H`
pub const PULSE_A_EFFECTS: VolAddress<DutyLenEnvelope> = unsafe { VolAddress::new(0x400_0062) };

/// Pulse A: Frequency and Master Control. GBATEK `SOUND1CNT_X`
pub const PULSE_A_FREQ_CTRL: VolAddress<PulseFrequencyControl> =
  unsafe { VolAddress::new(0x400_0064) };

//

/// Pulse B: Duty, Length, and Envelope. GBATEK `SOUND2CNT_L`
pub const PULSE_B_EFFECTS: VolAddress<DutyLenEnvelope> = unsafe { VolAddress::new(0x400_0068) };

/// Pulse B: Frequency and Master Control. GBATEK `SOUND2CNT_H`
pub const PULSE_B_FREQ_CTRL: VolAddress<PulseFrequencyControl> =
  unsafe { VolAddress::new(0x400_006C) };

//

/// Wave: Initialization and RAM control. GBATEK `SOUND3CNT_L`
pub const WAVE_INIT_RAM_CTRL: VolAddress<WaveInitRAMControl> =
  unsafe { VolAddress::new(0x400_0070) };

/// Wave: Length and Volume. GBATEK `SOUND3CNT_H`
pub const WAVE_LENGTH_VOLUME: VolAddress<WaveLengthVolume> = unsafe { VolAddress::new(0x400_0072) };

/// Wave: Frequency and Master Control. GBATEK `SOUND3CNT_X`
pub const WAVE_FREQ_CTRL: VolAddress<WaveFrequencyControl> = unsafe { VolAddress::new(0x400_0074) };

//

/// Noise: Length and Envelope. GBATEK `SOUND4CNT_L`
pub const NOISE_LENGTH_ENVELOPE: VolAddress<LengthEnvelope> =
  unsafe { VolAddress::new(0x400_0078) };

/// Noise: Frequency and Master Control. GBATEK `SOUND4CNT_H`
pub const NOISE_FREQUENCY: VolAddress<NoiseFrequencyControl> =
  unsafe { VolAddress::new(0x400_007C) };

//

/// Stereo Controls. GBATEK `SOUNDCNT_L`.
pub const STEREO_CONTROL: VolAddress<StereoControl> = unsafe { VolAddress::new(0x400_0080) };

/// Controls DMA sound mixing. GBATEK `SOUNDCNT_H`.
pub const DMA_MIXER: VolAddress<DMAMixer> = unsafe { VolAddress::new(0x400_0082) };

/// Sound status and sound master enable.
///
/// If sound is not enabled via this register, most sound registers are reset
/// (if active) and unaccessible. The exceptions are `DMA_MIXER` and `SOUNDBIAS`.
pub const SOUND_STATUS_ENABLE: VolAddress<SoundStatusMaster> =
  unsafe { VolAddress::new(0x400_0084) };

//

/// Controls final sound output. You can usually leave this at default.
pub const SOUNDBIAS: VolAddress<Soundbias> = unsafe { VolAddress::new(0x400_0088) };

/// The programmer-accessible wave ram data.
///
/// Note that there are two wave ram banks. At any given moment one of them is
/// set for use as playback, and the other one will be accessible here. The
/// playback bank is played by playing 4 bits as a single sample and then
/// rotating the entire 128 bits of wave ram to put the next sample in place.
/// Thus, when updating the wave ram you generally have to re-write all of the
/// memory even if you only want a small change, because the loop will have been
/// shifted around by some unknown amount.
pub const WAVE_RAM: VolBlock<u16, U8> = unsafe { VolBlock::new(0x400_0090) };

/// Input for DMA Sound Channel A.
///
/// This allows you to input 4 bytes at a time into DMA Sound A. There is an
/// internal buffer of 32 bytes. Each byte is a single sound sample (`i8`).
///
/// Please see GBATEK for a full explanation of the (somewhat complicated)
/// process of using the DMA sound outputs.
pub const FIFO_A: WOVolAddress<[i8; 4]> = unsafe { WOVolAddress::new(0x400_00A0) };

/// Input for DMA Sound Channel B.
///
/// As `FIFO_A`, but for DMA Sound B
pub const FIFO_B: WOVolAddress<[i8; 4]> = unsafe { WOVolAddress::new(0x400_00A4) };
