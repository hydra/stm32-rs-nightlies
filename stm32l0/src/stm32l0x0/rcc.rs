///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock control register
    pub cr: CR,
    ///0x04 - Internal clock sources calibration register
    pub icscr: ICSCR,
    _reserved2: [u8; 0x04],
    ///0x0c - Clock configuration register
    pub cfgr: CFGR,
    ///0x10 - Clock interrupt enable register
    pub cier: CIER,
    ///0x14 - Clock interrupt flag register
    pub cifr: CIFR,
    ///0x18 - Clock interrupt clear register
    pub cicr: CICR,
    ///0x1c - GPIO reset register
    pub ioprstr: IOPRSTR,
    ///0x20 - AHB peripheral reset register
    pub ahbrstr: AHBRSTR,
    ///0x24 - APB2 peripheral reset register
    pub apb2rstr: APB2RSTR,
    ///0x28 - APB1 peripheral reset register
    pub apb1rstr: APB1RSTR,
    ///0x2c - GPIO clock enable register
    pub iopenr: IOPENR,
    ///0x30 - AHB peripheral clock enable register
    pub ahbenr: AHBENR,
    ///0x34 - APB2 peripheral clock enable register
    pub apb2enr: APB2ENR,
    ///0x38 - APB1 peripheral clock enable register
    pub apb1enr: APB1ENR,
    ///0x3c - GPIO clock enable in sleep mode register
    pub iopsmen: IOPSMEN,
    ///0x40 - AHB peripheral clock enable in sleep mode register
    pub ahbsmenr: AHBSMENR,
    ///0x44 - APB2 peripheral clock enable in sleep mode register
    pub apb2smenr: APB2SMENR,
    ///0x48 - APB1 peripheral clock enable in sleep mode register
    pub apb1smenr: APB1SMENR,
    ///0x4c - Clock configuration register
    pub ccipr: CCIPR,
    ///0x50 - Control and status register
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
///CIER (r) register accessor: an alias for `Reg<CIER_SPEC>`
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
///GPIO reset register
pub mod ioprstr;
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
///IOPENR (rw) register accessor: an alias for `Reg<IOPENR_SPEC>`
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
///GPIO clock enable register
pub mod iopenr;
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
///IOPSMEN (rw) register accessor: an alias for `Reg<IOPSMEN_SPEC>`
pub type IOPSMEN = crate::Reg<iopsmen::IOPSMEN_SPEC>;
///GPIO clock enable in sleep mode register
pub mod iopsmen;
///AHBSMENR (rw) register accessor: an alias for `Reg<AHBSMENR_SPEC>`
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
///AHB peripheral clock enable in sleep mode register
pub mod ahbsmenr;
///APB2SMENR (rw) register accessor: an alias for `Reg<APB2SMENR_SPEC>`
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENR_SPEC>;
///APB2 peripheral clock enable in sleep mode register
pub mod apb2smenr;
///APB1SMENR (rw) register accessor: an alias for `Reg<APB1SMENR_SPEC>`
pub type APB1SMENR = crate::Reg<apb1smenr::APB1SMENR_SPEC>;
///APB1 peripheral clock enable in sleep mode register
pub mod apb1smenr;
///CCIPR (rw) register accessor: an alias for `Reg<CCIPR_SPEC>`
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
///Clock configuration register
pub mod ccipr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Control and status register
pub mod csr;
