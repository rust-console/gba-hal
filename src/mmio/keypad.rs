
use super::*;

/// Reads the low-active key values (see [KeyInput](gba_hal::data::KeyInput)).
pub const KEYINPUT: ROVolAddress<KeyInput> = unsafe { ROVolAddress::new(0x4_000_130) };

/// Key interrupt control.
pub const KEYCNT: VolAddress<KeyControl> = unsafe { VolAddress::new(0x4_000_130) };
