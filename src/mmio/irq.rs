
use super::*;

/// The address value of the `IME` register.
///
/// You can unsafely turn this into a `VolAddress<u8>` and manipulate it
/// yourself, if you choose to.
pub const IME_ADDRESS: usize = 0x4_000_208;

/// Private! See `enable_interrupts`
const IME: VolAddress<u8> = unsafe { VolAddress::new(IME_ADDRESS) };

/// This activates the ability for interrupts to fire _at all_. GBATEK `IME`
///
/// # Safety
///
/// When an interrupt fires and the interrupt handler runs, that's _basically_
/// an FFI call, from the Rust perspective. Rust can't analyze that ASM code and
/// so on. So there has to be something marked as `unsafe`, and we choose to
/// mark this as being the point of unsafety.
///
/// Once interrupts are on there's a lot of things your ASM interrupt handler
/// could do to mess up the safe Rust code, and we're trusting you to not do any
/// of those things.
///
/// You have to have linked your program with an interrupt handler that doesn't
/// break any of the assumptions that Rust has about the memory that it's
/// working with. If necessary, the interrupt handler can communicate with the
/// main program via some known volatile address (or addresses) that hold the
/// information to pass between them.
pub unsafe fn enable_interrupts() {
  IME.write(1);
}

/// Prevents all interrupts from firing.
///
/// It's always safe to _turn off_ interrupts, it's just not safe to turn them
/// on.
pub fn disable_interrupts() {
  IME.write(0);
}

/// Reads if interrupts are currently enabled.
pub fn interrupts_are_enabled() -> bool {
  unsafe { core::mem::transmute(IME.read() & 0b1) }
}

/// Interrupt Enable, see the type for info.
pub const IE: VolAddress<InterruptEnable> = unsafe { VolAddress::new(0x4_000_200) };

/// Check to see what interrupts are pending. Part of GBATEK `IF`.
pub const IRQ_PENDING: ROVolAddress<InterruptRequestFlags> =
  unsafe { ROVolAddress::new(0x4_000_200) };

/// Acknowledge an interrupt as processed. Part of GBATEK `IF`.
pub const IRQ_ACKNOWLEDGE: WOVolAddress<InterruptRequestFlags> =
  unsafe { WOVolAddress::new(0x4_000_200) };
