///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    ///0x10 - SBS temporal isolation control register
    pub hdplcr: HDPLCR,
    ///0x14 - SBS temporal isolation status register
    pub hdplsr: HDPLSR,
    ///0x18 - SBS next HDPL control register
    pub nexthdplcr: NEXTHDPLCR,
    _reserved3: [u8; 0x04],
    ///0x20 - SBS debug control register
    pub dbgcr: DBGCR,
    ///0x24 - SBS debug lock register
    pub dbglockr: DBGLOCKR,
    _reserved5: [u8; 0x0c],
    ///0x34 - SBS RSS command register
    pub rsscmdr: RSSCMDR,
    _reserved6: [u8; 0x68],
    ///0xa0 - SBS EPOCH selection control register
    pub epochselcr: EPOCHSELCR,
    _reserved7: [u8; 0x1c],
    ///0xc0 - SBS security mode configuration control register
    pub seccfgr: SECCFGR,
    _reserved8: [u8; 0x3c],
    ///0x100 - SBS product mode and configuration register
    pub pmcr: PMCR,
    ///0x104 - SBS FPU interrupt mask register
    pub fpuimr: FPUIMR,
    ///0x108 - SBS memory erase status register
    pub mesr: MESR,
    _reserved11: [u8; 0x04],
    ///0x110 - SBS compensation cell for I/Os control and status register
    pub cccsr: CCCSR,
    ///0x114 - SBS compensation cell for I/Os value register
    pub ccvalr: CCVALR,
    ///0x118 - SBS compensation cell for I/Os software code register
    pub ccswcr: CCSWCR,
    _reserved14: [u8; 0x04],
    ///0x120 - SBS Class B register
    pub cfgr2: CFGR2,
    _reserved15: [u8; 0x20],
    ///0x144 - SBS CPU non-secure lock register
    pub cnslckr: CNSLCKR,
    ///0x148 - SBS CPU secure lock register
    pub cslckr: CSLCKR,
    ///0x14c - SBS flift ECC NMI mask register
    pub eccnmir: ECCNMIR,
}
///HDPLCR (rw) register accessor: an alias for `Reg<HDPLCR_SPEC>`
pub type HDPLCR = crate::Reg<hdplcr::HDPLCR_SPEC>;
///SBS temporal isolation control register
pub mod hdplcr;
///HDPLSR (r) register accessor: an alias for `Reg<HDPLSR_SPEC>`
pub type HDPLSR = crate::Reg<hdplsr::HDPLSR_SPEC>;
///SBS temporal isolation status register
pub mod hdplsr;
///NEXTHDPLCR (rw) register accessor: an alias for `Reg<NEXTHDPLCR_SPEC>`
pub type NEXTHDPLCR = crate::Reg<nexthdplcr::NEXTHDPLCR_SPEC>;
///SBS next HDPL control register
pub mod nexthdplcr;
///DBGCR (rw) register accessor: an alias for `Reg<DBGCR_SPEC>`
pub type DBGCR = crate::Reg<dbgcr::DBGCR_SPEC>;
///SBS debug control register
pub mod dbgcr;
///DBGLOCKR (rw) register accessor: an alias for `Reg<DBGLOCKR_SPEC>`
pub type DBGLOCKR = crate::Reg<dbglockr::DBGLOCKR_SPEC>;
///SBS debug lock register
pub mod dbglockr;
///RSSCMDR (rw) register accessor: an alias for `Reg<RSSCMDR_SPEC>`
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDR_SPEC>;
///SBS RSS command register
pub mod rsscmdr;
///EPOCHSELCR (rw) register accessor: an alias for `Reg<EPOCHSELCR_SPEC>`
pub type EPOCHSELCR = crate::Reg<epochselcr::EPOCHSELCR_SPEC>;
///SBS EPOCH selection control register
pub mod epochselcr;
///SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
///SBS security mode configuration control register
pub mod seccfgr;
///PMCR (rw) register accessor: an alias for `Reg<PMCR_SPEC>`
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
///SBS product mode and configuration register
pub mod pmcr;
///FPUIMR (rw) register accessor: an alias for `Reg<FPUIMR_SPEC>`
pub type FPUIMR = crate::Reg<fpuimr::FPUIMR_SPEC>;
///SBS FPU interrupt mask register
pub mod fpuimr;
///MESR (rw) register accessor: an alias for `Reg<MESR_SPEC>`
pub type MESR = crate::Reg<mesr::MESR_SPEC>;
///SBS memory erase status register
pub mod mesr;
///CCCSR (rw) register accessor: an alias for `Reg<CCCSR_SPEC>`
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
///SBS compensation cell for I/Os control and status register
pub mod cccsr;
///CCVALR (r) register accessor: an alias for `Reg<CCVALR_SPEC>`
pub type CCVALR = crate::Reg<ccvalr::CCVALR_SPEC>;
///SBS compensation cell for I/Os value register
pub mod ccvalr;
///CCSWCR (rw) register accessor: an alias for `Reg<CCSWCR_SPEC>`
pub type CCSWCR = crate::Reg<ccswcr::CCSWCR_SPEC>;
///SBS compensation cell for I/Os software code register
pub mod ccswcr;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///SBS Class B register
pub mod cfgr2;
///CNSLCKR (rw) register accessor: an alias for `Reg<CNSLCKR_SPEC>`
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKR_SPEC>;
///SBS CPU non-secure lock register
pub mod cnslckr;
///CSLCKR (rw) register accessor: an alias for `Reg<CSLCKR_SPEC>`
pub type CSLCKR = crate::Reg<cslckr::CSLCKR_SPEC>;
///SBS CPU secure lock register
pub mod cslckr;
///ECCNMIR (rw) register accessor: an alias for `Reg<ECCNMIR_SPEC>`
pub type ECCNMIR = crate::Reg<eccnmir::ECCNMIR_SPEC>;
///SBS flift ECC NMI mask register
pub mod eccnmir;
