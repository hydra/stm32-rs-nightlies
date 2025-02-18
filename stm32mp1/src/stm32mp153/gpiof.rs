///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub gpiof_moder: GPIOF_MODER,
    ///0x04 - GPIO port output type register
    pub gpiof_otyper: GPIOF_OTYPER,
    ///0x08 - GPIO port output speed register
    pub gpiof_ospeedr: GPIOF_OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub gpiof_pupdr: GPIOF_PUPDR,
    ///0x10 - GPIO port input data register
    pub gpiof_idr: GPIOF_IDR,
    ///0x14 - GPIO port output data register
    pub gpiof_odr: GPIOF_ODR,
    ///0x18 - GPIO port bit set/reset register
    pub gpiof_bsrr: GPIOF_BSRR,
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    ///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    ///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
    pub gpiof_lckr: GPIOF_LCKR,
    ///0x20 - GPIO alternate function low register
    pub gpiof_afrl: GPIOF_AFRL,
    ///0x24 - GPIO alternate function high register
    pub gpiof_afrh: GPIOF_AFRH,
    ///0x28 - GPIO port bit reset register
    pub gpiof_brr: GPIOF_BRR,
    _reserved11: [u8; 0x039c],
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    pub gpiof_hwcfgr10: GPIOF_HWCFGR10,
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpiof_hwcfgr9: GPIOF_HWCFGR9,
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpiof_hwcfgr8: GPIOF_HWCFGR8,
    ///0x3d4 - GPIO hardware configuration register 7
    pub gpiof_hwcfgr7: GPIOF_HWCFGR7,
    ///0x3d8 - GPIO hardware configuration register 6
    pub gpiof_hwcfgr6: GPIOF_HWCFGR6,
    ///0x3dc - GPIO hardware configuration register 5
    pub gpiof_hwcfgr5: GPIOF_HWCFGR5,
    ///0x3e0 - GPIO hardware configuration register 4
    pub gpiof_hwcfgr4: GPIOF_HWCFGR4,
    ///0x3e4 - GPIO hardware configuration register 3
    pub gpiof_hwcfgr3: GPIOF_HWCFGR3,
    ///0x3e8 - GPIO hardware configuration register 2
    pub gpiof_hwcfgr2: GPIOF_HWCFGR2,
    ///0x3ec - GPIO hardware configuration register 1
    pub gpiof_hwcfgr1: GPIOF_HWCFGR1,
    ///0x3f0 - GPIO hardware configuration register 0
    pub gpiof_hwcfgr0: GPIOF_HWCFGR0,
    ///0x3f4 - GPIO version register
    pub gpiof_verr: GPIOF_VERR,
    ///0x3f8 - GPIO identification register
    pub gpiof_ipidr: GPIOF_IPIDR,
    ///0x3fc - GPIO size identification register
    pub gpiof_sidr: GPIOF_SIDR,
}
///GPIOF_MODER (rw) register accessor: an alias for `Reg<GPIOF_MODER_SPEC>`
pub type GPIOF_MODER = crate::Reg<gpiof_moder::GPIOF_MODER_SPEC>;
///GPIO port mode register
pub mod gpiof_moder;
///GPIOF_OTYPER (rw) register accessor: an alias for `Reg<GPIOF_OTYPER_SPEC>`
pub type GPIOF_OTYPER = crate::Reg<gpiof_otyper::GPIOF_OTYPER_SPEC>;
///GPIO port output type register
pub mod gpiof_otyper;
///GPIOF_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOF_OSPEEDR_SPEC>`
pub type GPIOF_OSPEEDR = crate::Reg<gpiof_ospeedr::GPIOF_OSPEEDR_SPEC>;
///GPIO port output speed register
pub mod gpiof_ospeedr;
///GPIOF_PUPDR (rw) register accessor: an alias for `Reg<GPIOF_PUPDR_SPEC>`
pub type GPIOF_PUPDR = crate::Reg<gpiof_pupdr::GPIOF_PUPDR_SPEC>;
///GPIO port pull-up/pull-down register
pub mod gpiof_pupdr;
///GPIOF_IDR (r) register accessor: an alias for `Reg<GPIOF_IDR_SPEC>`
pub type GPIOF_IDR = crate::Reg<gpiof_idr::GPIOF_IDR_SPEC>;
///GPIO port input data register
pub mod gpiof_idr;
///GPIOF_ODR (rw) register accessor: an alias for `Reg<GPIOF_ODR_SPEC>`
pub type GPIOF_ODR = crate::Reg<gpiof_odr::GPIOF_ODR_SPEC>;
///GPIO port output data register
pub mod gpiof_odr;
///GPIOF_BSRR (w) register accessor: an alias for `Reg<GPIOF_BSRR_SPEC>`
pub type GPIOF_BSRR = crate::Reg<gpiof_bsrr::GPIOF_BSRR_SPEC>;
///GPIO port bit set/reset register
pub mod gpiof_bsrr;
///GPIOF_LCKR (rw) register accessor: an alias for `Reg<GPIOF_LCKR_SPEC>`
pub type GPIOF_LCKR = crate::Reg<gpiof_lckr::GPIOF_LCKR_SPEC>;
///This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
pub mod gpiof_lckr;
///GPIOF_AFRL (rw) register accessor: an alias for `Reg<GPIOF_AFRL_SPEC>`
pub type GPIOF_AFRL = crate::Reg<gpiof_afrl::GPIOF_AFRL_SPEC>;
///GPIO alternate function low register
pub mod gpiof_afrl;
///GPIOF_AFRH (rw) register accessor: an alias for `Reg<GPIOF_AFRH_SPEC>`
pub type GPIOF_AFRH = crate::Reg<gpiof_afrh::GPIOF_AFRH_SPEC>;
///GPIO alternate function high register
pub mod gpiof_afrh;
///GPIOF_BRR (w) register accessor: an alias for `Reg<GPIOF_BRR_SPEC>`
pub type GPIOF_BRR = crate::Reg<gpiof_brr::GPIOF_BRR_SPEC>;
///GPIO port bit reset register
pub mod gpiof_brr;
///GPIOF_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR10_SPEC>`
pub type GPIOF_HWCFGR10 = crate::Reg<gpiof_hwcfgr10::GPIOF_HWCFGR10_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpiof_hwcfgr10;
///GPIOF_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR9_SPEC>`
pub type GPIOF_HWCFGR9 = crate::Reg<gpiof_hwcfgr9::GPIOF_HWCFGR9_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiof_hwcfgr9;
///GPIOF_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR8_SPEC>`
pub type GPIOF_HWCFGR8 = crate::Reg<gpiof_hwcfgr8::GPIOF_HWCFGR8_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiof_hwcfgr8;
///GPIOF_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR7_SPEC>`
pub type GPIOF_HWCFGR7 = crate::Reg<gpiof_hwcfgr7::GPIOF_HWCFGR7_SPEC>;
///GPIO hardware configuration register 7
pub mod gpiof_hwcfgr7;
///GPIOF_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR6_SPEC>`
pub type GPIOF_HWCFGR6 = crate::Reg<gpiof_hwcfgr6::GPIOF_HWCFGR6_SPEC>;
///GPIO hardware configuration register 6
pub mod gpiof_hwcfgr6;
///GPIOF_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR5_SPEC>`
pub type GPIOF_HWCFGR5 = crate::Reg<gpiof_hwcfgr5::GPIOF_HWCFGR5_SPEC>;
///GPIO hardware configuration register 5
pub mod gpiof_hwcfgr5;
///GPIOF_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR4_SPEC>`
pub type GPIOF_HWCFGR4 = crate::Reg<gpiof_hwcfgr4::GPIOF_HWCFGR4_SPEC>;
///GPIO hardware configuration register 4
pub mod gpiof_hwcfgr4;
///GPIOF_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR3_SPEC>`
pub type GPIOF_HWCFGR3 = crate::Reg<gpiof_hwcfgr3::GPIOF_HWCFGR3_SPEC>;
///GPIO hardware configuration register 3
pub mod gpiof_hwcfgr3;
///GPIOF_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR2_SPEC>`
pub type GPIOF_HWCFGR2 = crate::Reg<gpiof_hwcfgr2::GPIOF_HWCFGR2_SPEC>;
///GPIO hardware configuration register 2
pub mod gpiof_hwcfgr2;
///GPIOF_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR1_SPEC>`
pub type GPIOF_HWCFGR1 = crate::Reg<gpiof_hwcfgr1::GPIOF_HWCFGR1_SPEC>;
///GPIO hardware configuration register 1
pub mod gpiof_hwcfgr1;
///GPIOF_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOF_HWCFGR0_SPEC>`
pub type GPIOF_HWCFGR0 = crate::Reg<gpiof_hwcfgr0::GPIOF_HWCFGR0_SPEC>;
///GPIO hardware configuration register 0
pub mod gpiof_hwcfgr0;
///GPIOF_VERR (r) register accessor: an alias for `Reg<GPIOF_VERR_SPEC>`
pub type GPIOF_VERR = crate::Reg<gpiof_verr::GPIOF_VERR_SPEC>;
///GPIO version register
pub mod gpiof_verr;
///GPIOF_IPIDR (r) register accessor: an alias for `Reg<GPIOF_IPIDR_SPEC>`
pub type GPIOF_IPIDR = crate::Reg<gpiof_ipidr::GPIOF_IPIDR_SPEC>;
///GPIO identification register
pub mod gpiof_ipidr;
///GPIOF_SIDR (r) register accessor: an alias for `Reg<GPIOF_SIDR_SPEC>`
pub type GPIOF_SIDR = crate::Reg<gpiof_sidr::GPIOF_SIDR_SPEC>;
///GPIO size identification register
pub mod gpiof_sidr;
