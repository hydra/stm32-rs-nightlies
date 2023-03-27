///Register block
#[repr(C)]
pub struct FLT {
    ///0x00 -
    pub cr1: CR1,
    ///0x04 -
    pub cr2: CR2,
    ///0x08 -
    pub isr: ISR,
    ///0x0c -
    pub icr: ICR,
    ///0x10 -
    pub jchgr: JCHGR,
    ///0x14 -
    pub fcr: FCR,
    ///0x18 -
    pub jdatar: JDATAR,
    ///0x1c -
    pub rdatar: RDATAR,
    ///0x20 -
    pub awhtr: AWHTR,
    ///0x24 -
    pub awltr: AWLTR,
    ///0x28 -
    pub awsr: AWSR,
    ///0x2c -
    pub awcfr: AWCFR,
    ///0x30 -
    pub exmax: EXMAX,
    ///0x34 -
    pub exmin: EXMIN,
    ///0x38 -
    pub fltcnvtimr: FLTCNVTIMR,
    _reserved_end: [u8; 0x44],
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///
pub mod cr2;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///
pub mod isr;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///
pub mod icr;
///JCHGR (rw) register accessor: an alias for `Reg<JCHGR_SPEC>`
pub type JCHGR = crate::Reg<jchgr::JCHGR_SPEC>;
///
pub mod jchgr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///
pub mod fcr;
///JDATAR (r) register accessor: an alias for `Reg<JDATAR_SPEC>`
pub type JDATAR = crate::Reg<jdatar::JDATAR_SPEC>;
///
pub mod jdatar;
///RDATAR (r) register accessor: an alias for `Reg<RDATAR_SPEC>`
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
///
pub mod rdatar;
///AWHTR (rw) register accessor: an alias for `Reg<AWHTR_SPEC>`
pub type AWHTR = crate::Reg<awhtr::AWHTR_SPEC>;
///
pub mod awhtr;
///AWLTR (rw) register accessor: an alias for `Reg<AWLTR_SPEC>`
pub type AWLTR = crate::Reg<awltr::AWLTR_SPEC>;
///
pub mod awltr;
///AWSR (r) register accessor: an alias for `Reg<AWSR_SPEC>`
pub type AWSR = crate::Reg<awsr::AWSR_SPEC>;
///
pub mod awsr;
///AWCFR (rw) register accessor: an alias for `Reg<AWCFR_SPEC>`
pub type AWCFR = crate::Reg<awcfr::AWCFR_SPEC>;
///
pub mod awcfr;
///EXMAX (r) register accessor: an alias for `Reg<EXMAX_SPEC>`
pub type EXMAX = crate::Reg<exmax::EXMAX_SPEC>;
///
pub mod exmax;
///EXMIN (rw) register accessor: an alias for `Reg<EXMIN_SPEC>`
pub type EXMIN = crate::Reg<exmin::EXMIN_SPEC>;
///
pub mod exmin;
///FLTCNVTIMR (r) register accessor: an alias for `Reg<FLTCNVTIMR_SPEC>`
pub type FLTCNVTIMR = crate::Reg<fltcnvtimr::FLTCNVTIMR_SPEC>;
///
pub mod fltcnvtimr;
