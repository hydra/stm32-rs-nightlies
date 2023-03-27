///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RCC clock control register
    pub cr: CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - RCC HSI calibration register
    pub hsicfgr: HSICFGR,
    ///0x14 - RCC clock recovery RC register
    pub crrcr: CRRCR,
    ///0x18 - RCC CSI calibration register
    pub csicfgr: CSICFGR,
    ///0x1c - RCC clock configuration register
    pub cfgr: CFGR,
    ///0x20 - RCC CPU domain clock configuration register 2
    pub cfgr2: CFGR2,
    _reserved6: [u8; 0x04],
    ///0x28 - RCC PLL clock source selection register
    pub pll1cfgr: PLL1CFGR,
    ///0x2c - RCC PLL clock source selection register
    pub pll2cfgr: PLL2CFGR,
    _reserved8: [u8; 0x04],
    ///0x34 - RCC PLL1 dividers register
    pub pll1divr: PLL1DIVR,
    ///0x38 - RCC PLL1 fractional divider register
    pub pll1fracr: PLL1FRACR,
    ///0x3c - RCC PLL1 dividers register
    pub pll2divr: PLL2DIVR,
    ///0x40 - RCC PLL2 fractional divider register
    pub pll2fracr: PLL2FRACR,
    _reserved12: [u8; 0x0c],
    ///0x50 - RCC clock source interrupt enable register
    pub cier: CIER,
    ///0x54 - RCC clock source interrupt flag register
    pub cifr: CIFR,
    ///0x58 - RCC clock source interrupt clear register
    pub cicr: CICR,
    _reserved15: [u8; 0x04],
    ///0x60 - RCC AHB1 reset register
    pub ahb1rstr: AHB1RSTR,
    ///0x64 - RCC AHB2 peripheral reset register
    pub ahb2rstr: AHB2RSTR,
    _reserved17: [u8; 0x0c],
    ///0x74 - RCC APB1 peripheral low reset register
    pub apb1lrstr: APB1LRSTR,
    ///0x78 - RCC APB1 peripheral high reset register
    pub apb1hrstr: APB1HRSTR,
    ///0x7c - RCC APB2 peripheral reset register
    pub apb2rstr: APB2RSTR,
    ///0x80 - RCC APB3 peripheral reset register
    pub apb3rstr: APB3RSTR,
    _reserved21: [u8; 0x04],
    ///0x88 - RCC AHB1 peripherals clock register
    pub ahb1enr: AHB1ENR,
    ///0x8c - RCC AHB2 peripheral clock register
    pub ahb2enr: AHB2ENR,
    _reserved23: [u8; 0x0c],
    ///0x9c - RCC APB1 peripheral clock register
    pub apb1lenr: APB1LENR,
    ///0xa0 - RCC APB1 peripheral clock register
    pub apb1henr: APB1HENR,
    ///0xa4 - RCC APB2 peripheral clock register
    pub apb2enr: APB2ENR,
    ///0xa8 - RCC APB3 peripheral clock register
    pub apb3enr: APB3ENR,
    _reserved27: [u8; 0x04],
    ///0xb0 - RCC AHB1 sleep clock register
    pub ahb1lpenr: AHB1LPENR,
    ///0xb4 - RCC AHB2 sleep clock register
    pub ahb2lpenr: AHB2LPENR,
    _reserved29: [u8; 0x0c],
    ///0xc4 - RCC APB1 sleep clock register
    pub apb1llpenr: APB1LLPENR,
    ///0xc8 - RCC APB1 sleep clock register
    pub apb1hlpenr: APB1HLPENR,
    ///0xcc - RCC APB2 sleep clock register
    pub apb2lpenr: APB2LPENR,
    ///0xd0 - RCC APB3 sleep clock register
    pub apb3lpenr: APB3LPENR,
    _reserved33: [u8; 0x04],
    ///0xd8 - RCC kernel clock configuration register
    pub ccipr1: CCIPR1,
    ///0xdc - RCC kernel clock configuration register
    pub ccipr2: CCIPR2,
    ///0xe0 - RCC kernel clock configuration register
    pub ccipr3: CCIPR3,
    ///0xe4 - RCC kernel clock configuration register
    pub ccipr4: CCIPR4,
    ///0xe8 - RCC kernel clock configuration register
    pub ccipr5: CCIPR5,
    _reserved38: [u8; 0x04],
    ///0xf0 - RCC Backup domain control register
    pub bdcr: BDCR,
    ///0xf4 - RCC reset status register
    pub rsr: RSR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///RCC clock control register
pub mod cr;
///HSICFGR (rw) register accessor: an alias for `Reg<HSICFGR_SPEC>`
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGR_SPEC>;
///RCC HSI calibration register
pub mod hsicfgr;
///CRRCR (r) register accessor: an alias for `Reg<CRRCR_SPEC>`
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
///RCC clock recovery RC register
pub mod crrcr;
///CSICFGR (rw) register accessor: an alias for `Reg<CSICFGR_SPEC>`
pub type CSICFGR = crate::Reg<csicfgr::CSICFGR_SPEC>;
///RCC CSI calibration register
pub mod csicfgr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///RCC clock configuration register
pub mod cfgr;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///RCC CPU domain clock configuration register 2
pub mod cfgr2;
///PLL1CFGR (rw) register accessor: an alias for `Reg<PLL1CFGR_SPEC>`
pub type PLL1CFGR = crate::Reg<pll1cfgr::PLL1CFGR_SPEC>;
///RCC PLL clock source selection register
pub mod pll1cfgr;
///PLL2CFGR (rw) register accessor: an alias for `Reg<PLL2CFGR_SPEC>`
pub type PLL2CFGR = crate::Reg<pll2cfgr::PLL2CFGR_SPEC>;
///RCC PLL clock source selection register
pub mod pll2cfgr;
///PLL1DIVR (rw) register accessor: an alias for `Reg<PLL1DIVR_SPEC>`
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVR_SPEC>;
///RCC PLL1 dividers register
pub mod pll1divr;
///PLL1FRACR (rw) register accessor: an alias for `Reg<PLL1FRACR_SPEC>`
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACR_SPEC>;
///RCC PLL1 fractional divider register
pub mod pll1fracr;
///PLL2DIVR (rw) register accessor: an alias for `Reg<PLL2DIVR_SPEC>`
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVR_SPEC>;
///RCC PLL1 dividers register
pub mod pll2divr;
///PLL2FRACR (rw) register accessor: an alias for `Reg<PLL2FRACR_SPEC>`
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACR_SPEC>;
///RCC PLL2 fractional divider register
pub mod pll2fracr;
///CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`
pub type CIER = crate::Reg<cier::CIER_SPEC>;
///RCC clock source interrupt enable register
pub mod cier;
///CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
///RCC clock source interrupt flag register
pub mod cifr;
///CICR (rw) register accessor: an alias for `Reg<CICR_SPEC>`
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
///RCC clock source interrupt clear register
pub mod cicr;
///AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
///RCC AHB1 reset register
pub mod ahb1rstr;
///AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
///RCC AHB2 peripheral reset register
pub mod ahb2rstr;
///APB1LRSTR (rw) register accessor: an alias for `Reg<APB1LRSTR_SPEC>`
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTR_SPEC>;
///RCC APB1 peripheral low reset register
pub mod apb1lrstr;
///APB1HRSTR (rw) register accessor: an alias for `Reg<APB1HRSTR_SPEC>`
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTR_SPEC>;
///RCC APB1 peripheral high reset register
pub mod apb1hrstr;
///APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
///RCC APB2 peripheral reset register
pub mod apb2rstr;
///APB3RSTR (rw) register accessor: an alias for `Reg<APB3RSTR_SPEC>`
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
///RCC APB3 peripheral reset register
pub mod apb3rstr;
///AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
///RCC AHB1 peripherals clock register
pub mod ahb1enr;
///AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
///RCC AHB2 peripheral clock register
pub mod ahb2enr;
///APB1LENR (rw) register accessor: an alias for `Reg<APB1LENR_SPEC>`
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENR_SPEC>;
///RCC APB1 peripheral clock register
pub mod apb1lenr;
///APB1HENR (rw) register accessor: an alias for `Reg<APB1HENR_SPEC>`
pub type APB1HENR = crate::Reg<apb1henr::APB1HENR_SPEC>;
///RCC APB1 peripheral clock register
pub mod apb1henr;
///APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
///RCC APB2 peripheral clock register
pub mod apb2enr;
///APB3ENR (rw) register accessor: an alias for `Reg<APB3ENR_SPEC>`
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
///RCC APB3 peripheral clock register
pub mod apb3enr;
///AHB1LPENR (rw) register accessor: an alias for `Reg<AHB1LPENR_SPEC>`
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
///RCC AHB1 sleep clock register
pub mod ahb1lpenr;
///AHB2LPENR (rw) register accessor: an alias for `Reg<AHB2LPENR_SPEC>`
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
///RCC AHB2 sleep clock register
pub mod ahb2lpenr;
///APB1LLPENR (rw) register accessor: an alias for `Reg<APB1LLPENR_SPEC>`
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENR_SPEC>;
///RCC APB1 sleep clock register
pub mod apb1llpenr;
///APB1HLPENR (rw) register accessor: an alias for `Reg<APB1HLPENR_SPEC>`
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>;
///RCC APB1 sleep clock register
pub mod apb1hlpenr;
///APB2LPENR (rw) register accessor: an alias for `Reg<APB2LPENR_SPEC>`
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
///RCC APB2 sleep clock register
pub mod apb2lpenr;
///APB3LPENR (rw) register accessor: an alias for `Reg<APB3LPENR_SPEC>`
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENR_SPEC>;
///RCC APB3 sleep clock register
pub mod apb3lpenr;
///CCIPR1 (rw) register accessor: an alias for `Reg<CCIPR1_SPEC>`
pub type CCIPR1 = crate::Reg<ccipr1::CCIPR1_SPEC>;
///RCC kernel clock configuration register
pub mod ccipr1;
///CCIPR2 (rw) register accessor: an alias for `Reg<CCIPR2_SPEC>`
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2_SPEC>;
///RCC kernel clock configuration register
pub mod ccipr2;
///CCIPR3 (rw) register accessor: an alias for `Reg<CCIPR3_SPEC>`
pub type CCIPR3 = crate::Reg<ccipr3::CCIPR3_SPEC>;
///RCC kernel clock configuration register
pub mod ccipr3;
///CCIPR4 (rw) register accessor: an alias for `Reg<CCIPR4_SPEC>`
pub type CCIPR4 = crate::Reg<ccipr4::CCIPR4_SPEC>;
///RCC kernel clock configuration register
pub mod ccipr4;
///CCIPR5 (rw) register accessor: an alias for `Reg<CCIPR5_SPEC>`
pub type CCIPR5 = crate::Reg<ccipr5::CCIPR5_SPEC>;
///RCC kernel clock configuration register
pub mod ccipr5;
///BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///RCC Backup domain control register
pub mod bdcr;
///RSR (rw) register accessor: an alias for `Reg<RSR_SPEC>`
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
///RCC reset status register
pub mod rsr;
