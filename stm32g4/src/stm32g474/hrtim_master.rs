///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Master Timer Control Register
    pub mcr: MCR,
    ///0x04 - Master Timer Interrupt Status Register
    pub misr: MISR,
    ///0x08 - Master Timer Interrupt Clear Register
    pub micr: MICR,
    ///0x0c - HRTIM Master Timer DMA / Interrupt Enable Register
    pub mdier: MDIER,
    ///0x10 - Master Timer Counter Register
    pub mcntr: MCNTR,
    ///0x14 - Master Timer Period Register
    pub mper: MPER,
    ///0x18 - Master Timer Repetition Register
    pub mrep: MREP,
    ///0x1c - Master Timer Compare 1 Register
    pub mcmp1r: MCMP1R,
    _reserved8: [u8; 0x04],
    ///0x24 - Master Timer Compare 2 Register
    pub mcmp2r: MCMP2R,
    ///0x28 - Master Timer Compare 3 Register
    pub mcmp3r: MCMP3R,
    ///0x2c - Master Timer Compare 4 Register
    pub mcmp4r: MCMP4R,
}
///MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
///Master Timer Control Register
pub mod mcr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///Master Timer Interrupt Status Register
pub mod misr;
///MICR (w) register accessor: an alias for `Reg<MICR_SPEC>`
pub type MICR = crate::Reg<micr::MICR_SPEC>;
///Master Timer Interrupt Clear Register
pub mod micr;
///MDIER (rw) register accessor: an alias for `Reg<MDIER_SPEC>`
pub type MDIER = crate::Reg<mdier::MDIER_SPEC>;
///HRTIM Master Timer DMA / Interrupt Enable Register
pub mod mdier;
///MCNTR (rw) register accessor: an alias for `Reg<MCNTR_SPEC>`
pub type MCNTR = crate::Reg<mcntr::MCNTR_SPEC>;
///Master Timer Counter Register
pub mod mcntr;
///MPER (rw) register accessor: an alias for `Reg<MPER_SPEC>`
pub type MPER = crate::Reg<mper::MPER_SPEC>;
///Master Timer Period Register
pub mod mper;
///MREP (rw) register accessor: an alias for `Reg<MREP_SPEC>`
pub type MREP = crate::Reg<mrep::MREP_SPEC>;
///Master Timer Repetition Register
pub mod mrep;
///MCMP1R (rw) register accessor: an alias for `Reg<MCMP1R_SPEC>`
pub type MCMP1R = crate::Reg<mcmp1r::MCMP1R_SPEC>;
///Master Timer Compare 1 Register
pub mod mcmp1r;
///MCMP2R (rw) register accessor: an alias for `Reg<MCMP2R_SPEC>`
pub type MCMP2R = crate::Reg<mcmp2r::MCMP2R_SPEC>;
///Master Timer Compare 2 Register
pub mod mcmp2r;
///MCMP3R (rw) register accessor: an alias for `Reg<MCMP3R_SPEC>`
pub type MCMP3R = crate::Reg<mcmp3r::MCMP3R_SPEC>;
///Master Timer Compare 3 Register
pub mod mcmp3r;
///MCMP4R (rw) register accessor: an alias for `Reg<MCMP4R_SPEC>`
pub type MCMP4R = crate::Reg<mcmp4r::MCMP4R_SPEC>;
///Master Timer Compare 4 Register
pub mod mcmp4r;
