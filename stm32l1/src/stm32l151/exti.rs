///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - IMR
    pub imr: IMR,
    ///0x04 - EMR
    pub emr: EMR,
    ///0x08 - RTSR
    pub rtsr: RTSR,
    ///0x0c - FTSR
    pub ftsr: FTSR,
    ///0x10 - SWIER
    pub swier: SWIER,
    ///0x14 - PR
    pub pr: PR,
}
///IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`
pub type IMR = crate::Reg<imr::IMR_SPEC>;
///IMR
pub mod imr;
///EMR (rw) register accessor: an alias for `Reg<EMR_SPEC>`
pub type EMR = crate::Reg<emr::EMR_SPEC>;
///EMR
pub mod emr;
///RTSR (rw) register accessor: an alias for `Reg<RTSR_SPEC>`
pub type RTSR = crate::Reg<rtsr::RTSR_SPEC>;
///RTSR
pub mod rtsr;
///FTSR (rw) register accessor: an alias for `Reg<FTSR_SPEC>`
pub type FTSR = crate::Reg<ftsr::FTSR_SPEC>;
///FTSR
pub mod ftsr;
///SWIER (rw) register accessor: an alias for `Reg<SWIER_SPEC>`
pub type SWIER = crate::Reg<swier::SWIER_SPEC>;
///SWIER
pub mod swier;
///PR (rw) register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///PR
pub mod pr;
