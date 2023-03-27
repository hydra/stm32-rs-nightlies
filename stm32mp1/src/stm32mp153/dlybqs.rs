///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DLYB control register
    pub dlyb_cr: DLYB_CR,
    ///0x04 - DLYB configuration register
    pub dlyb_cfgr: DLYB_CFGR,
    _reserved2: [u8; 0x03ec],
    ///0x3f4 - DLYB IP version register
    pub dlyb_verr: DLYB_VERR,
    ///0x3f8 - DLYB IP identification register
    pub dlyb_ipidr: DLYB_IPIDR,
    ///0x3fc - DLYB size ID register
    pub dlyb_sidr: DLYB_SIDR,
}
///DLYB_CR (rw) register accessor: an alias for `Reg<DLYB_CR_SPEC>`
pub type DLYB_CR = crate::Reg<dlyb_cr::DLYB_CR_SPEC>;
///DLYB control register
pub mod dlyb_cr;
///DLYB_CFGR (rw) register accessor: an alias for `Reg<DLYB_CFGR_SPEC>`
pub type DLYB_CFGR = crate::Reg<dlyb_cfgr::DLYB_CFGR_SPEC>;
///DLYB configuration register
pub mod dlyb_cfgr;
///DLYB_VERR (r) register accessor: an alias for `Reg<DLYB_VERR_SPEC>`
pub type DLYB_VERR = crate::Reg<dlyb_verr::DLYB_VERR_SPEC>;
///DLYB IP version register
pub mod dlyb_verr;
///DLYB_IPIDR (r) register accessor: an alias for `Reg<DLYB_IPIDR_SPEC>`
pub type DLYB_IPIDR = crate::Reg<dlyb_ipidr::DLYB_IPIDR_SPEC>;
///DLYB IP identification register
pub mod dlyb_ipidr;
///DLYB_SIDR (r) register accessor: an alias for `Reg<DLYB_SIDR_SPEC>`
pub type DLYB_SIDR = crate::Reg<dlyb_sidr::DLYB_SIDR_SPEC>;
///DLYB size ID register
pub mod dlyb_sidr;
