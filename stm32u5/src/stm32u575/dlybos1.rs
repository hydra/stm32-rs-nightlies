///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub dlyb_cr: DLYB_CR,
    ///0x04 - configuration register
    pub dlyb_cfgr: DLYB_CFGR,
}
///DLYB_CR (rw) register accessor: an alias for `Reg<DLYB_CR_SPEC>`
pub type DLYB_CR = crate::Reg<dlyb_cr::DLYB_CR_SPEC>;
///control register
pub mod dlyb_cr;
///DLYB_CFGR (rw) register accessor: an alias for `Reg<DLYB_CFGR_SPEC>`
pub type DLYB_CFGR = crate::Reg<dlyb_cfgr::DLYB_CFGR_SPEC>;
///configuration register
pub mod dlyb_cfgr;
