///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZSC control register
    pub tzsc_cr: TZSC_CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - TZSC secure configuration register 1
    pub tzsc_seccfgr1: TZSC_SECCFGR1,
    ///0x14 - TZSC secure configuration register 2
    pub tzsc_seccfgr2: TZSC_SECCFGR2,
    ///0x18 - TZSC secure configuration register 3
    pub tzsc_seccfgr3: TZSC_SECCFGR3,
    _reserved4: [u8; 0x04],
    ///0x20 - TZSC privilege configuration register 1
    pub tzsc_privcfgr1: TZSC_PRIVCFGR1,
    ///0x24 - TZSC privilege configuration register 2
    pub tzsc_privcfgr2: TZSC_PRIVCFGR2,
    ///0x28 - TZSC privilege configuration register 3
    pub tzsc_privcfgr3: TZSC_PRIVCFGR3,
    _reserved7: [u8; 0x14],
    ///0x40 - TZSC memory 1 sub-region A watermark configuration register
    pub tzsc_mpcwm1acfgr: TZSC_MPCWM1ACFGR,
    ///0x44 - TZSC memory 1 sub-region A watermark register
    pub tzsc_mpcwm1ar: TZSC_MPCWM1AR,
    ///0x48 - TZSC memory 1 sub-region B watermark configuration register
    pub tzsc_mpcwm1bcfgr: TZSC_MPCWM1BCFGR,
    ///0x4c - TZSC memory 1 sub-region B watermark register
    pub tzsc_mpcwm1br: TZSC_MPCWM1BR,
    ///0x50 - TZSC memory 2 sub-region A watermark configuration register
    pub tzsc_mpcwm2acfgr: TZSC_MPCWM2ACFGR,
    ///0x54 - TZSC memory 2 sub-region A watermark register
    pub tzsc_mpcwm2ar: TZSC_MPCWM2AR,
    ///0x58 - TZSC memory 2 sub-region B watermark configuration register
    pub tzsc_mpcwm2bcfgr: TZSC_MPCWM2BCFGR,
    ///0x5c - TZSC memory 2 sub-region B watermark register
    pub tzsc_mpcwm2br: TZSC_MPCWM2BR,
    ///0x60 - TZSC memory 3 sub-region A watermark configuration register
    pub tzsc_mpcwm3acfgr: TZSC_MPCWM3ACFGR,
    ///0x64 - TZSC memory 3 sub-region A watermark register
    pub tzsc_mpcwm3ar: TZSC_MPCWM3AR,
    _reserved17: [u8; 0x08],
    ///0x70 - TZSC memory 4 sub-region A watermark configuration register
    pub tzsc_mpcwm4acfgr: TZSC_MPCWM4ACFGR,
    ///0x74 - TZSC memory 4 sub-region A watermark register
    pub tzsc_mpcwm4ar: TZSC_MPCWM4AR,
    _reserved19: [u8; 0x08],
    ///0x80 - TZSC memory 5 sub-region A watermark configuration register
    pub tzsc_mpcwm5acfgr: TZSC_MPCWM5ACFGR,
    ///0x84 - TZSC memory 5 sub-region A watermark register
    pub tzsc_mpcwm5ar: TZSC_MPCWM5AR,
    ///0x88 - TZSC memory 5 sub-region B watermark configuration register
    pub tzsc_mpcwm5bcfgr: TZSC_MPCWM5BCFGR,
    ///0x8c - TZSC memory 5 sub-region B watermark register
    pub tzsc_mpcwm5br: TZSC_MPCWM5BR,
}
///TZSC_CR (rw) register accessor: an alias for `Reg<TZSC_CR_SPEC>`
pub type TZSC_CR = crate::Reg<tzsc_cr::TZSC_CR_SPEC>;
///TZSC control register
pub mod tzsc_cr;
///TZSC_SECCFGR1 (rw) register accessor: an alias for `Reg<TZSC_SECCFGR1_SPEC>`
pub type TZSC_SECCFGR1 = crate::Reg<tzsc_seccfgr1::TZSC_SECCFGR1_SPEC>;
///TZSC secure configuration register 1
pub mod tzsc_seccfgr1;
///TZSC_SECCFGR2 (rw) register accessor: an alias for `Reg<TZSC_SECCFGR2_SPEC>`
pub type TZSC_SECCFGR2 = crate::Reg<tzsc_seccfgr2::TZSC_SECCFGR2_SPEC>;
///TZSC secure configuration register 2
pub mod tzsc_seccfgr2;
///TZSC_SECCFGR3 (rw) register accessor: an alias for `Reg<TZSC_SECCFGR3_SPEC>`
pub type TZSC_SECCFGR3 = crate::Reg<tzsc_seccfgr3::TZSC_SECCFGR3_SPEC>;
///TZSC secure configuration register 3
pub mod tzsc_seccfgr3;
///TZSC_PRIVCFGR1 (rw) register accessor: an alias for `Reg<TZSC_PRIVCFGR1_SPEC>`
pub type TZSC_PRIVCFGR1 = crate::Reg<tzsc_privcfgr1::TZSC_PRIVCFGR1_SPEC>;
///TZSC privilege configuration register 1
pub mod tzsc_privcfgr1;
///TZSC_PRIVCFGR2 (rw) register accessor: an alias for `Reg<TZSC_PRIVCFGR2_SPEC>`
pub type TZSC_PRIVCFGR2 = crate::Reg<tzsc_privcfgr2::TZSC_PRIVCFGR2_SPEC>;
///TZSC privilege configuration register 2
pub mod tzsc_privcfgr2;
///TZSC_PRIVCFGR3 (rw) register accessor: an alias for `Reg<TZSC_PRIVCFGR3_SPEC>`
pub type TZSC_PRIVCFGR3 = crate::Reg<tzsc_privcfgr3::TZSC_PRIVCFGR3_SPEC>;
///TZSC privilege configuration register 3
pub mod tzsc_privcfgr3;
///TZSC_MPCWM1ACFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM1ACFGR_SPEC>`
pub type TZSC_MPCWM1ACFGR = crate::Reg<tzsc_mpcwm1acfgr::TZSC_MPCWM1ACFGR_SPEC>;
///TZSC memory 1 sub-region A watermark configuration register
pub mod tzsc_mpcwm1acfgr;
///TZSC_MPCWM1AR (rw) register accessor: an alias for `Reg<TZSC_MPCWM1AR_SPEC>`
pub type TZSC_MPCWM1AR = crate::Reg<tzsc_mpcwm1ar::TZSC_MPCWM1AR_SPEC>;
///TZSC memory 1 sub-region A watermark register
pub mod tzsc_mpcwm1ar;
///TZSC_MPCWM1BCFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM1BCFGR_SPEC>`
pub type TZSC_MPCWM1BCFGR = crate::Reg<tzsc_mpcwm1bcfgr::TZSC_MPCWM1BCFGR_SPEC>;
///TZSC memory 1 sub-region B watermark configuration register
pub mod tzsc_mpcwm1bcfgr;
///TZSC_MPCWM1BR (rw) register accessor: an alias for `Reg<TZSC_MPCWM1BR_SPEC>`
pub type TZSC_MPCWM1BR = crate::Reg<tzsc_mpcwm1br::TZSC_MPCWM1BR_SPEC>;
///TZSC memory 1 sub-region B watermark register
pub mod tzsc_mpcwm1br;
///TZSC_MPCWM2ACFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM2ACFGR_SPEC>`
pub type TZSC_MPCWM2ACFGR = crate::Reg<tzsc_mpcwm2acfgr::TZSC_MPCWM2ACFGR_SPEC>;
///TZSC memory 2 sub-region A watermark configuration register
pub mod tzsc_mpcwm2acfgr;
///TZSC_MPCWM2AR (rw) register accessor: an alias for `Reg<TZSC_MPCWM2AR_SPEC>`
pub type TZSC_MPCWM2AR = crate::Reg<tzsc_mpcwm2ar::TZSC_MPCWM2AR_SPEC>;
///TZSC memory 2 sub-region A watermark register
pub mod tzsc_mpcwm2ar;
///TZSC_MPCWM2BCFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM2BCFGR_SPEC>`
pub type TZSC_MPCWM2BCFGR = crate::Reg<tzsc_mpcwm2bcfgr::TZSC_MPCWM2BCFGR_SPEC>;
///TZSC memory 2 sub-region B watermark configuration register
pub mod tzsc_mpcwm2bcfgr;
///TZSC_MPCWM2BR (rw) register accessor: an alias for `Reg<TZSC_MPCWM2BR_SPEC>`
pub type TZSC_MPCWM2BR = crate::Reg<tzsc_mpcwm2br::TZSC_MPCWM2BR_SPEC>;
///TZSC memory 2 sub-region B watermark register
pub mod tzsc_mpcwm2br;
///TZSC_MPCWM3ACFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM3ACFGR_SPEC>`
pub type TZSC_MPCWM3ACFGR = crate::Reg<tzsc_mpcwm3acfgr::TZSC_MPCWM3ACFGR_SPEC>;
///TZSC memory 3 sub-region A watermark configuration register
pub mod tzsc_mpcwm3acfgr;
///TZSC_MPCWM3AR (rw) register accessor: an alias for `Reg<TZSC_MPCWM3AR_SPEC>`
pub type TZSC_MPCWM3AR = crate::Reg<tzsc_mpcwm3ar::TZSC_MPCWM3AR_SPEC>;
///TZSC memory 3 sub-region A watermark register
pub mod tzsc_mpcwm3ar;
///TZSC_MPCWM4ACFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM4ACFGR_SPEC>`
pub type TZSC_MPCWM4ACFGR = crate::Reg<tzsc_mpcwm4acfgr::TZSC_MPCWM4ACFGR_SPEC>;
///TZSC memory 4 sub-region A watermark configuration register
pub mod tzsc_mpcwm4acfgr;
///TZSC_MPCWM4AR (rw) register accessor: an alias for `Reg<TZSC_MPCWM4AR_SPEC>`
pub type TZSC_MPCWM4AR = crate::Reg<tzsc_mpcwm4ar::TZSC_MPCWM4AR_SPEC>;
///TZSC memory 4 sub-region A watermark register
pub mod tzsc_mpcwm4ar;
///TZSC_MPCWM5ACFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM5ACFGR_SPEC>`
pub type TZSC_MPCWM5ACFGR = crate::Reg<tzsc_mpcwm5acfgr::TZSC_MPCWM5ACFGR_SPEC>;
///TZSC memory 5 sub-region A watermark configuration register
pub mod tzsc_mpcwm5acfgr;
///TZSC_MPCWM5AR (rw) register accessor: an alias for `Reg<TZSC_MPCWM5AR_SPEC>`
pub type TZSC_MPCWM5AR = crate::Reg<tzsc_mpcwm5ar::TZSC_MPCWM5AR_SPEC>;
///TZSC memory 5 sub-region A watermark register
pub mod tzsc_mpcwm5ar;
///TZSC_MPCWM5BCFGR (rw) register accessor: an alias for `Reg<TZSC_MPCWM5BCFGR_SPEC>`
pub type TZSC_MPCWM5BCFGR = crate::Reg<tzsc_mpcwm5bcfgr::TZSC_MPCWM5BCFGR_SPEC>;
///TZSC memory 5 sub-region B watermark configuration register
pub mod tzsc_mpcwm5bcfgr;
///TZSC_MPCWM5BR (rw) register accessor: an alias for `Reg<TZSC_MPCWM5BR_SPEC>`
pub type TZSC_MPCWM5BR = crate::Reg<tzsc_mpcwm5br::TZSC_MPCWM5BR_SPEC>;
///TZSC memory 5 sub-region B watermark register
pub mod tzsc_mpcwm5br;
