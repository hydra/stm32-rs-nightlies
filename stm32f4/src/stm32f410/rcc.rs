///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - clock control register
    pub cr: CR,
    ///0x04 - PLL configuration register
    pub pllcfgr: PLLCFGR,
    ///0x08 - clock configuration register
    pub cfgr: CFGR,
    ///0x0c - clock interrupt register
    pub cir: CIR,
    ///0x10 - AHB1 peripheral reset register
    pub ahb1rstr: AHB1RSTR,
    _reserved5: [u8; 0x0c],
    ///0x20 - APB1 peripheral reset register
    pub apb1rstr: APB1RSTR,
    ///0x24 - APB2 peripheral reset register
    pub apb2rstr: APB2RSTR,
    _reserved7: [u8; 0x08],
    ///0x30 - AHB1 peripheral clock register
    pub ahb1enr: AHB1ENR,
    _reserved8: [u8; 0x0c],
    ///0x40 - APB1 peripheral clock enable register
    pub apb1enr: APB1ENR,
    ///0x44 - APB2 peripheral clock enable register
    pub apb2enr: APB2ENR,
    _reserved10: [u8; 0x08],
    ///0x50 - AHB1 peripheral clock enable in low power mode register
    pub ahb1lpenr: AHB1LPENR,
    _reserved11: [u8; 0x0c],
    ///0x60 - APB1 peripheral clock enable in low power mode register
    pub apb1lpenr: APB1LPENR,
    ///0x64 - APB2 peripheral clock enabled in low power mode register
    pub apb2lpenr: APB2LPENR,
    _reserved13: [u8; 0x08],
    ///0x70 - Backup domain control register
    pub bdcr: BDCR,
    ///0x74 - clock control &amp; status register
    pub csr: CSR,
    _reserved15: [u8; 0x08],
    ///0x80 - spread spectrum clock generation register
    pub sscgr: SSCGR,
    _reserved16: [u8; 0x08],
    ///0x8c - DCKCFGR register
    pub dckcfgr: DCKCFGR,
    _reserved17: [u8; 0x04],
    ///0x94 - DCKCFGR2 register
    pub dckcfgr2: DCKCFGR2,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///clock control register
pub mod cr;
///PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
///PLL configuration register
pub mod pllcfgr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///clock configuration register
pub mod cfgr;
///CIR (rw) register accessor: an alias for `Reg<CIR_SPEC>`
pub type CIR = crate::Reg<cir::CIR_SPEC>;
///clock interrupt register
pub mod cir;
///AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
///AHB1 peripheral reset register
pub mod ahb1rstr;
///APB1RSTR (rw) register accessor: an alias for `Reg<APB1RSTR_SPEC>`
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
///APB1 peripheral reset register
pub mod apb1rstr;
///APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
///APB2 peripheral reset register
pub mod apb2rstr;
///AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
///AHB1 peripheral clock register
pub mod ahb1enr;
///APB1ENR (rw) register accessor: an alias for `Reg<APB1ENR_SPEC>`
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
///APB1 peripheral clock enable register
pub mod apb1enr;
///APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
///APB2 peripheral clock enable register
pub mod apb2enr;
///AHB1LPENR (rw) register accessor: an alias for `Reg<AHB1LPENR_SPEC>`
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
///AHB1 peripheral clock enable in low power mode register
pub mod ahb1lpenr;
///APB1LPENR (rw) register accessor: an alias for `Reg<APB1LPENR_SPEC>`
pub type APB1LPENR = crate::Reg<apb1lpenr::APB1LPENR_SPEC>;
///APB1 peripheral clock enable in low power mode register
pub mod apb1lpenr;
///APB2LPENR (rw) register accessor: an alias for `Reg<APB2LPENR_SPEC>`
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
///APB2 peripheral clock enabled in low power mode register
pub mod apb2lpenr;
///BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///Backup domain control register
pub mod bdcr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///clock control &amp; status register
pub mod csr;
///SSCGR (rw) register accessor: an alias for `Reg<SSCGR_SPEC>`
pub type SSCGR = crate::Reg<sscgr::SSCGR_SPEC>;
///spread spectrum clock generation register
pub mod sscgr;
///DCKCFGR (rw) register accessor: an alias for `Reg<DCKCFGR_SPEC>`
pub type DCKCFGR = crate::Reg<dckcfgr::DCKCFGR_SPEC>;
///DCKCFGR register
pub mod dckcfgr;
///DCKCFGR2 (rw) register accessor: an alias for `Reg<DCKCFGR2_SPEC>`
pub type DCKCFGR2 = crate::Reg<dckcfgr2::DCKCFGR2_SPEC>;
///DCKCFGR2 register
pub mod dckcfgr2;
