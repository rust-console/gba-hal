
use super::*;

/// Only uses the least significant 27 bits of the address
pub const DMA0_SOURCE: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0B0) };

/// Only uses the least significant 27 bits of the address
pub const DMA0_DEST: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0B4) };

/// How many _units_ to transfer (either 16-bit or 32-bit). 0 instead indicates 0x4000.
pub const DMA0_COUNT: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_0B8) };

/// Reads the DMA0 control register (always safe to read).
pub const DMA0_CONTROL: ROVolAddress<DMAControl> = unsafe { ROVolAddress::new(0x4_000_0BA) };

/// Writes to the DMA0 control register.
///
/// # Safety
///
/// You must follow all of the DMA rules outlined in
/// [GBATEK](https://problemkaputt.de/gbatek.htm#gbadmatransfers), and you must
/// also not use DMA to alter any memory that is currently pointed to by a
/// shared reference (`&T`), unique reference (`&mut T`), or const pointer
/// (`*const T`). Memory that is currently pointed to by a mut pointer (`*mut
/// T`) is _likely_ fine to alter, though Rust's exact semantics in this area
/// are honestly a little fuzzy.
pub unsafe fn set_dma0_control(ctrl: DMAControl) {
  VolAddress::new(DMA0_CONTROL.to_usize()).write(ctrl)
}

/// Only uses the least significant 28 bits of the address
pub const DMA1_SOURCE: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0BC) };

/// Only uses the least significant 28 bits of the address
pub const DMA1_DEST: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0C0) };

/// How many _units_ to transfer (either 16-bit or 32-bit). 0 instead indicates 0x1_0000.
pub const DMA1_COUNT: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_0C4) };

/// Reads the DMA1 control register (always safe to read).
pub const DMA1_CONTROL: VolAddress<DMAControl> = unsafe { VolAddress::new(0x4_000_0C6) };

/// Writes to the DMA1 control register.
///
/// # Safety
///
/// As per `set_dma0_control`
pub unsafe fn set_dma1_control(ctrl: DMAControl) {
  VolAddress::new(DMA1_CONTROL.to_usize()).write(ctrl)
}

/// Only uses the least significant 28 bits of the address
pub const DMA2_SOURCE: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0C8) };

/// Only uses the least significant 28 bits of the address
pub const DMA2_DEST: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0CC) };

/// How many _units_ to transfer (either 16-bit or 32-bit). 0 instead indicates 0x1_0000.
pub const DMA2_COUNT: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_0D0) };

/// Reads the DMA2 control register (always safe to read).
pub const DMA2_CONTROL: VolAddress<DMAControl> = unsafe { VolAddress::new(0x4_000_0D2) };

/// Writes to the DMA2 control register.
///
/// # Safety
///
/// As per `set_dma0_control`
pub unsafe fn set_dma2_control(ctrl: DMAControl) {
  VolAddress::new(DMA2_CONTROL.to_usize()).write(ctrl)
}

/// Only uses the least significant 28 bits of the address
pub const DMA3_SOURCE: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0D4) };

/// Only uses the least significant 28 bits of the address
pub const DMA3_DEST: WOVolAddress<*mut u32> = unsafe { WOVolAddress::new(0x4_000_0D8) };

/// How many _units_ to transfer (either 16-bit or 32-bit). 0 instead indicates 0x1_0000.
pub const DMA3_COUNT: WOVolAddress<u16> = unsafe { WOVolAddress::new(0x4_000_0DC) };

/// Reads the DMA3 control register (always safe to read).
pub const DMA3_CONTROL: VolAddress<DMAControl> = unsafe { VolAddress::new(0x4_000_0DE) };

/// Writes to the DMA3 control register.
///
/// # Safety
///
/// As per `set_dma0_control`
pub unsafe fn set_dma3_control(ctrl: DMAControl) {
  VolAddress::new(DMA3_CONTROL.to_usize()).write(ctrl)
}
