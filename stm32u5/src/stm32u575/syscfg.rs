///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SYSCFG secure configuration register
    pub seccfgr: SECCFGR,
    ///0x04 - configuration register 1
    pub cfgr1: CFGR1,
    ///0x08 - FPU interrupt mask register
    pub fpuimr: FPUIMR,
    ///0x0c - SYSCFG CPU non-secure lock register
    pub cnslckr: CNSLCKR,
    ///0x10 - SYSCFG CPU secure lock register
    pub cslockr: CSLOCKR,
    ///0x14 - configuration register 2
    pub cfgr2: CFGR2,
    ///0x18 - memory erase status register
    pub mesr: MESR,
    ///0x1c - compensation cell control/status register
    pub cccsr: CCCSR,
    ///0x20 - compensation cell value register
    pub ccvr: CCVR,
    ///0x24 - compensation cell code register
    pub cccr: CCCR,
    _reserved10: [u8; 0x04],
    ///0x2c - RSS command register
    pub rsscmdr: RSSCMDR,
    _reserved11: [u8; 0x40],
    ///0x70 - USB Type C and Power Delivery register
    pub ucpdr: UCPDR,
}
///SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
///SYSCFG secure configuration register
pub mod seccfgr;
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///configuration register 1
pub mod cfgr1;
///FPUIMR (rw) register accessor: an alias for `Reg<FPUIMR_SPEC>`
pub type FPUIMR = crate::Reg<fpuimr::FPUIMR_SPEC>;
///FPU interrupt mask register
pub mod fpuimr;
///CNSLCKR (rw) register accessor: an alias for `Reg<CNSLCKR_SPEC>`
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKR_SPEC>;
///SYSCFG CPU non-secure lock register
pub mod cnslckr;
///CSLOCKR (rw) register accessor: an alias for `Reg<CSLOCKR_SPEC>`
pub type CSLOCKR = crate::Reg<cslockr::CSLOCKR_SPEC>;
///SYSCFG CPU secure lock register
pub mod cslockr;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///configuration register 2
pub mod cfgr2;
///MESR (rw) register accessor: an alias for `Reg<MESR_SPEC>`
pub type MESR = crate::Reg<mesr::MESR_SPEC>;
///memory erase status register
pub mod mesr;
///CCCSR (rw) register accessor: an alias for `Reg<CCCSR_SPEC>`
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
///compensation cell control/status register
pub mod cccsr;
///CCVR (r) register accessor: an alias for `Reg<CCVR_SPEC>`
pub type CCVR = crate::Reg<ccvr::CCVR_SPEC>;
///compensation cell value register
pub mod ccvr;
///CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///compensation cell code register
pub mod cccr;
///RSSCMDR (rw) register accessor: an alias for `Reg<RSSCMDR_SPEC>`
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDR_SPEC>;
///RSS command register
pub mod rsscmdr;
///UCPDR (rw) register accessor: an alias for `Reg<UCPDR_SPEC>`
pub type UCPDR = crate::Reg<ucpdr::UCPDR_SPEC>;
///USB Type C and Power Delivery register
pub mod ucpdr;
