///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ///0x04 - peripheral mode configuration register
    pub pmcr: PMCR,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
    ///0x18 - SYSCFG timer break lockup register
    pub cfgr: CFGR,
    _reserved6: [u8; 0x04],
    ///0x20 - compensation cell control/status register
    pub cccsr: CCCSR,
    ///0x24 - SYSCFG compensation cell value register
    pub ccvr: CCVR,
    ///0x28 - SYSCFG compensation cell code register
    pub cccr: CCCR,
}
///PMCR (rw) register accessor: an alias for `Reg<PMCR_SPEC>`
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
///peripheral mode configuration register
pub mod pmcr;
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
///CCCSR (rw) register accessor: an alias for `Reg<CCCSR_SPEC>`
pub type CCCSR = crate::Reg<cccsr::CCCSR_SPEC>;
///compensation cell control/status register
pub mod cccsr;
///CCVR (r) register accessor: an alias for `Reg<CCVR_SPEC>`
pub type CCVR = crate::Reg<ccvr::CCVR_SPEC>;
///SYSCFG compensation cell value register
pub mod ccvr;
///CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///SYSCFG compensation cell code register
pub mod cccr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///SYSCFG timer break lockup register
pub mod cfgr;
