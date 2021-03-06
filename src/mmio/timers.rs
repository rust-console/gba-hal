use super::*;

/// Value that Timer0 will for reloads. Part of GBATEK `TM0CNT_L`.
pub const TIMER0_RELOAD: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_100) };

/// Value that Timer1 will for reloads. Part of GBATEK `TM1CNT_L`.
pub const TIMER1_RELOAD: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_104) };

/// Value that Timer2 will for reloads. Part of GBATEK `TM2CNT_L`.
pub const TIMER2_RELOAD: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_108) };

/// Value that Timer3 will for reloads. Part of GBATEK `TM3CNT_L`.
pub const TIMER3_RELOAD: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_10C) };

/// Current value of Timer0's counter. Part of GBATEK `TM0CNT_L`.
pub const TIMER0_COUNTER: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x4_000_100) };

/// Current value of Timer1's counter. Part of GBATEK `TM1CNT_L`.
pub const TIMER1_COUNTER: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x4_000_104) };

/// Current value of Timer2's counter. Part of GBATEK `TM2CNT_L`.
pub const TIMER2_COUNTER: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x4_000_108) };

/// Current value of Timer3's counter. Part of GBATEK `TM3CNT_L`.
pub const TIMER3_COUNTER: ROVolAddress<u16> = unsafe { ROVolAddress::new(0x4_000_10C) };

/// Controls the actions of Timer0. GBATEK `TM0CNT_H`.
pub const TIMER0_CONTROL: VolAddress<TimerControl> = unsafe { VolAddress::new(0x4_000_102) };

/// Controls the actions of Timer1. GBATEK `TM1CNT_H`.
pub const TIMER1_CONTROL: VolAddress<TimerControl> = unsafe { VolAddress::new(0x4_000_106) };

/// Controls the actions of Timer2. GBATEK `TM2CNT_H`.
pub const TIMER2_CONTROL: VolAddress<TimerControl> = unsafe { VolAddress::new(0x4_000_10A) };

/// Controls the actions of Timer3. GBATEK `TM3CNT_H`.
pub const TIMER3_CONTROL: VolAddress<TimerControl> = unsafe { VolAddress::new(0x4_000_10E) };
