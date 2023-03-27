///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - memory remap register
    pub memrm: MEMRM,
    ///0x04 - peripheral mode configuration register
    pub pmc: PMC,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
    _reserved6: [u8; 0x08],
    ///0x20 - Compensation cell control register
    pub cmpcr: CMPCR,
    _reserved7: [u8; 0x08],
    ///0x2c - Configuration register
    pub cfgr: CFGR,
}
///MEMRM (rw) register accessor: an alias for `Reg<MEMRM_SPEC>`
pub type MEMRM = crate::Reg<memrm::MEMRM_SPEC>;
///memory remap register
pub mod memrm;
///PMC (rw) register accessor: an alias for `Reg<PMC_SPEC>`
pub type PMC = crate::Reg<pmc::PMC_SPEC>;
///peripheral mode configuration register
pub mod pmc;
///EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///external interrupt configuration register 1
pub mod exticr1;
///EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///external interrupt configuration register 2
pub mod exticr2;
///EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///external interrupt configuration register 3
pub mod exticr3;
///EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///external interrupt configuration register 4
pub mod exticr4;
///CMPCR (r) register accessor: an alias for `Reg<CMPCR_SPEC>`
pub type CMPCR = crate::Reg<cmpcr::CMPCR_SPEC>;
///Compensation cell control register
pub mod cmpcr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///Configuration register
pub mod cfgr;
