///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt mask register
    pub imr1: IMR1,
    ///0x04 - Event mask register
    pub emr1: EMR1,
    ///0x08 - Rising Trigger selection register
    pub rtsr1: RTSR1,
    ///0x0c - Falling Trigger selection register
    pub ftsr1: FTSR1,
    ///0x10 - Software interrupt event register
    pub swier1: SWIER1,
    ///0x14 - Pending register
    pub pr1: PR1,
    _reserved6: [u8; 0x08],
    ///0x20 - Interrupt mask register
    pub imr2: IMR2,
    ///0x24 - Event mask register
    pub emr2: EMR2,
    ///0x28 - Rising Trigger selection register
    pub rtsr2: RTSR2,
    ///0x2c - Falling Trigger selection register
    pub ftsr2: FTSR2,
    ///0x30 - Software interrupt event register
    pub swier2: SWIER2,
    ///0x34 - Pending register
    pub pr2: PR2,
}
///IMR1 (rw) register accessor: an alias for `Reg<IMR1_SPEC>`
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
///Interrupt mask register
pub mod imr1;
///EMR1 (rw) register accessor: an alias for `Reg<EMR1_SPEC>`
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
///Event mask register
pub mod emr1;
///RTSR1 (rw) register accessor: an alias for `Reg<RTSR1_SPEC>`
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
///Rising Trigger selection register
pub mod rtsr1;
///FTSR1 (rw) register accessor: an alias for `Reg<FTSR1_SPEC>`
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
///Falling Trigger selection register
pub mod ftsr1;
///SWIER1 (rw) register accessor: an alias for `Reg<SWIER1_SPEC>`
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
///Software interrupt event register
pub mod swier1;
///PR1 (rw) register accessor: an alias for `Reg<PR1_SPEC>`
pub type PR1 = crate::Reg<pr1::PR1_SPEC>;
///Pending register
pub mod pr1;
///IMR2 (rw) register accessor: an alias for `Reg<IMR2_SPEC>`
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
///Interrupt mask register
pub mod imr2;
///EMR2 (rw) register accessor: an alias for `Reg<EMR2_SPEC>`
pub type EMR2 = crate::Reg<emr2::EMR2_SPEC>;
///Event mask register
pub mod emr2;
///RTSR2 (rw) register accessor: an alias for `Reg<RTSR2_SPEC>`
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
///Rising Trigger selection register
pub mod rtsr2;
///FTSR2 (rw) register accessor: an alias for `Reg<FTSR2_SPEC>`
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
///Falling Trigger selection register
pub mod ftsr2;
///SWIER2 (rw) register accessor: an alias for `Reg<SWIER2_SPEC>`
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
///Software interrupt event register
pub mod swier2;
///PR2 (rw) register accessor: an alias for `Reg<PR2_SPEC>`
pub type PR2 = crate::Reg<pr2::PR2_SPEC>;
///Pending register
pub mod pr2;
