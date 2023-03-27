///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub gpioh_moder: GPIOH_MODER,
    ///0x04 - GPIO port output type register
    pub gpioh_otyper: GPIOH_OTYPER,
    ///0x08 - GPIO port output speed register
    pub gpioh_ospeedr: GPIOH_OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub gpioh_pupdr: GPIOH_PUPDR,
    ///0x10 - GPIO port input data register
    pub gpioh_idr: GPIOH_IDR,
    ///0x14 - GPIO port output data register
    pub gpioh_odr: GPIOH_ODR,
    ///0x18 - GPIO port bit set/reset register
    pub gpioh_bsrr: GPIOH_BSRR,
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    ///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    ///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
    pub gpioh_lckr: GPIOH_LCKR,
    ///0x20 - GPIO alternate function low register
    pub gpioh_afrl: GPIOH_AFRL,
    ///0x24 - GPIO alternate function high register
    pub gpioh_afrh: GPIOH_AFRH,
    ///0x28 - GPIO port bit reset register
    pub gpioh_brr: GPIOH_BRR,
    _reserved11: [u8; 0x039c],
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    pub gpioh_hwcfgr10: GPIOH_HWCFGR10,
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpioh_hwcfgr9: GPIOH_HWCFGR9,
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpioh_hwcfgr8: GPIOH_HWCFGR8,
    ///0x3d4 - GPIO hardware configuration register 7
    pub gpioh_hwcfgr7: GPIOH_HWCFGR7,
    ///0x3d8 - GPIO hardware configuration register 6
    pub gpioh_hwcfgr6: GPIOH_HWCFGR6,
    ///0x3dc - GPIO hardware configuration register 5
    pub gpioh_hwcfgr5: GPIOH_HWCFGR5,
    ///0x3e0 - GPIO hardware configuration register 4
    pub gpioh_hwcfgr4: GPIOH_HWCFGR4,
    ///0x3e4 - GPIO hardware configuration register 3
    pub gpioh_hwcfgr3: GPIOH_HWCFGR3,
    ///0x3e8 - GPIO hardware configuration register 2
    pub gpioh_hwcfgr2: GPIOH_HWCFGR2,
    ///0x3ec - GPIO hardware configuration register 1
    pub gpioh_hwcfgr1: GPIOH_HWCFGR1,
    ///0x3f0 - GPIO hardware configuration register 0
    pub gpioh_hwcfgr0: GPIOH_HWCFGR0,
    ///0x3f4 - GPIO version register
    pub gpioh_verr: GPIOH_VERR,
    ///0x3f8 - GPIO identification register
    pub gpioh_ipidr: GPIOH_IPIDR,
    ///0x3fc - GPIO size identification register
    pub gpioh_sidr: GPIOH_SIDR,
}
///GPIOH_MODER (rw) register accessor: an alias for `Reg<GPIOH_MODER_SPEC>`
pub type GPIOH_MODER = crate::Reg<gpioh_moder::GPIOH_MODER_SPEC>;
///GPIO port mode register
pub mod gpioh_moder;
///GPIOH_OTYPER (rw) register accessor: an alias for `Reg<GPIOH_OTYPER_SPEC>`
pub type GPIOH_OTYPER = crate::Reg<gpioh_otyper::GPIOH_OTYPER_SPEC>;
///GPIO port output type register
pub mod gpioh_otyper;
///GPIOH_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOH_OSPEEDR_SPEC>`
pub type GPIOH_OSPEEDR = crate::Reg<gpioh_ospeedr::GPIOH_OSPEEDR_SPEC>;
///GPIO port output speed register
pub mod gpioh_ospeedr;
///GPIOH_PUPDR (rw) register accessor: an alias for `Reg<GPIOH_PUPDR_SPEC>`
pub type GPIOH_PUPDR = crate::Reg<gpioh_pupdr::GPIOH_PUPDR_SPEC>;
///GPIO port pull-up/pull-down register
pub mod gpioh_pupdr;
///GPIOH_IDR (r) register accessor: an alias for `Reg<GPIOH_IDR_SPEC>`
pub type GPIOH_IDR = crate::Reg<gpioh_idr::GPIOH_IDR_SPEC>;
///GPIO port input data register
pub mod gpioh_idr;
///GPIOH_ODR (rw) register accessor: an alias for `Reg<GPIOH_ODR_SPEC>`
pub type GPIOH_ODR = crate::Reg<gpioh_odr::GPIOH_ODR_SPEC>;
///GPIO port output data register
pub mod gpioh_odr;
///GPIOH_BSRR (w) register accessor: an alias for `Reg<GPIOH_BSRR_SPEC>`
pub type GPIOH_BSRR = crate::Reg<gpioh_bsrr::GPIOH_BSRR_SPEC>;
///GPIO port bit set/reset register
pub mod gpioh_bsrr;
///GPIOH_LCKR (rw) register accessor: an alias for `Reg<GPIOH_LCKR_SPEC>`
pub type GPIOH_LCKR = crate::Reg<gpioh_lckr::GPIOH_LCKR_SPEC>;
///This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
pub mod gpioh_lckr;
///GPIOH_AFRL (rw) register accessor: an alias for `Reg<GPIOH_AFRL_SPEC>`
pub type GPIOH_AFRL = crate::Reg<gpioh_afrl::GPIOH_AFRL_SPEC>;
///GPIO alternate function low register
pub mod gpioh_afrl;
///GPIOH_AFRH (rw) register accessor: an alias for `Reg<GPIOH_AFRH_SPEC>`
pub type GPIOH_AFRH = crate::Reg<gpioh_afrh::GPIOH_AFRH_SPEC>;
///GPIO alternate function high register
pub mod gpioh_afrh;
///GPIOH_BRR (w) register accessor: an alias for `Reg<GPIOH_BRR_SPEC>`
pub type GPIOH_BRR = crate::Reg<gpioh_brr::GPIOH_BRR_SPEC>;
///GPIO port bit reset register
pub mod gpioh_brr;
///GPIOH_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR10_SPEC>`
pub type GPIOH_HWCFGR10 = crate::Reg<gpioh_hwcfgr10::GPIOH_HWCFGR10_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioh_hwcfgr10;
///GPIOH_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR9_SPEC>`
pub type GPIOH_HWCFGR9 = crate::Reg<gpioh_hwcfgr9::GPIOH_HWCFGR9_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioh_hwcfgr9;
///GPIOH_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR8_SPEC>`
pub type GPIOH_HWCFGR8 = crate::Reg<gpioh_hwcfgr8::GPIOH_HWCFGR8_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioh_hwcfgr8;
///GPIOH_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR7_SPEC>`
pub type GPIOH_HWCFGR7 = crate::Reg<gpioh_hwcfgr7::GPIOH_HWCFGR7_SPEC>;
///GPIO hardware configuration register 7
pub mod gpioh_hwcfgr7;
///GPIOH_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR6_SPEC>`
pub type GPIOH_HWCFGR6 = crate::Reg<gpioh_hwcfgr6::GPIOH_HWCFGR6_SPEC>;
///GPIO hardware configuration register 6
pub mod gpioh_hwcfgr6;
///GPIOH_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR5_SPEC>`
pub type GPIOH_HWCFGR5 = crate::Reg<gpioh_hwcfgr5::GPIOH_HWCFGR5_SPEC>;
///GPIO hardware configuration register 5
pub mod gpioh_hwcfgr5;
///GPIOH_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR4_SPEC>`
pub type GPIOH_HWCFGR4 = crate::Reg<gpioh_hwcfgr4::GPIOH_HWCFGR4_SPEC>;
///GPIO hardware configuration register 4
pub mod gpioh_hwcfgr4;
///GPIOH_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR3_SPEC>`
pub type GPIOH_HWCFGR3 = crate::Reg<gpioh_hwcfgr3::GPIOH_HWCFGR3_SPEC>;
///GPIO hardware configuration register 3
pub mod gpioh_hwcfgr3;
///GPIOH_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR2_SPEC>`
pub type GPIOH_HWCFGR2 = crate::Reg<gpioh_hwcfgr2::GPIOH_HWCFGR2_SPEC>;
///GPIO hardware configuration register 2
pub mod gpioh_hwcfgr2;
///GPIOH_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR1_SPEC>`
pub type GPIOH_HWCFGR1 = crate::Reg<gpioh_hwcfgr1::GPIOH_HWCFGR1_SPEC>;
///GPIO hardware configuration register 1
pub mod gpioh_hwcfgr1;
///GPIOH_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOH_HWCFGR0_SPEC>`
pub type GPIOH_HWCFGR0 = crate::Reg<gpioh_hwcfgr0::GPIOH_HWCFGR0_SPEC>;
///GPIO hardware configuration register 0
pub mod gpioh_hwcfgr0;
///GPIOH_VERR (r) register accessor: an alias for `Reg<GPIOH_VERR_SPEC>`
pub type GPIOH_VERR = crate::Reg<gpioh_verr::GPIOH_VERR_SPEC>;
///GPIO version register
pub mod gpioh_verr;
///GPIOH_IPIDR (r) register accessor: an alias for `Reg<GPIOH_IPIDR_SPEC>`
pub type GPIOH_IPIDR = crate::Reg<gpioh_ipidr::GPIOH_IPIDR_SPEC>;
///GPIO identification register
pub mod gpioh_ipidr;
///GPIOH_SIDR (r) register accessor: an alias for `Reg<GPIOH_SIDR_SPEC>`
pub type GPIOH_SIDR = crate::Reg<gpioh_sidr::GPIOH_SIDR_SPEC>;
///GPIO size identification register
pub mod gpioh_sidr;
