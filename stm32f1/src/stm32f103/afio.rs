///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Event Control Register (AFIO_EVCR)
    pub evcr: EVCR,
    ///0x04 - AF remap and debug I/O configuration register (AFIO_MAPR)
    pub mapr: MAPR,
    ///0x08 - External interrupt configuration register 1 (AFIO_EXTICR1)
    pub exticr1: EXTICR1,
    ///0x0c - External interrupt configuration register 2 (AFIO_EXTICR2)
    pub exticr2: EXTICR2,
    ///0x10 - External interrupt configuration register 3 (AFIO_EXTICR3)
    pub exticr3: EXTICR3,
    ///0x14 - External interrupt configuration register 4 (AFIO_EXTICR4)
    pub exticr4: EXTICR4,
    _reserved6: [u8; 0x04],
    ///0x1c - AF remap and debug I/O configuration register
    pub mapr2: MAPR2,
}
///EVCR (rw) register accessor: an alias for `Reg<EVCR_SPEC>`
pub type EVCR = crate::Reg<evcr::EVCR_SPEC>;
///Event Control Register (AFIO_EVCR)
pub mod evcr;
///MAPR (rw) register accessor: an alias for `Reg<MAPR_SPEC>`
pub type MAPR = crate::Reg<mapr::MAPR_SPEC>;
///AF remap and debug I/O configuration register (AFIO_MAPR)
pub mod mapr;
///EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///External interrupt configuration register 1 (AFIO_EXTICR1)
pub mod exticr1;
///EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///External interrupt configuration register 2 (AFIO_EXTICR2)
pub mod exticr2;
///EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///External interrupt configuration register 3 (AFIO_EXTICR3)
pub mod exticr3;
///EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///External interrupt configuration register 4 (AFIO_EXTICR4)
pub mod exticr4;
///MAPR2 (rw) register accessor: an alias for `Reg<MAPR2_SPEC>`
pub type MAPR2 = crate::Reg<mapr2::MAPR2_SPEC>;
///AF remap and debug I/O configuration register
pub mod mapr2;
