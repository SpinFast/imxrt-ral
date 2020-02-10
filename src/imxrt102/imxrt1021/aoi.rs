#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AND/OR/INVERT module

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Boolean Function Term 0 and 1 Configuration Register for EVENTn
pub mod BFCRT010 {

    /// Product term 1, D input configuration
    pub mod PT1_DC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the D input in this product term to a logical zero
            pub const PT1_DC_0: u32 = 0b00;

            /// 0b01: Pass the D input in this product term
            pub const PT1_DC_1: u32 = 0b01;

            /// 0b10: Complement the D input in this product term
            pub const PT1_DC_2: u32 = 0b10;

            /// 0b11: Force the D input in this product term to a logical one
            pub const PT1_DC_3: u32 = 0b11;
        }
    }

    /// Product term 1, C input configuration
    pub mod PT1_CC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the C input in this product term to a logical zero
            pub const PT1_CC_0: u32 = 0b00;

            /// 0b01: Pass the C input in this product term
            pub const PT1_CC_1: u32 = 0b01;

            /// 0b10: Complement the C input in this product term
            pub const PT1_CC_2: u32 = 0b10;

            /// 0b11: Force the C input in this product term to a logical one
            pub const PT1_CC_3: u32 = 0b11;
        }
    }

    /// Product term 1, B input configuration
    pub mod PT1_BC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the B input in this product term to a logical zero
            pub const PT1_BC_0: u32 = 0b00;

            /// 0b01: Pass the B input in this product term
            pub const PT1_BC_1: u32 = 0b01;

            /// 0b10: Complement the B input in this product term
            pub const PT1_BC_2: u32 = 0b10;

            /// 0b11: Force the B input in this product term to a logical one
            pub const PT1_BC_3: u32 = 0b11;
        }
    }

    /// Product term 1, A input configuration
    pub mod PT1_AC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the A input in this product term to a logical zero
            pub const PT1_AC_0: u32 = 0b00;

            /// 0b01: Pass the A input in this product term
            pub const PT1_AC_1: u32 = 0b01;

            /// 0b10: Complement the A input in this product term
            pub const PT1_AC_2: u32 = 0b10;

            /// 0b11: Force the A input in this product term to a logical one
            pub const PT1_AC_3: u32 = 0b11;
        }
    }

    /// Product term 0, D input configuration
    pub mod PT0_DC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the D input in this product term to a logical zero
            pub const PT0_DC_0: u32 = 0b00;

            /// 0b01: Pass the D input in this product term
            pub const PT0_DC_1: u32 = 0b01;

            /// 0b10: Complement the D input in this product term
            pub const PT0_DC_2: u32 = 0b10;

            /// 0b11: Force the D input in this product term to a logical one
            pub const PT0_DC_3: u32 = 0b11;
        }
    }

    /// Product term 0, C input configuration
    pub mod PT0_CC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the C input in this product term to a logical zero
            pub const PT0_CC_0: u32 = 0b00;

            /// 0b01: Pass the C input in this product term
            pub const PT0_CC_1: u32 = 0b01;

            /// 0b10: Complement the C input in this product term
            pub const PT0_CC_2: u32 = 0b10;

            /// 0b11: Force the C input in this product term to a logical one
            pub const PT0_CC_3: u32 = 0b11;
        }
    }

    /// Product term 0, B input configuration
    pub mod PT0_BC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the B input in this product term to a logical zero
            pub const PT0_BC_0: u32 = 0b00;

            /// 0b01: Pass the B input in this product term
            pub const PT0_BC_1: u32 = 0b01;

            /// 0b10: Complement the B input in this product term
            pub const PT0_BC_2: u32 = 0b10;

            /// 0b11: Force the B input in this product term to a logical one
            pub const PT0_BC_3: u32 = 0b11;
        }
    }

    /// Product term 0, A input configuration
    pub mod PT0_AC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the A input in this product term to a logical zero
            pub const PT0_AC_0: u32 = 0b00;

            /// 0b01: Pass the A input in this product term
            pub const PT0_AC_1: u32 = 0b01;

            /// 0b10: Complement the A input in this product term
            pub const PT0_AC_2: u32 = 0b10;

            /// 0b11: Force the A input in this product term to a logical one
            pub const PT0_AC_3: u32 = 0b11;
        }
    }
}

/// Boolean Function Term 0 and 1 Configuration Register for EVENTn
pub mod BFCRT011 {
    pub use super::BFCRT010::PT0_AC;
    pub use super::BFCRT010::PT0_BC;
    pub use super::BFCRT010::PT0_CC;
    pub use super::BFCRT010::PT0_DC;
    pub use super::BFCRT010::PT1_AC;
    pub use super::BFCRT010::PT1_BC;
    pub use super::BFCRT010::PT1_CC;
    pub use super::BFCRT010::PT1_DC;
}

/// Boolean Function Term 0 and 1 Configuration Register for EVENTn
pub mod BFCRT012 {
    pub use super::BFCRT010::PT0_AC;
    pub use super::BFCRT010::PT0_BC;
    pub use super::BFCRT010::PT0_CC;
    pub use super::BFCRT010::PT0_DC;
    pub use super::BFCRT010::PT1_AC;
    pub use super::BFCRT010::PT1_BC;
    pub use super::BFCRT010::PT1_CC;
    pub use super::BFCRT010::PT1_DC;
}

/// Boolean Function Term 0 and 1 Configuration Register for EVENTn
pub mod BFCRT013 {
    pub use super::BFCRT010::PT0_AC;
    pub use super::BFCRT010::PT0_BC;
    pub use super::BFCRT010::PT0_CC;
    pub use super::BFCRT010::PT0_DC;
    pub use super::BFCRT010::PT1_AC;
    pub use super::BFCRT010::PT1_BC;
    pub use super::BFCRT010::PT1_CC;
    pub use super::BFCRT010::PT1_DC;
}

/// Boolean Function Term 2 and 3 Configuration Register for EVENTn
pub mod BFCRT230 {

    /// Product term 3, D input configuration
    pub mod PT3_DC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the D input in this product term to a logical zero
            pub const PT3_DC_0: u32 = 0b00;

            /// 0b01: Pass the D input in this product term
            pub const PT3_DC_1: u32 = 0b01;

            /// 0b10: Complement the D input in this product term
            pub const PT3_DC_2: u32 = 0b10;

            /// 0b11: Force the D input in this product term to a logical one
            pub const PT3_DC_3: u32 = 0b11;
        }
    }

    /// Product term 3, C input configuration
    pub mod PT3_CC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the C input in this product term to a logical zero
            pub const PT3_CC_0: u32 = 0b00;

            /// 0b01: Pass the C input in this product term
            pub const PT3_CC_1: u32 = 0b01;

            /// 0b10: Complement the C input in this product term
            pub const PT3_CC_2: u32 = 0b10;

            /// 0b11: Force the C input in this product term to a logical one
            pub const PT3_CC_3: u32 = 0b11;
        }
    }

    /// Product term 3, B input configuration
    pub mod PT3_BC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the B input in this product term to a logical zero
            pub const PT3_BC_0: u32 = 0b00;

            /// 0b01: Pass the B input in this product term
            pub const PT3_BC_1: u32 = 0b01;

            /// 0b10: Complement the B input in this product term
            pub const PT3_BC_2: u32 = 0b10;

            /// 0b11: Force the B input in this product term to a logical one
            pub const PT3_BC_3: u32 = 0b11;
        }
    }

    /// Product term 3, A input configuration
    pub mod PT3_AC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the A input in this product term to a logical zero
            pub const PT3_AC_0: u32 = 0b00;

            /// 0b01: Pass the A input in this product term
            pub const PT3_AC_1: u32 = 0b01;

            /// 0b10: Complement the A input in this product term
            pub const PT3_AC_2: u32 = 0b10;

            /// 0b11: Force the A input in this product term to a logical one
            pub const PT3_AC_3: u32 = 0b11;
        }
    }

    /// Product term 2, D input configuration
    pub mod PT2_DC {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the D input in this product term to a logical zero
            pub const PT2_DC_0: u32 = 0b00;

            /// 0b01: Pass the D input in this product term
            pub const PT2_DC_1: u32 = 0b01;

            /// 0b10: Complement the D input in this product term
            pub const PT2_DC_2: u32 = 0b10;

            /// 0b11: Force the D input in this product term to a logical one
            pub const PT2_DC_3: u32 = 0b11;
        }
    }

    /// Product term 2, C input configuration
    pub mod PT2_CC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the C input in this product term to a logical zero
            pub const PT2_CC_0: u32 = 0b00;

            /// 0b01: Pass the C input in this product term
            pub const PT2_CC_1: u32 = 0b01;

            /// 0b10: Complement the C input in this product term
            pub const PT2_CC_2: u32 = 0b10;

            /// 0b11: Force the C input in this product term to a logical one
            pub const PT2_CC_3: u32 = 0b11;
        }
    }

    /// Product term 2, B input configuration
    pub mod PT2_BC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the B input in this product term to a logical zero
            pub const PT2_BC_0: u32 = 0b00;

            /// 0b01: Pass the B input in this product term
            pub const PT2_BC_1: u32 = 0b01;

            /// 0b10: Complement the B input in this product term
            pub const PT2_BC_2: u32 = 0b10;

            /// 0b11: Force the B input in this product term to a logical one
            pub const PT2_BC_3: u32 = 0b11;
        }
    }

    /// Product term 2, A input configuration
    pub mod PT2_AC {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Force the A input in this product term to a logical zero
            pub const PT2_AC_0: u32 = 0b00;

            /// 0b01: Pass the A input in this product term
            pub const PT2_AC_1: u32 = 0b01;

            /// 0b10: Complement the A input in this product term
            pub const PT2_AC_2: u32 = 0b10;

            /// 0b11: Force the A input in this product term to a logical one
            pub const PT2_AC_3: u32 = 0b11;
        }
    }
}

/// Boolean Function Term 2 and 3 Configuration Register for EVENTn
pub mod BFCRT231 {
    pub use super::BFCRT230::PT2_AC;
    pub use super::BFCRT230::PT2_BC;
    pub use super::BFCRT230::PT2_CC;
    pub use super::BFCRT230::PT2_DC;
    pub use super::BFCRT230::PT3_AC;
    pub use super::BFCRT230::PT3_BC;
    pub use super::BFCRT230::PT3_CC;
    pub use super::BFCRT230::PT3_DC;
}

/// Boolean Function Term 2 and 3 Configuration Register for EVENTn
pub mod BFCRT232 {
    pub use super::BFCRT230::PT2_AC;
    pub use super::BFCRT230::PT2_BC;
    pub use super::BFCRT230::PT2_CC;
    pub use super::BFCRT230::PT2_DC;
    pub use super::BFCRT230::PT3_AC;
    pub use super::BFCRT230::PT3_BC;
    pub use super::BFCRT230::PT3_CC;
    pub use super::BFCRT230::PT3_DC;
}

/// Boolean Function Term 2 and 3 Configuration Register for EVENTn
pub mod BFCRT233 {
    pub use super::BFCRT230::PT2_AC;
    pub use super::BFCRT230::PT2_BC;
    pub use super::BFCRT230::PT2_CC;
    pub use super::BFCRT230::PT2_DC;
    pub use super::BFCRT230::PT3_AC;
    pub use super::BFCRT230::PT3_BC;
    pub use super::BFCRT230::PT3_CC;
    pub use super::BFCRT230::PT3_DC;
}
pub struct RegisterBlock {
    /// Boolean Function Term 0 and 1 Configuration Register for EVENTn
    pub BFCRT010: RWRegister<u16>,

    /// Boolean Function Term 2 and 3 Configuration Register for EVENTn
    pub BFCRT230: RWRegister<u16>,

    /// Boolean Function Term 0 and 1 Configuration Register for EVENTn
    pub BFCRT011: RWRegister<u16>,

    /// Boolean Function Term 2 and 3 Configuration Register for EVENTn
    pub BFCRT231: RWRegister<u16>,

    /// Boolean Function Term 0 and 1 Configuration Register for EVENTn
    pub BFCRT012: RWRegister<u16>,

    /// Boolean Function Term 2 and 3 Configuration Register for EVENTn
    pub BFCRT232: RWRegister<u16>,

    /// Boolean Function Term 0 and 1 Configuration Register for EVENTn
    pub BFCRT013: RWRegister<u16>,

    /// Boolean Function Term 2 and 3 Configuration Register for EVENTn
    pub BFCRT233: RWRegister<u16>,
}
pub struct ResetValues {
    pub BFCRT010: u16,
    pub BFCRT230: u16,
    pub BFCRT011: u16,
    pub BFCRT231: u16,
    pub BFCRT012: u16,
    pub BFCRT232: u16,
    pub BFCRT013: u16,
    pub BFCRT233: u16,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}

/// Access functions for the AOI peripheral instance
pub mod AOI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x403b4000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AOI
    pub const reset: ResetValues = ResetValues {
        BFCRT010: 0x00000000,
        BFCRT011: 0x00000000,
        BFCRT012: 0x00000000,
        BFCRT013: 0x00000000,
        BFCRT230: 0x00000000,
        BFCRT231: 0x00000000,
        BFCRT232: 0x00000000,
        BFCRT233: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AOI_TAKEN: bool = false;

    /// Safe access to AOI
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
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AOI_TAKEN {
                None
            } else {
                AOI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AOI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AOI_TAKEN && inst.addr == INSTANCE.addr {
                AOI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AOI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AOI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AOI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AOI: *const RegisterBlock = 0x403b4000 as *const _;