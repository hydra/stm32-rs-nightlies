///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MPCBB control register
    pub mpcbb4_cr: MPCBB4_CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - GTZC2 SRAM4 MPCBB configuration lock register
    pub mpcbb4_cfglock: MPCBB4_CFGLOCK,
    _reserved2: [u8; 0xec],
    ///0x100 - MPCBB security configuration for super-block 0 register
    pub mpcbb4_seccfgr0: MPCBB4_SECCFGR0,
    _reserved3: [u8; 0xfc],
    ///0x200 - MPCBB privileged configuration for super-block 0 register
    pub mpcbb4_privcfgr0: MPCBB4_PRIVCFGR0,
}
///MPCBB4_CR (rw) register accessor: an alias for `Reg<MPCBB4_CR_SPEC>`
pub type MPCBB4_CR = crate::Reg<mpcbb4_cr::MPCBB4_CR_SPEC>;
///MPCBB control register
pub mod mpcbb4_cr;
///MPCBB4_CFGLOCK (rw) register accessor: an alias for `Reg<MPCBB4_CFGLOCK_SPEC>`
pub type MPCBB4_CFGLOCK = crate::Reg<mpcbb4_cfglock::MPCBB4_CFGLOCK_SPEC>;
///GTZC2 SRAM4 MPCBB configuration lock register
pub mod mpcbb4_cfglock;
///MPCBB4_SECCFGR0 (rw) register accessor: an alias for `Reg<MPCBB4_SECCFGR0_SPEC>`
pub type MPCBB4_SECCFGR0 = crate::Reg<mpcbb4_seccfgr0::MPCBB4_SECCFGR0_SPEC>;
///MPCBB security configuration for super-block 0 register
pub mod mpcbb4_seccfgr0;
///MPCBB4_PRIVCFGR0 (rw) register accessor: an alias for `Reg<MPCBB4_PRIVCFGR0_SPEC>`
pub type MPCBB4_PRIVCFGR0 = crate::Reg<mpcbb4_privcfgr0::MPCBB4_PRIVCFGR0_SPEC>;
///MPCBB privileged configuration for super-block 0 register
pub mod mpcbb4_privcfgr0;
