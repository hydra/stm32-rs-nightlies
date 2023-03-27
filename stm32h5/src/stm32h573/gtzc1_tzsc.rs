///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GTZC1 TZSC control register
    pub cr: CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - GTZC1 TZSC secure configuration register 1
    pub seccfgr1: SECCFGR1,
    ///0x14 - GTZC1 TZSC secure configuration register 2
    pub seccfgr2: SECCFGR2,
    ///0x18 - GTZC1 TZSC secure configuration register 3
    pub seccfgr3: SECCFGR3,
    _reserved4: [u8; 0x04],
    ///0x20 - GTZC1 TZSC privilege configuration register 1
    pub privcfgr1: PRIVCFGR1,
    ///0x24 - GTZC1 TZSC privilege configuration register 2
    pub privcfgr2: PRIVCFGR2,
    ///0x28 - GTZC1 TZSC privilege configuration register 3
    pub privcfgr3: PRIVCFGR3,
    _reserved7: [u8; 0x14],
    ///0x40 - GTZC1 TZSC memory 1 sub-region A watermark configuration register
    pub mpcwm1acfgr: MPCWM1ACFGR,
    ///0x44 - GTZC1 TZSC memory 1 sub-region A watermark register
    pub mpcwm1ar: MPCWM1AR,
    ///0x48 - GTZC1 TZSC memory 1 sub-region B watermark configuration register
    pub mpcwm1bcfgr: MPCWM1BCFGR,
    ///0x4c - GTZC1 TZSC memory 1 sub-region B watermark register
    pub mpcwm1br: MPCWM1BR,
    ///0x50 - GTZC1 TZSC memory 2 sub-region A watermark configuration register
    pub mpcwm2acfgr: MPCWM2ACFGR,
    ///0x54 - GTZC1 TZSC memory 2 sub-region A watermark register
    pub mpcwm2ar: MPCWM2AR,
    ///0x58 - GTZC1 TZSC memory 2 sub-region B watermark configuration register
    pub mpcwm2bcfgr: MPCWM2BCFGR,
    ///0x5c - GTZC1 TZSC memory 2 sub-region B watermark register
    pub mpcwm2br: MPCWM2BR,
    ///0x60 - GTZC1 TZSC memory 3 sub-region A watermark configuration register
    pub mpcwm3acfgr: MPCWM3ACFGR,
    ///0x64 - GTZC1 TZSC memory 3 sub-region A watermark register
    pub mpcwm3ar: MPCWM3AR,
    ///0x68 - GTZC1 TZSC memory 3 sub-region B watermark configuration register
    pub mpcwm3bcfgr: MPCWM3BCFGR,
    ///0x6c - GTZC1 TZSC memory 3 sub-region B watermark register
    pub mpcwm3br: MPCWM3BR,
    ///0x70 - GTZC1 TZSC memory 4 sub-region A watermark configuration register
    pub mpcwm4acfgr: MPCWM4ACFGR,
    ///0x74 - GTZC1 TZSC memory 4 sub-region A watermark register
    pub mpcwm4ar: MPCWM4AR,
    ///0x78 - GTZC1 TZSC memory 4 sub-region B watermark configuration register
    pub mpcwm4bcfgr: MPCWM4BCFGR,
    ///0x7c - GTZC1 TZSC memory 4 sub-region B watermark register
    pub mpcwm4br: MPCWM4BR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///GTZC1 TZSC control register
pub mod cr;
///SECCFGR1 (rw) register accessor: an alias for `Reg<SECCFGR1_SPEC>`
pub type SECCFGR1 = crate::Reg<seccfgr1::SECCFGR1_SPEC>;
///GTZC1 TZSC secure configuration register 1
pub mod seccfgr1;
///SECCFGR2 (rw) register accessor: an alias for `Reg<SECCFGR2_SPEC>`
pub type SECCFGR2 = crate::Reg<seccfgr2::SECCFGR2_SPEC>;
///GTZC1 TZSC secure configuration register 2
pub mod seccfgr2;
///SECCFGR3 (rw) register accessor: an alias for `Reg<SECCFGR3_SPEC>`
pub type SECCFGR3 = crate::Reg<seccfgr3::SECCFGR3_SPEC>;
///GTZC1 TZSC secure configuration register 3
pub mod seccfgr3;
///PRIVCFGR1 (rw) register accessor: an alias for `Reg<PRIVCFGR1_SPEC>`
pub type PRIVCFGR1 = crate::Reg<privcfgr1::PRIVCFGR1_SPEC>;
///GTZC1 TZSC privilege configuration register 1
pub mod privcfgr1;
///PRIVCFGR2 (rw) register accessor: an alias for `Reg<PRIVCFGR2_SPEC>`
pub type PRIVCFGR2 = crate::Reg<privcfgr2::PRIVCFGR2_SPEC>;
///GTZC1 TZSC privilege configuration register 2
pub mod privcfgr2;
///PRIVCFGR3 (rw) register accessor: an alias for `Reg<PRIVCFGR3_SPEC>`
pub type PRIVCFGR3 = crate::Reg<privcfgr3::PRIVCFGR3_SPEC>;
///GTZC1 TZSC privilege configuration register 3
pub mod privcfgr3;
///MPCWM1ACFGR (rw) register accessor: an alias for `Reg<MPCWM1ACFGR_SPEC>`
pub type MPCWM1ACFGR = crate::Reg<mpcwm1acfgr::MPCWM1ACFGR_SPEC>;
///GTZC1 TZSC memory 1 sub-region A watermark configuration register
pub mod mpcwm1acfgr;
///MPCWM1AR (rw) register accessor: an alias for `Reg<MPCWM1AR_SPEC>`
pub type MPCWM1AR = crate::Reg<mpcwm1ar::MPCWM1AR_SPEC>;
///GTZC1 TZSC memory 1 sub-region A watermark register
pub mod mpcwm1ar;
///MPCWM1BCFGR (rw) register accessor: an alias for `Reg<MPCWM1BCFGR_SPEC>`
pub type MPCWM1BCFGR = crate::Reg<mpcwm1bcfgr::MPCWM1BCFGR_SPEC>;
///GTZC1 TZSC memory 1 sub-region B watermark configuration register
pub mod mpcwm1bcfgr;
///MPCWM1BR (rw) register accessor: an alias for `Reg<MPCWM1BR_SPEC>`
pub type MPCWM1BR = crate::Reg<mpcwm1br::MPCWM1BR_SPEC>;
///GTZC1 TZSC memory 1 sub-region B watermark register
pub mod mpcwm1br;
///MPCWM2ACFGR (rw) register accessor: an alias for `Reg<MPCWM2ACFGR_SPEC>`
pub type MPCWM2ACFGR = crate::Reg<mpcwm2acfgr::MPCWM2ACFGR_SPEC>;
///GTZC1 TZSC memory 2 sub-region A watermark configuration register
pub mod mpcwm2acfgr;
///MPCWM2AR (rw) register accessor: an alias for `Reg<MPCWM2AR_SPEC>`
pub type MPCWM2AR = crate::Reg<mpcwm2ar::MPCWM2AR_SPEC>;
///GTZC1 TZSC memory 2 sub-region A watermark register
pub mod mpcwm2ar;
///MPCWM2BCFGR (rw) register accessor: an alias for `Reg<MPCWM2BCFGR_SPEC>`
pub type MPCWM2BCFGR = crate::Reg<mpcwm2bcfgr::MPCWM2BCFGR_SPEC>;
///GTZC1 TZSC memory 2 sub-region B watermark configuration register
pub mod mpcwm2bcfgr;
///MPCWM2BR (rw) register accessor: an alias for `Reg<MPCWM2BR_SPEC>`
pub type MPCWM2BR = crate::Reg<mpcwm2br::MPCWM2BR_SPEC>;
///GTZC1 TZSC memory 2 sub-region B watermark register
pub mod mpcwm2br;
///MPCWM3ACFGR (rw) register accessor: an alias for `Reg<MPCWM3ACFGR_SPEC>`
pub type MPCWM3ACFGR = crate::Reg<mpcwm3acfgr::MPCWM3ACFGR_SPEC>;
///GTZC1 TZSC memory 3 sub-region A watermark configuration register
pub mod mpcwm3acfgr;
///MPCWM3AR (rw) register accessor: an alias for `Reg<MPCWM3AR_SPEC>`
pub type MPCWM3AR = crate::Reg<mpcwm3ar::MPCWM3AR_SPEC>;
///GTZC1 TZSC memory 3 sub-region A watermark register
pub mod mpcwm3ar;
///MPCWM3BCFGR (rw) register accessor: an alias for `Reg<MPCWM3BCFGR_SPEC>`
pub type MPCWM3BCFGR = crate::Reg<mpcwm3bcfgr::MPCWM3BCFGR_SPEC>;
///GTZC1 TZSC memory 3 sub-region B watermark configuration register
pub mod mpcwm3bcfgr;
///MPCWM3BR (rw) register accessor: an alias for `Reg<MPCWM3BR_SPEC>`
pub type MPCWM3BR = crate::Reg<mpcwm3br::MPCWM3BR_SPEC>;
///GTZC1 TZSC memory 3 sub-region B watermark register
pub mod mpcwm3br;
///MPCWM4ACFGR (rw) register accessor: an alias for `Reg<MPCWM4ACFGR_SPEC>`
pub type MPCWM4ACFGR = crate::Reg<mpcwm4acfgr::MPCWM4ACFGR_SPEC>;
///GTZC1 TZSC memory 4 sub-region A watermark configuration register
pub mod mpcwm4acfgr;
///MPCWM4AR (rw) register accessor: an alias for `Reg<MPCWM4AR_SPEC>`
pub type MPCWM4AR = crate::Reg<mpcwm4ar::MPCWM4AR_SPEC>;
///GTZC1 TZSC memory 4 sub-region A watermark register
pub mod mpcwm4ar;
///MPCWM4BCFGR (rw) register accessor: an alias for `Reg<MPCWM4BCFGR_SPEC>`
pub type MPCWM4BCFGR = crate::Reg<mpcwm4bcfgr::MPCWM4BCFGR_SPEC>;
///GTZC1 TZSC memory 4 sub-region B watermark configuration register
pub mod mpcwm4bcfgr;
///MPCWM4BR (rw) register accessor: an alias for `Reg<MPCWM4BR_SPEC>`
pub type MPCWM4BR = crate::Reg<mpcwm4br::MPCWM4BR_SPEC>;
///GTZC1 TZSC memory 4 sub-region B watermark register
pub mod mpcwm4br;
