///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SYSCFG configuration register 1
    pub cfgr1: CFGR1,
    _reserved1: [u8; 0x14],
    ///0x18 - SYSCFG configuration register 1
    pub cfgr2: CFGR2,
}
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///SYSCFG configuration register 1
pub mod cfgr1;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///SYSCFG configuration register 1
pub mod cfgr2;
