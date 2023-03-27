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
    ///0x14 - CFGR2
    pub cfgr2: CFGR2,
    ///0x18 - SCSR
    pub scsr: SCSR,
    ///0x1c - SKR
    pub skr: SKR,
    ///0x20 - SWPR
    pub swpr: SWPR,
    ///0x24 - SWPR2
    pub swpr2: SWPR2,
    _reserved10: [u8; 0x04],
    ///0x2c - RSSCMDR
    pub rsscmdr: RSSCMDR,
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
///SCSR (rw) register accessor: an alias for `Reg<SCSR_SPEC>`
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
///SCSR
pub mod scsr;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///CFGR2
pub mod cfgr2;
///SWPR (w) register accessor: an alias for `Reg<SWPR_SPEC>`
pub type SWPR = crate::Reg<swpr::SWPR_SPEC>;
///SWPR
pub mod swpr;
///SKR (w) register accessor: an alias for `Reg<SKR_SPEC>`
pub type SKR = crate::Reg<skr::SKR_SPEC>;
///SKR
pub mod skr;
///SWPR2 (w) register accessor: an alias for `Reg<SWPR2_SPEC>`
pub type SWPR2 = crate::Reg<swpr2::SWPR2_SPEC>;
///SWPR2
pub mod swpr2;
///RSSCMDR (rw) register accessor: an alias for `Reg<RSSCMDR_SPEC>`
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDR_SPEC>;
///RSSCMDR
pub mod rsscmdr;
