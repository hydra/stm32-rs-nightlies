///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - memory remap register
    pub memrmp: MEMRMP,
    ///0x04 - peripheral mode configuration register
    pub pmc: PMC,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
}
///MEMRMP (rw) register accessor: an alias for `Reg<MEMRMP_SPEC>`
pub type MEMRMP = crate::Reg<memrmp::MEMRMP_SPEC>;
///memory remap register
pub mod memrmp;
///PMC (rw) register accessor: an alias for `Reg<PMC_SPEC>`
pub type PMC = crate::Reg<pmc::PMC_SPEC>;
///peripheral mode configuration register
pub mod pmc;
///EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///external interrupt configuration register 1
pub mod exticr1;
///EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///external interrupt configuration register 2
pub mod exticr2;
///EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///external interrupt configuration register 3
pub mod exticr3;
///EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///external interrupt configuration register 4
pub mod exticr4;
