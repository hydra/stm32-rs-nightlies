///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock control register
    pub cr: CR,
    ///0x04 - Internal clock sources calibration register
    pub icscr: ICSCR,
    ///0x08 - Clock configuration register
    pub cfgr: CFGR,
    ///0x0c - Clock interrupt register
    pub cir: CIR,
    ///0x10 - AHB peripheral reset register
    pub ahbrstr: AHBRSTR,
    ///0x14 - APB2 peripheral reset register
    pub apb2rstr: APB2RSTR,
    ///0x18 - APB1 peripheral reset register
    pub apb1rstr: APB1RSTR,
    ///0x1c - AHB peripheral clock enable register
    pub ahbenr: AHBENR,
    ///0x20 - APB2 peripheral clock enable register
    pub apb2enr: APB2ENR,
    ///0x24 - APB1 peripheral clock enable register
    pub apb1enr: APB1ENR,
    ///0x28 - AHB peripheral clock enable in low power mode register
    pub ahblpenr: AHBLPENR,
    ///0x2c - APB2 peripheral clock enable in low power mode register
    pub apb2lpenr: APB2LPENR,
    ///0x30 - APB1 peripheral clock enable in low power mode register
    pub apb1lpenr: APB1LPENR,
    ///0x34 - Control/status register
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
///CIR (rw) register accessor: an alias for `Reg<CIR_SPEC>`
pub type CIR = crate::Reg<cir::CIR_SPEC>;
///Clock interrupt register
pub mod cir;
///AHBRSTR (rw) register accessor: an alias for `Reg<AHBRSTR_SPEC>`
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
///AHB peripheral reset register
pub mod ahbrstr;
///APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
///APB2 peripheral reset register
pub mod apb2rstr;
///APB1RSTR (rw) register accessor: an alias for `Reg<APB1RSTR_SPEC>`
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
///APB1 peripheral reset register
pub mod apb1rstr;
///AHBENR (rw) register accessor: an alias for `Reg<AHBENR_SPEC>`
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
///AHB peripheral clock enable register
pub mod ahbenr;
///APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
///APB2 peripheral clock enable register
pub mod apb2enr;
///APB1ENR (rw) register accessor: an alias for `Reg<APB1ENR_SPEC>`
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
///APB1 peripheral clock enable register
pub mod apb1enr;
///AHBLPENR (rw) register accessor: an alias for `Reg<AHBLPENR_SPEC>`
pub type AHBLPENR = crate::Reg<ahblpenr::AHBLPENR_SPEC>;
///AHB peripheral clock enable in low power mode register
pub mod ahblpenr;
///APB2LPENR (rw) register accessor: an alias for `Reg<APB2LPENR_SPEC>`
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
///APB2 peripheral clock enable in low power mode register
pub mod apb2lpenr;
///APB1LPENR (rw) register accessor: an alias for `Reg<APB1LPENR_SPEC>`
pub type APB1LPENR = crate::Reg<apb1lpenr::APB1LPENR_SPEC>;
///APB1 peripheral clock enable in low power mode register
pub mod apb1lpenr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Control/status register
pub mod csr;
