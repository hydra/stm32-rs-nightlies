///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )
    pub syscfg_bootr: SYSCFG_BOOTR,
    ///0x04 - SYSCFG peripheral mode configuration set register
    pub syscfg_pmcsetr: SYSCFG_PMCSETR,
    _reserved2: [u8; 0x10],
    ///0x18 - SYSCFG IO control register
    pub syscfg_ioctrlsetr: SYSCFG_IOCTRLSETR,
    ///0x1c - SYSCFG interconnect control register
    pub syscfg_icnr: SYSCFG_ICNR,
    ///0x20 - SYSCFG compensation cell control register
    pub syscfg_cmpcr: SYSCFG_CMPCR,
    ///0x24 - SYSCFG compensation cell enable set register
    pub syscfg_cmpensetr: SYSCFG_CMPENSETR,
    ///0x28 - SYSCFG compensation cell enable set register
    pub syscfg_cmpenclrr: SYSCFG_CMPENCLRR,
    ///0x2c - SYSCFG control timer break register
    pub syscfg_cbr: SYSCFG_CBR,
    _reserved8: [u8; 0x14],
    ///0x44 - SYSCFG peripheral mode configuration clear register
    pub syscfg_pmcclrr: SYSCFG_PMCCLRR,
    _reserved9: [u8; 0x10],
    ///0x58 - SYSCFG IO control register
    pub syscfg_ioctrlclrr: SYSCFG_IOCTRLCLRR,
    _reserved10: [u8; 0x0398],
    ///0x3f4 - SYSCFG version register
    pub syscfg_verr: SYSCFG_VERR,
    ///0x3f8 - SYSCFG identification register
    pub syscfg_ipidr: SYSCFG_IPIDR,
    ///0x3fc - SYSCFG size identification register
    pub syscfg_sidr: SYSCFG_SIDR,
}
///SYSCFG_BOOTR (rw) register accessor: an alias for `Reg<SYSCFG_BOOTR_SPEC>`
pub type SYSCFG_BOOTR = crate::Reg<syscfg_bootr::SYSCFG_BOOTR_SPEC>;
///This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )
pub mod syscfg_bootr;
///SYSCFG_PMCSETR (rw) register accessor: an alias for `Reg<SYSCFG_PMCSETR_SPEC>`
pub type SYSCFG_PMCSETR = crate::Reg<syscfg_pmcsetr::SYSCFG_PMCSETR_SPEC>;
///SYSCFG peripheral mode configuration set register
pub mod syscfg_pmcsetr;
///SYSCFG_IOCTRLSETR (rw) register accessor: an alias for `Reg<SYSCFG_IOCTRLSETR_SPEC>`
pub type SYSCFG_IOCTRLSETR = crate::Reg<syscfg_ioctrlsetr::SYSCFG_IOCTRLSETR_SPEC>;
///SYSCFG IO control register
pub mod syscfg_ioctrlsetr;
///SYSCFG_ICNR (rw) register accessor: an alias for `Reg<SYSCFG_ICNR_SPEC>`
pub type SYSCFG_ICNR = crate::Reg<syscfg_icnr::SYSCFG_ICNR_SPEC>;
///SYSCFG interconnect control register
pub mod syscfg_icnr;
///SYSCFG_CMPCR (rw) register accessor: an alias for `Reg<SYSCFG_CMPCR_SPEC>`
pub type SYSCFG_CMPCR = crate::Reg<syscfg_cmpcr::SYSCFG_CMPCR_SPEC>;
///SYSCFG compensation cell control register
pub mod syscfg_cmpcr;
///SYSCFG_CMPENSETR (rw) register accessor: an alias for `Reg<SYSCFG_CMPENSETR_SPEC>`
pub type SYSCFG_CMPENSETR = crate::Reg<syscfg_cmpensetr::SYSCFG_CMPENSETR_SPEC>;
///SYSCFG compensation cell enable set register
pub mod syscfg_cmpensetr;
///SYSCFG_CMPENCLRR (rw) register accessor: an alias for `Reg<SYSCFG_CMPENCLRR_SPEC>`
pub type SYSCFG_CMPENCLRR = crate::Reg<syscfg_cmpenclrr::SYSCFG_CMPENCLRR_SPEC>;
///SYSCFG compensation cell enable set register
pub mod syscfg_cmpenclrr;
///SYSCFG_CBR (rw) register accessor: an alias for `Reg<SYSCFG_CBR_SPEC>`
pub type SYSCFG_CBR = crate::Reg<syscfg_cbr::SYSCFG_CBR_SPEC>;
///SYSCFG control timer break register
pub mod syscfg_cbr;
///SYSCFG_PMCCLRR (rw) register accessor: an alias for `Reg<SYSCFG_PMCCLRR_SPEC>`
pub type SYSCFG_PMCCLRR = crate::Reg<syscfg_pmcclrr::SYSCFG_PMCCLRR_SPEC>;
///SYSCFG peripheral mode configuration clear register
pub mod syscfg_pmcclrr;
///SYSCFG_IOCTRLCLRR (rw) register accessor: an alias for `Reg<SYSCFG_IOCTRLCLRR_SPEC>`
pub type SYSCFG_IOCTRLCLRR = crate::Reg<syscfg_ioctrlclrr::SYSCFG_IOCTRLCLRR_SPEC>;
///SYSCFG IO control register
pub mod syscfg_ioctrlclrr;
///SYSCFG_VERR (r) register accessor: an alias for `Reg<SYSCFG_VERR_SPEC>`
pub type SYSCFG_VERR = crate::Reg<syscfg_verr::SYSCFG_VERR_SPEC>;
///SYSCFG version register
pub mod syscfg_verr;
///SYSCFG_IPIDR (r) register accessor: an alias for `Reg<SYSCFG_IPIDR_SPEC>`
pub type SYSCFG_IPIDR = crate::Reg<syscfg_ipidr::SYSCFG_IPIDR_SPEC>;
///SYSCFG identification register
pub mod syscfg_ipidr;
///SYSCFG_SIDR (r) register accessor: an alias for `Reg<SYSCFG_SIDR_SPEC>`
pub type SYSCFG_SIDR = crate::Reg<syscfg_sidr::SYSCFG_SIDR_SPEC>;
///SYSCFG size identification register
pub mod syscfg_sidr;
