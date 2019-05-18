
use super::*;

pub const KEYINPUT: ROVolAddress<KeyInput> = unsafe { ROVolAddress::new(0x4_000_130) };

pub const KEYCNT: VolAddress<KeyControl> = unsafe { VolAddress::new(0x4_000_130) };
