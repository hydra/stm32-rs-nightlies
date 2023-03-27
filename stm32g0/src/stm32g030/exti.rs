///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - EXTI rising trigger selection register
    pub rtsr1: RTSR1,
    ///0x04 - EXTI falling trigger selection register
    pub ftsr1: FTSR1,
    ///0x08 - EXTI software interrupt event register
    pub swier1: SWIER1,
    ///0x0c - EXTI rising edge pending register
    pub rpr1: RPR1,
    ///0x10 - EXTI falling edge pending register
    pub fpr1: FPR1,
    _reserved5: [u8; 0x4c],
    ///0x60 - EXTI external interrupt selection register
    pub exticr1: EXTICR1,
    ///0x64 - EXTI external interrupt selection register
    pub exticr2: EXTICR2,
    ///0x68 - EXTI external interrupt selection register
    pub exticr3: EXTICR3,
    ///0x6c - EXTI external interrupt selection register
    pub exticr4: EXTICR4,
    _reserved9: [u8; 0x10],
    ///0x80 - EXTI CPU wakeup with interrupt mask register
    pub imr1: IMR1,
    ///0x84 - EXTI CPU wakeup with event mask register
    pub emr1: EMR1,
}
///RTSR1 (rw) register accessor: an alias for `Reg<RTSR1_SPEC>`
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
///EXTI rising trigger selection register
pub mod rtsr1;
///FTSR1 (rw) register accessor: an alias for `Reg<FTSR1_SPEC>`
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
///EXTI falling trigger selection register
pub mod ftsr1;
///SWIER1 (rw) register accessor: an alias for `Reg<SWIER1_SPEC>`
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
///EXTI software interrupt event register
pub mod swier1;
///RPR1 (rw) register accessor: an alias for `Reg<RPR1_SPEC>`
pub type RPR1 = crate::Reg<rpr1::RPR1_SPEC>;
///EXTI rising edge pending register
pub mod rpr1;
///FPR1 (rw) register accessor: an alias for `Reg<FPR1_SPEC>`
pub type FPR1 = crate::Reg<fpr1::FPR1_SPEC>;
///EXTI falling edge pending register
pub mod fpr1;
///EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///EXTI external interrupt selection register
pub mod exticr1;
///EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///EXTI external interrupt selection register
pub mod exticr2;
///EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///EXTI external interrupt selection register
pub mod exticr3;
///EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///EXTI external interrupt selection register
pub mod exticr4;
///IMR1 (rw) register accessor: an alias for `Reg<IMR1_SPEC>`
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
///EXTI CPU wakeup with interrupt mask register
pub mod imr1;
///EMR1 (rw) register accessor: an alias for `Reg<EMR1_SPEC>`
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
///EXTI CPU wakeup with event mask register
pub mod emr1;
