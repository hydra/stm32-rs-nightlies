///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub gpiod_moder: GPIOD_MODER,
    ///0x04 - GPIO port output type register
    pub gpiod_otyper: GPIOD_OTYPER,
    ///0x08 - GPIO port output speed register
    pub gpiod_ospeedr: GPIOD_OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub gpiod_pupdr: GPIOD_PUPDR,
    ///0x10 - GPIO port input data register
    pub gpiod_idr: GPIOD_IDR,
    ///0x14 - GPIO port output data register
    pub gpiod_odr: GPIOD_ODR,
    ///0x18 - GPIO port bit set/reset register
    pub gpiod_bsrr: GPIOD_BSRR,
    ///0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
    ///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
    ///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
    pub gpiod_lckr: GPIOD_LCKR,
    ///0x20 - GPIO alternate function low register
    pub gpiod_afrl: GPIOD_AFRL,
    ///0x24 - GPIO alternate function high register
    pub gpiod_afrh: GPIOD_AFRH,
    ///0x28 - GPIO port bit reset register
    pub gpiod_brr: GPIOD_BRR,
    _reserved11: [u8; 0x039c],
    ///0x3c8 - For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
    pub gpiod_hwcfgr10: GPIOD_HWCFGR10,
    ///0x3cc - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpiod_hwcfgr9: GPIOD_HWCFGR9,
    ///0x3d0 - For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
    pub gpiod_hwcfgr8: GPIOD_HWCFGR8,
    ///0x3d4 - GPIO hardware configuration register 7
    pub gpiod_hwcfgr7: GPIOD_HWCFGR7,
    ///0x3d8 - GPIO hardware configuration register 6
    pub gpiod_hwcfgr6: GPIOD_HWCFGR6,
    ///0x3dc - GPIO hardware configuration register 5
    pub gpiod_hwcfgr5: GPIOD_HWCFGR5,
    ///0x3e0 - GPIO hardware configuration register 4
    pub gpiod_hwcfgr4: GPIOD_HWCFGR4,
    ///0x3e4 - GPIO hardware configuration register 3
    pub gpiod_hwcfgr3: GPIOD_HWCFGR3,
    ///0x3e8 - GPIO hardware configuration register 2
    pub gpiod_hwcfgr2: GPIOD_HWCFGR2,
    ///0x3ec - GPIO hardware configuration register 1
    pub gpiod_hwcfgr1: GPIOD_HWCFGR1,
    ///0x3f0 - GPIO hardware configuration register 0
    pub gpiod_hwcfgr0: GPIOD_HWCFGR0,
    ///0x3f4 - GPIO version register
    pub gpiod_verr: GPIOD_VERR,
    ///0x3f8 - GPIO identification register
    pub gpiod_ipidr: GPIOD_IPIDR,
    ///0x3fc - GPIO size identification register
    pub gpiod_sidr: GPIOD_SIDR,
}
///GPIOD_MODER (rw) register accessor: an alias for `Reg<GPIOD_MODER_SPEC>`
pub type GPIOD_MODER = crate::Reg<gpiod_moder::GPIOD_MODER_SPEC>;
///GPIO port mode register
pub mod gpiod_moder;
///GPIOD_OTYPER (rw) register accessor: an alias for `Reg<GPIOD_OTYPER_SPEC>`
pub type GPIOD_OTYPER = crate::Reg<gpiod_otyper::GPIOD_OTYPER_SPEC>;
///GPIO port output type register
pub mod gpiod_otyper;
///GPIOD_OSPEEDR (rw) register accessor: an alias for `Reg<GPIOD_OSPEEDR_SPEC>`
pub type GPIOD_OSPEEDR = crate::Reg<gpiod_ospeedr::GPIOD_OSPEEDR_SPEC>;
///GPIO port output speed register
pub mod gpiod_ospeedr;
///GPIOD_PUPDR (rw) register accessor: an alias for `Reg<GPIOD_PUPDR_SPEC>`
pub type GPIOD_PUPDR = crate::Reg<gpiod_pupdr::GPIOD_PUPDR_SPEC>;
///GPIO port pull-up/pull-down register
pub mod gpiod_pupdr;
///GPIOD_IDR (r) register accessor: an alias for `Reg<GPIOD_IDR_SPEC>`
pub type GPIOD_IDR = crate::Reg<gpiod_idr::GPIOD_IDR_SPEC>;
///GPIO port input data register
pub mod gpiod_idr;
///GPIOD_ODR (rw) register accessor: an alias for `Reg<GPIOD_ODR_SPEC>`
pub type GPIOD_ODR = crate::Reg<gpiod_odr::GPIOD_ODR_SPEC>;
///GPIO port output data register
pub mod gpiod_odr;
///GPIOD_BSRR (w) register accessor: an alias for `Reg<GPIOD_BSRR_SPEC>`
pub type GPIOD_BSRR = crate::Reg<gpiod_bsrr::GPIOD_BSRR_SPEC>;
///GPIO port bit set/reset register
pub mod gpiod_bsrr;
///GPIOD_LCKR (rw) register accessor: an alias for `Reg<GPIOD_LCKR_SPEC>`
pub type GPIOD_LCKR = crate::Reg<gpiod_lckr::GPIOD_LCKR_SPEC>;
///This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\]
///is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\]
///must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).
pub mod gpiod_lckr;
///GPIOD_AFRL (rw) register accessor: an alias for `Reg<GPIOD_AFRL_SPEC>`
pub type GPIOD_AFRL = crate::Reg<gpiod_afrl::GPIOD_AFRL_SPEC>;
///GPIO alternate function low register
pub mod gpiod_afrl;
///GPIOD_AFRH (rw) register accessor: an alias for `Reg<GPIOD_AFRH_SPEC>`
pub type GPIOD_AFRH = crate::Reg<gpiod_afrh::GPIOD_AFRH_SPEC>;
///GPIO alternate function high register
pub mod gpiod_afrh;
///GPIOD_BRR (w) register accessor: an alias for `Reg<GPIOD_BRR_SPEC>`
pub type GPIOD_BRR = crate::Reg<gpiod_brr::GPIOD_BRR_SPEC>;
///GPIO port bit reset register
pub mod gpiod_brr;
///GPIOD_HWCFGR10 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR10_SPEC>`
pub type GPIOD_HWCFGR10 = crate::Reg<gpiod_hwcfgr10::GPIOD_HWCFGR10_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:
pub mod gpiod_hwcfgr10;
///GPIOD_HWCFGR9 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR9_SPEC>`
pub type GPIOD_HWCFGR9 = crate::Reg<gpiod_hwcfgr9::GPIOD_HWCFGR9_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiod_hwcfgr9;
///GPIOD_HWCFGR8 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR8_SPEC>`
pub type GPIOD_HWCFGR8 = crate::Reg<gpiod_hwcfgr8::GPIOD_HWCFGR8_SPEC>;
///For GPIOA, B, C, D, E, F, G, H, I, and GPIOJ: For GPIOK and GPIOZ:
pub mod gpiod_hwcfgr8;
///GPIOD_HWCFGR7 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR7_SPEC>`
pub type GPIOD_HWCFGR7 = crate::Reg<gpiod_hwcfgr7::GPIOD_HWCFGR7_SPEC>;
///GPIO hardware configuration register 7
pub mod gpiod_hwcfgr7;
///GPIOD_HWCFGR6 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR6_SPEC>`
pub type GPIOD_HWCFGR6 = crate::Reg<gpiod_hwcfgr6::GPIOD_HWCFGR6_SPEC>;
///GPIO hardware configuration register 6
pub mod gpiod_hwcfgr6;
///GPIOD_HWCFGR5 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR5_SPEC>`
pub type GPIOD_HWCFGR5 = crate::Reg<gpiod_hwcfgr5::GPIOD_HWCFGR5_SPEC>;
///GPIO hardware configuration register 5
pub mod gpiod_hwcfgr5;
///GPIOD_HWCFGR4 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR4_SPEC>`
pub type GPIOD_HWCFGR4 = crate::Reg<gpiod_hwcfgr4::GPIOD_HWCFGR4_SPEC>;
///GPIO hardware configuration register 4
pub mod gpiod_hwcfgr4;
///GPIOD_HWCFGR3 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR3_SPEC>`
pub type GPIOD_HWCFGR3 = crate::Reg<gpiod_hwcfgr3::GPIOD_HWCFGR3_SPEC>;
///GPIO hardware configuration register 3
pub mod gpiod_hwcfgr3;
///GPIOD_HWCFGR2 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR2_SPEC>`
pub type GPIOD_HWCFGR2 = crate::Reg<gpiod_hwcfgr2::GPIOD_HWCFGR2_SPEC>;
///GPIO hardware configuration register 2
pub mod gpiod_hwcfgr2;
///GPIOD_HWCFGR1 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR1_SPEC>`
pub type GPIOD_HWCFGR1 = crate::Reg<gpiod_hwcfgr1::GPIOD_HWCFGR1_SPEC>;
///GPIO hardware configuration register 1
pub mod gpiod_hwcfgr1;
///GPIOD_HWCFGR0 (r) register accessor: an alias for `Reg<GPIOD_HWCFGR0_SPEC>`
pub type GPIOD_HWCFGR0 = crate::Reg<gpiod_hwcfgr0::GPIOD_HWCFGR0_SPEC>;
///GPIO hardware configuration register 0
pub mod gpiod_hwcfgr0;
///GPIOD_VERR (r) register accessor: an alias for `Reg<GPIOD_VERR_SPEC>`
pub type GPIOD_VERR = crate::Reg<gpiod_verr::GPIOD_VERR_SPEC>;
///GPIO version register
pub mod gpiod_verr;
///GPIOD_IPIDR (r) register accessor: an alias for `Reg<GPIOD_IPIDR_SPEC>`
pub type GPIOD_IPIDR = crate::Reg<gpiod_ipidr::GPIOD_IPIDR_SPEC>;
///GPIO identification register
pub mod gpiod_ipidr;
///GPIOD_SIDR (r) register accessor: an alias for `Reg<GPIOD_SIDR_SPEC>`
pub type GPIOD_SIDR = crate::Reg<gpiod_sidr::GPIOD_SIDR_SPEC>;
///GPIO size identification register
pub mod gpiod_sidr;
