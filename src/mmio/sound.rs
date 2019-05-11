
use super::*;
use typenum::consts::U8;

/// Pulse A: Sweep. GBATEK `SOUND1CNT_L`
pub const PULSE_A_SWEEP: VolAddress<SweepSetting> = unsafe { VolAddress::new(0x400_0060) };

/// Pulse A: Duty, Length, and Envelope. GBATEK `SOUND1CNT_H`
pub const PULSE_A_EFFECTS: VolAddress<DutyLenEnvelopeSetting> = unsafe { VolAddress::new(0x400_0062) };

/// Pulse A: Frequency and Master Controls. GBATEK `SOUND1CNT_X`
pub const PULSE_A_FREQ_CTRL: VolAddress<FrequencyMasterControlSetting> = unsafe { VolAddress::new(0x400_0064) };

/// Pulse B: Duty, Length, and Envelope. GBATEK `SOUND2CNT_L`
pub const PULSE_B_EFFECTS: VolAddress<DutyLenEnvelopeSetting> = unsafe { VolAddress::new(0x400_0068) };

/// Pulse B: Frequency and Master Controls. GBATEK `SOUND2CNT_H`
pub const SOUND2CNT_H: VolAddress<FrequencyMasterControlSetting> = unsafe { VolAddress::new(0x400_006C) };

pub const SOUND3CNT_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_0070) };
pub const SOUND3CNT_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_0072) };
pub const SOUND3CNT_X: VolAddress<u16> = unsafe { VolAddress::new(0x400_0074) };

pub const SOUND4CNT_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_0078) };
pub const SOUND4CNT_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_007C) };

pub const SOUNDCNT_L: VolAddress<u16> = unsafe { VolAddress::new(0x400_0080) };
pub const SOUNDCNT_H: VolAddress<u16> = unsafe { VolAddress::new(0x400_0082) };
pub const SOUNDCNT_X: VolAddress<u16> = unsafe { VolAddress::new(0x400_0084) };

pub const SOUNDBIAS: VolAddress<u16> = unsafe { VolAddress::new(0x400_0088) };

pub const WAVE_RAM: VolBlock<u16, U8> = unsafe { VolBlock::new(0x400_0090) };

pub const FIFO_A: WOVolAddress<u32> = unsafe { WOVolAddress::new(0x400_00A0) };
pub const FIFO_B: WOVolAddress<u32> = unsafe { WOVolAddress::new(0x400_00A4) };
