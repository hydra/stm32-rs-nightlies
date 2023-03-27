///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock control register
    pub cr: CR,
    ///0x04 - Internal clock sources calibration register
    pub icscr: ICSCR,
    ///0x08 - Clock configuration register
    pub cfgr: CFGR,
    ///0x0c - PLL configuration register
    pub pllcfgr: PLLCFGR,
    _reserved4: [u8; 0x04],
    ///0x14 - RCC clock recovery RC register
    pub crrcr: CRRCR,
    ///0x18 - Clock interrupt enable register
    pub cier: CIER,
    ///0x1c - Clock interrupt flag register
    pub cifr: CIFR,
    ///0x20 - Clock interrupt clear register
    pub cicr: CICR,
    ///0x24 - I/O port reset register
    pub ioprstr: IOPRSTR,
    ///0x28 - AHB peripheral reset register
    pub ahbrstr: AHBRSTR,
    ///0x2c - APB peripheral reset register 1
    pub apbrstr1: APBRSTR1,
    ///0x30 - APB peripheral reset register 2
    pub apbrstr2: APBRSTR2,
    ///0x34 - GPIO clock enable register
    pub iopenr: IOPENR,
    ///0x38 - AHB peripheral clock enable register
    pub ahbenr: AHBENR,
    ///0x3c - APB peripheral clock enable register 1
    pub apbenr1: APBENR1,
    ///0x40 - APB peripheral clock enable register 2
    pub apbenr2: APBENR2,
    ///0x44 - GPIO in Sleep mode clock enable register
    pub iopsmenr: IOPSMENR,
    ///0x48 - AHB peripheral clock enable in Sleep mode register
    pub ahbsmenr: AHBSMENR,
    ///0x4c - APB peripheral clock enable in Sleep mode register 1
    pub apbsmenr1: APBSMENR1,
    ///0x50 - APB peripheral clock enable in Sleep mode register 2
    pub apbsmenr2: APBSMENR2,
    ///0x54 - Peripherals independent clock configuration register
    pub ccipr: CCIPR,
    ///0x58 - Peripherals independent clock configuration register 2
    pub ccipr2: CCIPR2,
    ///0x5c - RTC domain control register
    pub bdcr: BDCR,
    ///0x60 - Control/status register
    pub csr: CSR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Clock control register
pub mod cr;
///ICSCR (rw) register accessor: an alias for `Reg<ICSCR_SPEC>`
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
///Internal clock sources calibration register
pub mod icscr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///Clock configuration register
pub mod cfgr;
///PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
///PLL configuration register
pub mod pllcfgr;
///CRRCR (r) register accessor: an alias for `Reg<CRRCR_SPEC>`
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
///RCC clock recovery RC register
pub mod crrcr;
///CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`
pub type CIER = crate::Reg<cier::CIER_SPEC>;
///Clock interrupt enable register
pub mod cier;
///CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
///Clock interrupt flag register
pub mod cifr;
///CICR (w) register accessor: an alias for `Reg<CICR_SPEC>`
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
///Clock interrupt clear register
pub mod cicr;
///IOPRSTR (rw) register accessor: an alias for `Reg<IOPRSTR_SPEC>`
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
///I/O port reset register
pub mod ioprstr;
///AHBRSTR (rw) register accessor: an alias for `Reg<AHBRSTR_SPEC>`
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
///AHB peripheral reset register
pub mod ahbrstr;
///APBRSTR1 (rw) register accessor: an alias for `Reg<APBRSTR1_SPEC>`
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1_SPEC>;
///APB peripheral reset register 1
pub mod apbrstr1;
///APBRSTR2 (rw) register accessor: an alias for `Reg<APBRSTR2_SPEC>`
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2_SPEC>;
///APB peripheral reset register 2
pub mod apbrstr2;
///IOPENR (rw) register accessor: an alias for `Reg<IOPENR_SPEC>`
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
///GPIO clock enable register
pub mod iopenr;
///AHBENR (rw) register accessor: an alias for `Reg<AHBENR_SPEC>`
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
///AHB peripheral clock enable register
pub mod ahbenr;
///APBENR1 (rw) register accessor: an alias for `Reg<APBENR1_SPEC>`
pub type APBENR1 = crate::Reg<apbenr1::APBENR1_SPEC>;
///APB peripheral clock enable register 1
pub mod apbenr1;
///APBENR2 (rw) register accessor: an alias for `Reg<APBENR2_SPEC>`
pub type APBENR2 = crate::Reg<apbenr2::APBENR2_SPEC>;
///APB peripheral clock enable register 2
pub mod apbenr2;
///IOPSMENR (rw) register accessor: an alias for `Reg<IOPSMENR_SPEC>`
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENR_SPEC>;
///GPIO in Sleep mode clock enable register
pub mod iopsmenr;
///AHBSMENR (rw) register accessor: an alias for `Reg<AHBSMENR_SPEC>`
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
///AHB peripheral clock enable in Sleep mode register
pub mod ahbsmenr;
///APBSMENR1 (rw) register accessor: an alias for `Reg<APBSMENR1_SPEC>`
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1_SPEC>;
///APB peripheral clock enable in Sleep mode register 1
pub mod apbsmenr1;
///APBSMENR2 (rw) register accessor: an alias for `Reg<APBSMENR2_SPEC>`
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2_SPEC>;
///APB peripheral clock enable in Sleep mode register 2
pub mod apbsmenr2;
///CCIPR (rw) register accessor: an alias for `Reg<CCIPR_SPEC>`
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
///Peripherals independent clock configuration register
pub mod ccipr;
///CCIPR2 (rw) register accessor: an alias for `Reg<CCIPR2_SPEC>`
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2_SPEC>;
///Peripherals independent clock configuration register 2
pub mod ccipr2;
///BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///RTC domain control register
pub mod bdcr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Control/status register
pub mod csr;
