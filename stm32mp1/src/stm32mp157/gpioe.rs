///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub gpioe_moder: GPIOE_MODER,
    ///0x04 - GPIO port output type register
    pub gpioe_otyper: GPIOE_OTYPER,
    ///0x08 - GPIO port output speed register
    pub gpioe_ospeedr: GPIOE_OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub gpioe_pupdr: GPIOE_PUPDR,
    ///0x10 - GPIO port input data register
    pub gpioe_idr: GPIOE_IDR,
    ///0x14 - GPIO port output data register
    pub gpioe_odr: GPIOE_ODR,
    ///0x18 - GPIO port bit set/reset register
    pub gpioe_bsrr: GPIOE_BSRR,
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    ///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    ///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
    pub gpioe_lckr: GPIOE_LCKR,
    ///0x20 - GPIO alternate function low register
    pub gpioe_afrl: GPIOE_AFRL,
    ///0x24 - GPIO alternate function high register
    pub gpioe_afrh: GPIOE_AFRH,
    ///0x28 - GPIO port bit reset register
    pub gpioe_brr: GPIOE_BRR,
    _reserved11: [u8; 0x039c],
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    pub gpioe_hwcfgr10: GPIOE_HWCFGR10,
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpioe_hwcfgr9: GPIOE_HWCFGR9,
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpioe_hwcfgr8: GPIOE_HWCFGR8,
    ///0x3d4 - GPIO hardware configuration register 7
    pub gpioe_hwcfgr7: GPIOE_HWCFGR7,
    ///0x3d8 - GPIO hardware configuration register 6
    pub gpioe_hwcfgr6: GPIOE_HWCFGR6,
    ///0x3dc - GPIO hardware configuration register 5
    pub gpioe_hwcfgr5: GPIOE_HWCFGR5,
    ///0x3e0 - GPIO hardware configuration register 4
    pub gpioe_hwcfgr4: GPIOE_HWCFGR4,
    ///0x3e4 - GPIO hardware configuration register 3
    pub gpioe_hwcfgr3: GPIOE_HWCFGR3,
    ///0x3e8 - GPIO hardware configuration register 2
    pub gpioe_hwcfgr2: GPIOE_HWCFGR2,
    ///0x3ec - GPIO hardware configuration register 1
    pub gpioe_hwcfgr1: GPIOE_HWCFGR1,
    ///0x3f0 - GPIO hardware configuration register 0
    pub gpioe_hwcfgr0: GPIOE_HWCFGR0,
    ///0x3f4 - GPIO version register
    pub gpioe_verr: GPIOE_VERR,
    ///0x3f8 - GPIO identification register
    pub gpioe_ipidr: GPIOE_IPIDR,
    ///0x3fc - GPIO size identification register
    pub gpioe_sidr: GPIOE_SIDR,
}
///GPIOE_MODER (rw) register accessor: an alias for `Reg<GPIOE_MODER_SPEC>`
pub type GPIOE_MODER = crate::Reg<gpioe_moder::GPIOE_MODER_SPEC>;
///GPIO port mode register
pub mod gpioe_moder;
///GPIOE_OTYPER (rw) register accessor: an alias for `Reg<GPIOE_OTYPER_SPEC>`
pub type GPIOE_OTYPER = crate::Reg<gpioe_otyper::GPIOE_OTYPER_SPEC>;
///GPIO port output type register
pub mod gpioe_otyper;
///GPIOE_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOE_OSPEEDR_SPEC>`
pub type GPIOE_OSPEEDR = crate::Reg<gpioe_ospeedr::GPIOE_OSPEEDR_SPEC>;
///GPIO port output speed register
pub mod gpioe_ospeedr;
///GPIOE_PUPDR (rw) register accessor: an alias for `Reg<GPIOE_PUPDR_SPEC>`
pub type GPIOE_PUPDR = crate::Reg<gpioe_pupdr::GPIOE_PUPDR_SPEC>;
///GPIO port pull-up/pull-down register
pub mod gpioe_pupdr;
///GPIOE_IDR (r) register accessor: an alias for `Reg<GPIOE_IDR_SPEC>`
pub type GPIOE_IDR = crate::Reg<gpioe_idr::GPIOE_IDR_SPEC>;
///GPIO port input data register
pub mod gpioe_idr;
///GPIOE_ODR (rw) register accessor: an alias for `Reg<GPIOE_ODR_SPEC>`
pub type GPIOE_ODR = crate::Reg<gpioe_odr::GPIOE_ODR_SPEC>;
///GPIO port output data register
pub mod gpioe_odr;
///GPIOE_BSRR (w) register accessor: an alias for `Reg<GPIOE_BSRR_SPEC>`
pub type GPIOE_BSRR = crate::Reg<gpioe_bsrr::GPIOE_BSRR_SPEC>;
///GPIO port bit set/reset register
pub mod gpioe_bsrr;
///GPIOE_LCKR (rw) register accessor: an alias for `Reg<GPIOE_LCKR_SPEC>`
pub type GPIOE_LCKR = crate::Reg<gpioe_lckr::GPIOE_LCKR_SPEC>;
///This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
pub mod gpioe_lckr;
///GPIOE_AFRL (rw) register accessor: an alias for `Reg<GPIOE_AFRL_SPEC>`
pub type GPIOE_AFRL = crate::Reg<gpioe_afrl::GPIOE_AFRL_SPEC>;
///GPIO alternate function low register
pub mod gpioe_afrl;
///GPIOE_AFRH (rw) register accessor: an alias for `Reg<GPIOE_AFRH_SPEC>`
pub type GPIOE_AFRH = crate::Reg<gpioe_afrh::GPIOE_AFRH_SPEC>;
///GPIO alternate function high register
pub mod gpioe_afrh;
///GPIOE_BRR (w) register accessor: an alias for `Reg<GPIOE_BRR_SPEC>`
pub type GPIOE_BRR = crate::Reg<gpioe_brr::GPIOE_BRR_SPEC>;
///GPIO port bit reset register
pub mod gpioe_brr;
///GPIOE_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR10_SPEC>`
pub type GPIOE_HWCFGR10 = crate::Reg<gpioe_hwcfgr10::GPIOE_HWCFGR10_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpioe_hwcfgr10;
///GPIOE_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR9_SPEC>`
pub type GPIOE_HWCFGR9 = crate::Reg<gpioe_hwcfgr9::GPIOE_HWCFGR9_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioe_hwcfgr9;
///GPIOE_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR8_SPEC>`
pub type GPIOE_HWCFGR8 = crate::Reg<gpioe_hwcfgr8::GPIOE_HWCFGR8_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpioe_hwcfgr8;
///GPIOE_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR7_SPEC>`
pub type GPIOE_HWCFGR7 = crate::Reg<gpioe_hwcfgr7::GPIOE_HWCFGR7_SPEC>;
///GPIO hardware configuration register 7
pub mod gpioe_hwcfgr7;
///GPIOE_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR6_SPEC>`
pub type GPIOE_HWCFGR6 = crate::Reg<gpioe_hwcfgr6::GPIOE_HWCFGR6_SPEC>;
///GPIO hardware configuration register 6
pub mod gpioe_hwcfgr6;
///GPIOE_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR5_SPEC>`
pub type GPIOE_HWCFGR5 = crate::Reg<gpioe_hwcfgr5::GPIOE_HWCFGR5_SPEC>;
///GPIO hardware configuration register 5
pub mod gpioe_hwcfgr5;
///GPIOE_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR4_SPEC>`
pub type GPIOE_HWCFGR4 = crate::Reg<gpioe_hwcfgr4::GPIOE_HWCFGR4_SPEC>;
///GPIO hardware configuration register 4
pub mod gpioe_hwcfgr4;
///GPIOE_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR3_SPEC>`
pub type GPIOE_HWCFGR3 = crate::Reg<gpioe_hwcfgr3::GPIOE_HWCFGR3_SPEC>;
///GPIO hardware configuration register 3
pub mod gpioe_hwcfgr3;
///GPIOE_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR2_SPEC>`
pub type GPIOE_HWCFGR2 = crate::Reg<gpioe_hwcfgr2::GPIOE_HWCFGR2_SPEC>;
///GPIO hardware configuration register 2
pub mod gpioe_hwcfgr2;
///GPIOE_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR1_SPEC>`
pub type GPIOE_HWCFGR1 = crate::Reg<gpioe_hwcfgr1::GPIOE_HWCFGR1_SPEC>;
///GPIO hardware configuration register 1
pub mod gpioe_hwcfgr1;
///GPIOE_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOE_HWCFGR0_SPEC>`
pub type GPIOE_HWCFGR0 = crate::Reg<gpioe_hwcfgr0::GPIOE_HWCFGR0_SPEC>;
///GPIO hardware configuration register 0
pub mod gpioe_hwcfgr0;
///GPIOE_VERR (r) register accessor: an alias for `Reg<GPIOE_VERR_SPEC>`
pub type GPIOE_VERR = crate::Reg<gpioe_verr::GPIOE_VERR_SPEC>;
///GPIO version register
pub mod gpioe_verr;
///GPIOE_IPIDR (r) register accessor: an alias for `Reg<GPIOE_IPIDR_SPEC>`
pub type GPIOE_IPIDR = crate::Reg<gpioe_ipidr::GPIOE_IPIDR_SPEC>;
///GPIO identification register
pub mod gpioe_ipidr;
///GPIOE_SIDR (r) register accessor: an alias for `Reg<GPIOE_SIDR_SPEC>`
pub type GPIOE_SIDR = crate::Reg<gpioe_sidr::GPIOE_SIDR_SPEC>;
///GPIO size identification register
pub mod gpioe_sidr;
