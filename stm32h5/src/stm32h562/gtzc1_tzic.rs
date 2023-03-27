///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZIC interrupt enable register 1
    pub gtzc1_tzic_ier1: GTZC1_TZIC_IER1,
    ///0x04 - GTZC1 TZIC interrupt enable register 2
    pub gtzc1_tzic_ier2: GTZC1_TZIC_IER2,
    ///0x08 - GTZC1 TZIC interrupt enable register 3
    pub gtzc1_tzic_ier3: GTZC1_TZIC_IER3,
    ///0x0c - GTZC1 TZIC interrupt enable register 4
    pub gtzc1_tzic_ier4: GTZC1_TZIC_IER4,
    ///0x10 - TZIC status register 1
    pub gtzc1_tzic_sr1: GTZC1_TZIC_SR1,
    ///0x14 - TZIC status register 2
    pub gtzc1_tzic_sr2: GTZC1_TZIC_SR2,
    ///0x18 - TZIC status register 3
    pub gtzc1_tzic_sr3: GTZC1_TZIC_SR3,
    ///0x1c - GTZC1 TZIC status register 4
    pub gtzc1_tzic_sr4: GTZC1_TZIC_SR4,
    ///0x20 - TZIC flag clear register 1
    pub gtzc1_tzic_fcr1: GTZC1_TZIC_FCR1,
    ///0x24 - TZIC flag clear register 2
    pub gtzc1_tzic_fcr2: GTZC1_TZIC_FCR2,
    ///0x28 - TZIC flag clear register 3
    pub gtzc1_tzic_fcr3: GTZC1_TZIC_FCR3,
    ///0x2c - GTZC1 TZIC flag clear register 4
    pub gtzc1_tzic_fcr4: GTZC1_TZIC_FCR4,
}
///GTZC1_TZIC_IER1 (rw) register accessor: an alias for `Reg<GTZC1_TZIC_IER1_SPEC>`
pub type GTZC1_TZIC_IER1 = crate::Reg<gtzc1_tzic_ier1::GTZC1_TZIC_IER1_SPEC>;
///TZIC interrupt enable register 1
pub mod gtzc1_tzic_ier1;
///GTZC1_TZIC_IER2 (rw) register accessor: an alias for `Reg<GTZC1_TZIC_IER2_SPEC>`
pub type GTZC1_TZIC_IER2 = crate::Reg<gtzc1_tzic_ier2::GTZC1_TZIC_IER2_SPEC>;
///GTZC1 TZIC interrupt enable register 2
pub mod gtzc1_tzic_ier2;
///GTZC1_TZIC_IER3 (rw) register accessor: an alias for `Reg<GTZC1_TZIC_IER3_SPEC>`
pub type GTZC1_TZIC_IER3 = crate::Reg<gtzc1_tzic_ier3::GTZC1_TZIC_IER3_SPEC>;
///GTZC1 TZIC interrupt enable register 3
pub mod gtzc1_tzic_ier3;
///GTZC1_TZIC_IER4 (rw) register accessor: an alias for `Reg<GTZC1_TZIC_IER4_SPEC>`
pub type GTZC1_TZIC_IER4 = crate::Reg<gtzc1_tzic_ier4::GTZC1_TZIC_IER4_SPEC>;
///GTZC1 TZIC interrupt enable register 4
pub mod gtzc1_tzic_ier4;
///GTZC1_TZIC_SR1 (r) register accessor: an alias for `Reg<GTZC1_TZIC_SR1_SPEC>`
pub type GTZC1_TZIC_SR1 = crate::Reg<gtzc1_tzic_sr1::GTZC1_TZIC_SR1_SPEC>;
///TZIC status register 1
pub mod gtzc1_tzic_sr1;
///GTZC1_TZIC_SR2 (r) register accessor: an alias for `Reg<GTZC1_TZIC_SR2_SPEC>`
pub type GTZC1_TZIC_SR2 = crate::Reg<gtzc1_tzic_sr2::GTZC1_TZIC_SR2_SPEC>;
///TZIC status register 2
pub mod gtzc1_tzic_sr2;
///GTZC1_TZIC_SR3 (r) register accessor: an alias for `Reg<GTZC1_TZIC_SR3_SPEC>`
pub type GTZC1_TZIC_SR3 = crate::Reg<gtzc1_tzic_sr3::GTZC1_TZIC_SR3_SPEC>;
///TZIC status register 3
pub mod gtzc1_tzic_sr3;
///GTZC1_TZIC_SR4 (r) register accessor: an alias for `Reg<GTZC1_TZIC_SR4_SPEC>`
pub type GTZC1_TZIC_SR4 = crate::Reg<gtzc1_tzic_sr4::GTZC1_TZIC_SR4_SPEC>;
///GTZC1 TZIC status register 4
pub mod gtzc1_tzic_sr4;
///GTZC1_TZIC_FCR1 (w) register accessor: an alias for `Reg<GTZC1_TZIC_FCR1_SPEC>`
pub type GTZC1_TZIC_FCR1 = crate::Reg<gtzc1_tzic_fcr1::GTZC1_TZIC_FCR1_SPEC>;
///TZIC flag clear register 1
pub mod gtzc1_tzic_fcr1;
///GTZC1_TZIC_FCR2 (w) register accessor: an alias for `Reg<GTZC1_TZIC_FCR2_SPEC>`
pub type GTZC1_TZIC_FCR2 = crate::Reg<gtzc1_tzic_fcr2::GTZC1_TZIC_FCR2_SPEC>;
///TZIC flag clear register 2
pub mod gtzc1_tzic_fcr2;
///GTZC1_TZIC_FCR3 (w) register accessor: an alias for `Reg<GTZC1_TZIC_FCR3_SPEC>`
pub type GTZC1_TZIC_FCR3 = crate::Reg<gtzc1_tzic_fcr3::GTZC1_TZIC_FCR3_SPEC>;
///TZIC flag clear register 3
pub mod gtzc1_tzic_fcr3;
///GTZC1_TZIC_FCR4 (w) register accessor: an alias for `Reg<GTZC1_TZIC_FCR4_SPEC>`
pub type GTZC1_TZIC_FCR4 = crate::Reg<gtzc1_tzic_fcr4::GTZC1_TZIC_FCR4_SPEC>;
///GTZC1 TZIC flag clear register 4
pub mod gtzc1_tzic_fcr4;
