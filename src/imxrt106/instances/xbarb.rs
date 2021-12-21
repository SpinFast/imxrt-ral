#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Crossbar Switch
//!
//! Used by: imxrt1061, imxrt1062, imxrt1064

pub use crate::imxrt106::peripherals::xbarb::Instance;
pub use crate::imxrt106::peripherals::xbarb::{RegisterBlock, ResetValues};

pub use crate::imxrt106::peripherals::xbarb::{SEL0, SEL1, SEL2, SEL3, SEL4, SEL5, SEL6, SEL7};
#[cfg(not(feature = "nosync"))]
use core::sync::atomic::{AtomicBool, Ordering};

/// The XBARB2 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type XBARB2 = Instance<2>;

/// The XBARB2 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XBARB2 = Instance<2>;
/// ```
#[cfg(feature = "doc")]
pub struct XBARB2 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for XBARB2 {}
impl crate::Valid for XBARB2 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XBARB2_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XBARB2 peripheral instance
#[cfg(not(feature = "nosync"))]
impl XBARB2 {
    const INSTANCE: Self = Self {
        addr: 0x403c0000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in XBARB2
    pub const reset: ResetValues = ResetValues {
        SEL0: 0x00000000,
        SEL1: 0x00000000,
        SEL2: 0x00000000,
        SEL3: 0x00000000,
        SEL4: 0x00000000,
        SEL5: 0x00000000,
        SEL6: 0x00000000,
        SEL7: 0x00000000,
    };

    /// Safe access to XBARB2
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = XBARB2_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XBARB2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XBARB2_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XBARB2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XBARB2_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl XBARB2 {
    /// The interrupts associated with XBARB2
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with XBARB2
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XBARB2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XBARB2: *const RegisterBlock = 0x403c0000 as *const _;

/// The XBARB3 peripheral instance.
#[cfg(not(feature = "doc"))]
pub type XBARB3 = Instance<3>;

/// The XBARB3 peripheral instance.
///
/// This is a new type only for documentation purposes. When
/// compiling for a target, this is defined as
///
/// ```rust
/// pub type XBARB3 = Instance<3>;
/// ```
#[cfg(feature = "doc")]
pub struct XBARB3 {
    #[allow(unused)] // Only for documentation generation.
    addr: u32,
}

impl crate::private::Sealed for XBARB3 {}
impl crate::Valid for XBARB3 {}

#[cfg(not(feature = "nosync"))]
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static XBARB3_TAKEN: AtomicBool = AtomicBool::new(false);

/// Access functions for the XBARB3 peripheral instance
#[cfg(not(feature = "nosync"))]
impl XBARB3 {
    const INSTANCE: Self = Self {
        addr: 0x403c4000,
        #[cfg(not(feature = "doc"))]
        intrs: &[],
    };

    /// Reset values for each field in XBARB3
    pub const reset: ResetValues = ResetValues {
        SEL0: 0x00000000,
        SEL1: 0x00000000,
        SEL2: 0x00000000,
        SEL3: 0x00000000,
        SEL4: 0x00000000,
        SEL5: 0x00000000,
        SEL6: 0x00000000,
        SEL7: 0x00000000,
    };

    /// Safe access to XBARB3
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[inline]
    pub fn take() -> Option<Self> {
        let taken = XBARB3_TAKEN.swap(true, Ordering::SeqCst);
        if taken {
            None
        } else {
            Some(Self::INSTANCE)
        }
    }

    /// Release exclusive access to XBARB3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[inline]
    pub fn release(_: Self) {
        let taken = XBARB3_TAKEN.swap(false, Ordering::SeqCst);
        assert!(taken, "Released a peripheral which was not taken");
    }

    /// Unsafely steal XBARB3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[inline]
    pub unsafe fn steal() -> Self {
        XBARB3_TAKEN.store(true, Ordering::SeqCst);
        Self::INSTANCE
    }
}

impl XBARB3 {
    /// The interrupts associated with XBARB3
    #[cfg(not(feature = "doc"))]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];

    /// The interrupts associated with XBARB3
    ///
    /// Note: the values are invalid for a documentation build.
    #[cfg(feature = "doc")]
    pub const INTERRUPTS: [crate::Interrupt; 0] = [];
}

/// Raw pointer to XBARB3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const XBARB3: *const RegisterBlock = 0x403c4000 as *const _;
