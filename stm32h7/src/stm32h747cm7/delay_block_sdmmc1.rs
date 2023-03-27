///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DLYB control register
    pub cr: CR,
    ///0x04 - DLYB configuration register
    pub cfgr: CFGR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DLYB control register
pub mod cr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///DLYB configuration register
pub mod cfgr;
