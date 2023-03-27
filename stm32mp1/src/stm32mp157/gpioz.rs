///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub gpioz_moder: GPIOZ_MODER,
    ///0x04 - GPIO port output type register
    pub gpioz_otyper: GPIOZ_OTYPER,
    ///0x08 - GPIO port output speed register
    pub gpioz_ospeedr: GPIOZ_OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub gpioz_pupdr: GPIOZ_PUPDR,
    ///0x10 - GPIO port input data register
    pub gpioz_idr: GPIOZ_IDR,
    ///0x14 - GPIO port output data register
    pub gpioz_odr: GPIOZ_ODR,
    ///0x18 - GPIO port bit set/reset register
    pub gpioz_bsrr: GPIOZ_BSRR,
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    ///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    ///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
    pub gpioz_lckr: GPIOZ_LCKR,
    ///0x20 - GPIO alternate function low register
    pub gpioz_afrl: GPIOZ_AFRL,
    ///0x24 - GPIO alternate function high register
    pub gpioz_afrh: GPIOZ_AFRH,
    ///0x28 - GPIO port bit reset register
    pub gpioz_brr: GPIOZ_BRR,
    _reserved11: [u8; 0x04],
    ///0x30 - This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.
    pub gpioz_seccfgr: GPIOZ_SECCFGR,
    _reserved12: [u8; 0x0394],
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    pub gpioz_hwcfgr10: GPIOZ_HWCFGR10,
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpioz_hwcfgr9: GPIOZ_HWCFGR9,
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpioz_hwcfgr8: GPIOZ_HWCFGR8,
    ///0x3d4 - GPIO hardware configuration register 7
    pub gpioz_hwcfgr7: GPIOZ_HWCFGR7,
    ///0x3d8 - GPIO hardware configuration register 6
    pub gpioz_hwcfgr6: GPIOZ_HWCFGR6,
    ///0x3dc - GPIO hardware configuration register 5
    pub gpioz_hwcfgr5: GPIOZ_HWCFGR5,
    ///0x3e0 - GPIO hardware configuration register 4
    pub gpioz_hwcfgr4: GPIOZ_HWCFGR4,
    ///0x3e4 - GPIO hardware configuration register 3
    pub gpioz_hwcfgr3: GPIOZ_HWCFGR3,
    ///0x3e8 - GPIO hardware configuration register 2
    pub gpioz_hwcfgr2: GPIOZ_HWCFGR2,
    ///0x3ec - GPIO hardware configuration register 1
    pub gpioz_hwcfgr1: GPIOZ_HWCFGR1,
    ///0x3f0 - GPIO hardware configuration register 0
    pub gpioz_hwcfgr0: GPIOZ_HWCFGR0,
    ///0x3f4 - GPIO version register
    pub gpioz_verr: GPIOZ_VERR,
    ///0x3f8 - GPIO identification register
    pub gpioz_ipidr: GPIOZ_IPIDR,
    ///0x3fc - GPIO size identification register
    pub gpioz_sidr: GPIOZ_SIDR,
}
///GPIOZ_MODER (rw) register accessor: an alias for `Reg<GPIOZ_MODER_SPEC>`
pub type GPIOZ_MODER = crate::Reg<gpioz_moder::GPIOZ_MODER_SPEC>;
///GPIO port mode register
pub mod gpioz_moder;
///GPIOZ_OTYPER (rw) register accessor: an alias for `Reg<GPIOZ_OTYPER_SPEC>`
pub type GPIOZ_OTYPER = crate::Reg<gpioz_otyper::GPIOZ_OTYPER_SPEC>;
///GPIO port output type register
pub mod gpioz_otyper;
///GPIOZ_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOZ_OSPEEDR_SPEC>`
pub type GPIOZ_OSPEEDR = crate::Reg<gpioz_ospeedr::GPIOZ_OSPEEDR_SPEC>;
///GPIO port output speed register
pub mod gpioz_ospeedr;
///GPIOZ_PUPDR (rw) register accessor: an alias for `Reg<GPIOZ_PUPDR_SPEC>`
pub type GPIOZ_PUPDR = crate::Reg<gpioz_pupdr::GPIOZ_PUPDR_SPEC>;
///GPIO port pull-up/pull-down register
pub mod gpioz_pupdr;
///GPIOZ_IDR (r) register accessor: an alias for `Reg<GPIOZ_IDR_SPEC>`
pub type GPIOZ_IDR = crate::Reg<gpioz_idr::GPIOZ_IDR_SPEC>;
///GPIO port input data register
pub mod gpioz_idr;
///GPIOZ_ODR (rw) register accessor: an alias for `Reg<GPIOZ_ODR_SPEC>`
pub type GPIOZ_ODR = crate::Reg<gpioz_odr::GPIOZ_ODR_SPEC>;
///GPIO port output data register
pub mod gpioz_odr;
///GPIOZ_BSRR (w) register accessor: an alias for `Reg<GPIOZ_BSRR_SPEC>`
pub type GPIOZ_BSRR = crate::Reg<gpioz_bsrr::GPIOZ_BSRR_SPEC>;
///GPIO port bit set/reset register
pub mod gpioz_bsrr;
///GPIOZ_LCKR (rw) register accessor: an alias for `Reg<GPIOZ_LCKR_SPEC>`
pub type GPIOZ_LCKR = crate::Reg<gpioz_lckr::GPIOZ_LCKR_SPEC>;
///This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
pub mod gpioz_lckr;
///GPIOZ_AFRL (rw) register accessor: an alias for `Reg<GPIOZ_AFRL_SPEC>`
pub type GPIOZ_AFRL = crate::Reg<gpioz_afrl::GPIOZ_AFRL_SPEC>;
///GPIO alternate function low register
pub mod gpioz_afrl;
///GPIOZ_AFRH (rw) register accessor: an alias for `Reg<GPIOZ_AFRH_SPEC>`
pub type GPIOZ_AFRH = crate::Reg<gpioz_afrh::GPIOZ_AFRH_SPEC>;
///GPIO alternate function high register
pub mod gpioz_afrh;
///GPIOZ_BRR (w) register accessor: an alias for `Reg<GPIOZ_BRR_SPEC>`
pub type GPIOZ_BRR = crate::Reg<gpioz_brr::GPIOZ_BRR_SPEC>;
///GPIO port bit reset register
pub mod gpioz_brr;
///GPIOZ_SECCFGR (w) register accessor: an alias for `Reg<GPIOZ_SECCFGR_SPEC>`
pub type GPIOZ_SECCFGR = crate::Reg<gpioz_seccfgr::GPIOZ_SECCFGR_SPEC>;
///This register provides write access security and can be written only by a secure access. It is used to configure a selected I/O as secure. A non-secure write access to this register is discarded.
pub mod gpioz_seccfgr;
///GPIOZ_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR10_SPEC>`
pub type GPIOZ_HWCFGR10 = crate::Reg<gpioz_hwcfgr10::GPIOZ_HWCFGR10_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioz_hwcfgr10;
///GPIOZ_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR9_SPEC>`
pub type GPIOZ_HWCFGR9 = crate::Reg<gpioz_hwcfgr9::GPIOZ_HWCFGR9_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioz_hwcfgr9;
///GPIOZ_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR8_SPEC>`
pub type GPIOZ_HWCFGR8 = crate::Reg<gpioz_hwcfgr8::GPIOZ_HWCFGR8_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioz_hwcfgr8;
///GPIOZ_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR7_SPEC>`
pub type GPIOZ_HWCFGR7 = crate::Reg<gpioz_hwcfgr7::GPIOZ_HWCFGR7_SPEC>;
///GPIO hardware configuration register 7
pub mod gpioz_hwcfgr7;
///GPIOZ_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR6_SPEC>`
pub type GPIOZ_HWCFGR6 = crate::Reg<gpioz_hwcfgr6::GPIOZ_HWCFGR6_SPEC>;
///GPIO hardware configuration register 6
pub mod gpioz_hwcfgr6;
///GPIOZ_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR5_SPEC>`
pub type GPIOZ_HWCFGR5 = crate::Reg<gpioz_hwcfgr5::GPIOZ_HWCFGR5_SPEC>;
///GPIO hardware configuration register 5
pub mod gpioz_hwcfgr5;
///GPIOZ_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR4_SPEC>`
pub type GPIOZ_HWCFGR4 = crate::Reg<gpioz_hwcfgr4::GPIOZ_HWCFGR4_SPEC>;
///GPIO hardware configuration register 4
pub mod gpioz_hwcfgr4;
///GPIOZ_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR3_SPEC>`
pub type GPIOZ_HWCFGR3 = crate::Reg<gpioz_hwcfgr3::GPIOZ_HWCFGR3_SPEC>;
///GPIO hardware configuration register 3
pub mod gpioz_hwcfgr3;
///GPIOZ_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR2_SPEC>`
pub type GPIOZ_HWCFGR2 = crate::Reg<gpioz_hwcfgr2::GPIOZ_HWCFGR2_SPEC>;
///GPIO hardware configuration register 2
pub mod gpioz_hwcfgr2;
///GPIOZ_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR1_SPEC>`
pub type GPIOZ_HWCFGR1 = crate::Reg<gpioz_hwcfgr1::GPIOZ_HWCFGR1_SPEC>;
///GPIO hardware configuration register 1
pub mod gpioz_hwcfgr1;
///GPIOZ_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOZ_HWCFGR0_SPEC>`
pub type GPIOZ_HWCFGR0 = crate::Reg<gpioz_hwcfgr0::GPIOZ_HWCFGR0_SPEC>;
///GPIO hardware configuration register 0
pub mod gpioz_hwcfgr0;
///GPIOZ_VERR (r) register accessor: an alias for `Reg<GPIOZ_VERR_SPEC>`
pub type GPIOZ_VERR = crate::Reg<gpioz_verr::GPIOZ_VERR_SPEC>;
///GPIO version register
pub mod gpioz_verr;
///GPIOZ_IPIDR (r) register accessor: an alias for `Reg<GPIOZ_IPIDR_SPEC>`
pub type GPIOZ_IPIDR = crate::Reg<gpioz_ipidr::GPIOZ_IPIDR_SPEC>;
///GPIO identification register
pub mod gpioz_ipidr;
///GPIOZ_SIDR (r) register accessor: an alias for `Reg<GPIOZ_SIDR_SPEC>`
pub type GPIOZ_SIDR = crate::Reg<gpioz_sidr::GPIOZ_SIDR_SPEC>;
///GPIO size identification register
pub mod gpioz_sidr;
