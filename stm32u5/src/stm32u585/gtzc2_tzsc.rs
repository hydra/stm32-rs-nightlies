///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZSC control register
    pub tzsc_cr: TZSC_CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - TZSC secure configuration register 1
    pub tzsc_seccfgr1: TZSC_SECCFGR1,
    _reserved2: [u8; 0x0c],
    ///0x20 - TZSC privilege configuration register 1
    pub tzsc_privcfgr1: TZSC_PRIVCFGR1,
}
///TZSC_CR (rw) register accessor: an alias for `Reg<TZSC_CR_SPEC>`
pub type TZSC_CR = crate::Reg<tzsc_cr::TZSC_CR_SPEC>;
///TZSC control register
pub mod tzsc_cr;
///TZSC_SECCFGR1 (rw) register accessor: an alias for `Reg<TZSC_SECCFGR1_SPEC>`
pub type TZSC_SECCFGR1 = crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1_SPEC>;
///TZSC secure configuration register 1
pub mod tzsc_seccfgr1;
///TZSC_PRIVCFGR1 (rw) register accessor: an alias for `Reg<TZSC_PRIVCFGR1_SPEC>`
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>;
///TZSC privilege configuration register 1
pub mod tzsc_privcfgr1;
