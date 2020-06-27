//! Access to various system and model specific registers.

pub mod control;
pub mod model_specific;
pub mod rflags;

/// Gets the current instruction pointer. Note that this is only approximate as it requires a few
/// instructions to execute.
#[cfg(feature = "inline_asm")]
#[inline(always)]
pub fn read_rip() -> u64 {
    let rip: u64;
    unsafe {
        asm!(
            "lea {}, rip", out(reg) rip, options(nostack, nomem)
        );
    }
    rip
}
