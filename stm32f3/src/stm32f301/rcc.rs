///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock control register
    pub cr: CR,
    ///0x04 - Clock configuration register (RCC_CFGR)
    pub cfgr: CFGR,
    ///0x08 - Clock interrupt register (RCC_CIR)
    pub cir: CIR,
    ///0x0c - APB2 peripheral reset register (RCC_APB2RSTR)
    pub apb2rstr: APB2RSTR,
    ///0x10 - APB1 peripheral reset register (RCC_APB1RSTR)
    pub apb1rstr: APB1RSTR,
    ///0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)
    pub ahbenr: AHBENR,
    ///0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)
    pub apb2enr: APB2ENR,
    ///0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)
    pub apb1enr: APB1ENR,
    ///0x20 - Backup domain control register (RCC_BDCR)
    pub bdcr: BDCR,
    ///0x24 - Control/status register (RCC_CSR)
    pub csr: CSR,
    ///0x28 - AHB peripheral reset register
    pub ahbrstr: AHBRSTR,
    ///0x2c - Clock configuration register 2
    pub cfgr2: CFGR2,
    ///0x30 - Clock configuration register 3
    pub cfgr3: CFGR3,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Clock control register
pub mod cr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///Clock configuration register (RCC_CFGR)
pub mod cfgr;
///CIR (rw) register accessor: an alias for `Reg<CIR_SPEC>`
pub type CIR = crate::Reg<cir::CIR_SPEC>;
///Clock interrupt register (RCC_CIR)
pub mod cir;
///APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
///APB2 peripheral reset register (RCC_APB2RSTR)
pub mod apb2rstr;
///APB1RSTR (rw) register accessor: an alias for `Reg<APB1RSTR_SPEC>`
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
///APB1 peripheral reset register (RCC_APB1RSTR)
pub mod apb1rstr;
///AHBENR (rw) register accessor: an alias for `Reg<AHBENR_SPEC>`
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
///AHB Peripheral Clock enable register (RCC_AHBENR)
pub mod ahbenr;
///APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
///APB2 peripheral clock enable register (RCC_APB2ENR)
pub mod apb2enr;
///APB1ENR (rw) register accessor: an alias for `Reg<APB1ENR_SPEC>`
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
///APB1 peripheral clock enable register (RCC_APB1ENR)
pub mod apb1enr;
///BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///Backup domain control register (RCC_BDCR)
pub mod bdcr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Control/status register (RCC_CSR)
pub mod csr;
///AHBRSTR (rw) register accessor: an alias for `Reg<AHBRSTR_SPEC>`
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
///AHB peripheral reset register
pub mod ahbrstr;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///Clock configuration register 2
pub mod cfgr2;
///CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
///Clock configuration register 3
pub mod cfgr3;
