///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub gpiob_moder: GPIOB_MODER,
    ///0x04 - GPIO port output type register
    pub gpiob_otyper: GPIOB_OTYPER,
    ///0x08 - GPIO port output speed register
    pub gpiob_ospeedr: GPIOB_OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub gpiob_pupdr: GPIOB_PUPDR,
    ///0x10 - GPIO port input data register
    pub gpiob_idr: GPIOB_IDR,
    ///0x14 - GPIO port output data register
    pub gpiob_odr: GPIOB_ODR,
    ///0x18 - GPIO port bit set/reset register
    pub gpiob_bsrr: GPIOB_BSRR,
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    ///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    ///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
    pub gpiob_lckr: GPIOB_LCKR,
    ///0x20 - GPIO alternate function low register
    pub gpiob_afrl: GPIOB_AFRL,
    ///0x24 - GPIO alternate function high register
    pub gpiob_afrh: GPIOB_AFRH,
    ///0x28 - GPIO port bit reset register
    pub gpiob_brr: GPIOB_BRR,
    _reserved11: [u8; 0x039c],
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    pub gpiob_hwcfgr10: GPIOB_HWCFGR10,
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpiob_hwcfgr9: GPIOB_HWCFGR9,
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpiob_hwcfgr8: GPIOB_HWCFGR8,
    ///0x3d4 - GPIO hardware configuration register 7
    pub gpiob_hwcfgr7: GPIOB_HWCFGR7,
    ///0x3d8 - GPIO hardware configuration register 6
    pub gpiob_hwcfgr6: GPIOB_HWCFGR6,
    ///0x3dc - GPIO hardware configuration register 5
    pub gpiob_hwcfgr5: GPIOB_HWCFGR5,
    ///0x3e0 - GPIO hardware configuration register 4
    pub gpiob_hwcfgr4: GPIOB_HWCFGR4,
    ///0x3e4 - GPIO hardware configuration register 3
    pub gpiob_hwcfgr3: GPIOB_HWCFGR3,
    ///0x3e8 - GPIO hardware configuration register 2
    pub gpiob_hwcfgr2: GPIOB_HWCFGR2,
    ///0x3ec - GPIO hardware configuration register 1
    pub gpiob_hwcfgr1: GPIOB_HWCFGR1,
    ///0x3f0 - GPIO hardware configuration register 0
    pub gpiob_hwcfgr0: GPIOB_HWCFGR0,
    ///0x3f4 - GPIO version register
    pub gpiob_verr: GPIOB_VERR,
    ///0x3f8 - GPIO identification register
    pub gpiob_ipidr: GPIOB_IPIDR,
    ///0x3fc - GPIO size identification register
    pub gpiob_sidr: GPIOB_SIDR,
}
///GPIOB_MODER (rw) register accessor: an alias for `Reg<GPIOB_MODER_SPEC>`
pub type GPIOB_MODER = crate::Reg<gpiob_moder::GPIOB_MODER_SPEC>;
///GPIO port mode register
pub mod gpiob_moder;
///GPIOB_OTYPER (rw) register accessor: an alias for `Reg<GPIOB_OTYPER_SPEC>`
pub type GPIOB_OTYPER = crate::Reg<gpiob_otyper::GPIOB_OTYPER_SPEC>;
///GPIO port output type register
pub mod gpiob_otyper;
///GPIOB_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOB_OSPEEDR_SPEC>`
pub type GPIOB_OSPEEDR = crate::Reg<gpiob_ospeedr::GPIOB_OSPEEDR_SPEC>;
///GPIO port output speed register
pub mod gpiob_ospeedr;
///GPIOB_PUPDR (rw) register accessor: an alias for `Reg<GPIOB_PUPDR_SPEC>`
pub type GPIOB_PUPDR = crate::Reg<gpiob_pupdr::GPIOB_PUPDR_SPEC>;
///GPIO port pull-up/pull-down register
pub mod gpiob_pupdr;
///GPIOB_IDR (r) register accessor: an alias for `Reg<GPIOB_IDR_SPEC>`
pub type GPIOB_IDR = crate::Reg<gpiob_idr::GPIOB_IDR_SPEC>;
///GPIO port input data register
pub mod gpiob_idr;
///GPIOB_ODR (rw) register accessor: an alias for `Reg<GPIOB_ODR_SPEC>`
pub type GPIOB_ODR = crate::Reg<gpiob_odr::GPIOB_ODR_SPEC>;
///GPIO port output data register
pub mod gpiob_odr;
///GPIOB_BSRR (w) register accessor: an alias for `Reg<GPIOB_BSRR_SPEC>`
pub type GPIOB_BSRR = crate::Reg<gpiob_bsrr::GPIOB_BSRR_SPEC>;
///GPIO port bit set/reset register
pub mod gpiob_bsrr;
///GPIOB_LCKR (rw) register accessor: an alias for `Reg<GPIOB_LCKR_SPEC>`
pub type GPIOB_LCKR = crate::Reg<gpiob_lckr::GPIOB_LCKR_SPEC>;
///This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
pub mod gpiob_lckr;
///GPIOB_AFRL (rw) register accessor: an alias for `Reg<GPIOB_AFRL_SPEC>`
pub type GPIOB_AFRL = crate::Reg<gpiob_afrl::GPIOB_AFRL_SPEC>;
///GPIO alternate function low register
pub mod gpiob_afrl;
///GPIOB_AFRH (rw) register accessor: an alias for `Reg<GPIOB_AFRH_SPEC>`
pub type GPIOB_AFRH = crate::Reg<gpiob_afrh::GPIOB_AFRH_SPEC>;
///GPIO alternate function high register
pub mod gpiob_afrh;
///GPIOB_BRR (w) register accessor: an alias for `Reg<GPIOB_BRR_SPEC>`
pub type GPIOB_BRR = crate::Reg<gpiob_brr::GPIOB_BRR_SPEC>;
///GPIO port bit reset register
pub mod gpiob_brr;
///GPIOB_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR10_SPEC>`
pub type GPIOB_HWCFGR10 = crate::Reg<gpiob_hwcfgr10::GPIOB_HWCFGR10_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpiob_hwcfgr10;
///GPIOB_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR9_SPEC>`
pub type GPIOB_HWCFGR9 = crate::Reg<gpiob_hwcfgr9::GPIOB_HWCFGR9_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiob_hwcfgr9;
///GPIOB_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR8_SPEC>`
pub type GPIOB_HWCFGR8 = crate::Reg<gpiob_hwcfgr8::GPIOB_HWCFGR8_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiob_hwcfgr8;
///GPIOB_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR7_SPEC>`
pub type GPIOB_HWCFGR7 = crate::Reg<gpiob_hwcfgr7::GPIOB_HWCFGR7_SPEC>;
///GPIO hardware configuration register 7
pub mod gpiob_hwcfgr7;
///GPIOB_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR6_SPEC>`
pub type GPIOB_HWCFGR6 = crate::Reg<gpiob_hwcfgr6::GPIOB_HWCFGR6_SPEC>;
///GPIO hardware configuration register 6
pub mod gpiob_hwcfgr6;
///GPIOB_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR5_SPEC>`
pub type GPIOB_HWCFGR5 = crate::Reg<gpiob_hwcfgr5::GPIOB_HWCFGR5_SPEC>;
///GPIO hardware configuration register 5
pub mod gpiob_hwcfgr5;
///GPIOB_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR4_SPEC>`
pub type GPIOB_HWCFGR4 = crate::Reg<gpiob_hwcfgr4::GPIOB_HWCFGR4_SPEC>;
///GPIO hardware configuration register 4
pub mod gpiob_hwcfgr4;
///GPIOB_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR3_SPEC>`
pub type GPIOB_HWCFGR3 = crate::Reg<gpiob_hwcfgr3::GPIOB_HWCFGR3_SPEC>;
///GPIO hardware configuration register 3
pub mod gpiob_hwcfgr3;
///GPIOB_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR2_SPEC>`
pub type GPIOB_HWCFGR2 = crate::Reg<gpiob_hwcfgr2::GPIOB_HWCFGR2_SPEC>;
///GPIO hardware configuration register 2
pub mod gpiob_hwcfgr2;
///GPIOB_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR1_SPEC>`
pub type GPIOB_HWCFGR1 = crate::Reg<gpiob_hwcfgr1::GPIOB_HWCFGR1_SPEC>;
///GPIO hardware configuration register 1
pub mod gpiob_hwcfgr1;
///GPIOB_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOB_HWCFGR0_SPEC>`
pub type GPIOB_HWCFGR0 = crate::Reg<gpiob_hwcfgr0::GPIOB_HWCFGR0_SPEC>;
///GPIO hardware configuration register 0
pub mod gpiob_hwcfgr0;
///GPIOB_VERR (r) register accessor: an alias for `Reg<GPIOB_VERR_SPEC>`
pub type GPIOB_VERR = crate::Reg<gpiob_verr::GPIOB_VERR_SPEC>;
///GPIO version register
pub mod gpiob_verr;
///GPIOB_IPIDR (r) register accessor: an alias for `Reg<GPIOB_IPIDR_SPEC>`
pub type GPIOB_IPIDR = crate::Reg<gpiob_ipidr::GPIOB_IPIDR_SPEC>;
///GPIO identification register
pub mod gpiob_ipidr;
///GPIOB_SIDR (r) register accessor: an alias for `Reg<GPIOB_SIDR_SPEC>`
pub type GPIOB_SIDR = crate::Reg<gpiob_sidr::GPIOB_SIDR_SPEC>;
///GPIO size identification register
pub mod gpiob_sidr;
