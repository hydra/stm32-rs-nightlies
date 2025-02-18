///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZSC control register
    pub cr: CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - TZSC secure configuration register 1
    pub seccfgr1: SECCFGR1,
    ///0x14 - TZSC secure configuration register 2
    pub seccfgr2: SECCFGR2,
    _reserved3: [u8; 0x08],
    ///0x20 - TZSC privilege configuration register 1
    pub privcfgr1: PRIVCFGR1,
    ///0x24 - TZSC privilege configuration register 2
    pub privcfgr2: PRIVCFGR2,
    _reserved5: [u8; 0x08],
    ///0x30 - TZSC external memory non-secure watermark register 1
    pub mpcwm1_nswmr1: MPCWM1_NSWMR1,
    ///0x34 - TZSC external memory non-secure watermark register 1
    pub mpcwm1_nswmr2: MPCWM1_NSWMR2,
    ///0x38 - TZSC external memory non-secure watermark register 1
    pub mpcwm2_nswmr1: MPCWM2_NSWMR1,
    ///0x3c - TZSC external memory non-secure watermark register 2
    pub mpcwm2_nswmr2: MPCWM2_NSWMR2,
    ///0x40 - TZSC external memory non-secure watermark register 2
    pub mpcwm3_nswmr1: MPCWM3_NSWMR1,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///TZSC control register
pub mod cr;
///SECCFGR1 (rw) register accessor: an alias for `Reg<SECCFGR1_SPEC>`
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1_SPEC>;
///TZSC secure configuration register 1
pub mod seccfgr1;
///SECCFGR2 (rw) register accessor: an alias for `Reg<SECCFGR2_SPEC>`
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2_SPEC>;
///TZSC secure configuration register 2
pub mod seccfgr2;
///PRIVCFGR1 (rw) register accessor: an alias for `Reg<PRIVCFGR1_SPEC>`
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1_SPEC>;
///TZSC privilege configuration register 1
pub mod privcfgr1;
///PRIVCFGR2 (rw) register accessor: an alias for `Reg<PRIVCFGR2_SPEC>`
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2_SPEC>;
///TZSC privilege configuration register 2
pub mod privcfgr2;
///MPCWM1_NSWMR1 (rw) register accessor: an alias for `Reg<MPCWM1_NSWMR1_SPEC>`
pub type MPCWM1_NSWMR1 = crate::Reg<mpcwm1_nswmr1::MPCWM1_NSWMR1_SPEC>;
///TZSC external memory non-secure watermark register 1
pub mod mpcwm1_nswmr1;
///MPCWM1_NSWMR2 (rw) register accessor: an alias for `Reg<MPCWM1_NSWMR2_SPEC>`
pub type MPCWM1_NSWMR2 = crate::Reg<mpcwm1_nswmr2::MPCWM1_NSWMR2_SPEC>;
///TZSC external memory non-secure watermark register 1
pub mod mpcwm1_nswmr2;
///MPCWM2_NSWMR1 (rw) register accessor: an alias for `Reg<MPCWM2_NSWMR1_SPEC>`
pub type MPCWM2_NSWMR1 = crate::Reg<mpcwm2_nswmr1::MPCWM2_NSWMR1_SPEC>;
///TZSC external memory non-secure watermark register 1
pub mod mpcwm2_nswmr1;
///MPCWM3_NSWMR1 (rw) register accessor: an alias for `Reg<MPCWM3_NSWMR1_SPEC>`
pub type MPCWM3_NSWMR1 = crate::Reg<mpcwm3_nswmr1::MPCWM3_NSWMR1_SPEC>;
///TZSC external memory non-secure watermark register 2
pub mod mpcwm3_nswmr1;
///MPCWM2_NSWMR2 (rw) register accessor: an alias for `Reg<MPCWM2_NSWMR2_SPEC>`
pub type MPCWM2_NSWMR2 = crate::Reg<mpcwm2_nswmr2::MPCWM2_NSWMR2_SPEC>;
///TZSC external memory non-secure watermark register 2
pub mod mpcwm2_nswmr2;
