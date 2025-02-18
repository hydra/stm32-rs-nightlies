///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Code segment start address
    pub cssa: CSSA,
    ///0x04 - Code segment length
    pub csl: CSL,
    ///0x08 - Non-volatile data segment start address
    pub nvdssa: NVDSSA,
    ///0x0c - Non-volatile data segment length
    pub nvdsl: NVDSL,
    ///0x10 - Volatile data segment start address
    pub vdssa: VDSSA,
    ///0x14 - Volatile data segment length
    pub vdsl: VDSL,
    _reserved6: [u8; 0x08],
    ///0x20 - Configuration register
    pub cr: CR,
}
///CSSA (rw) register accessor: an alias for `Reg<CSSA_SPEC>`
pub type CSSA = crate::Reg<cssa::CSSA_SPEC>;
///Code segment start address
pub mod cssa;
///CSL (rw) register accessor: an alias for `Reg<CSL_SPEC>`
pub type CSL = crate::Reg<csl::CSL_SPEC>;
///Code segment length
pub mod csl;
///NVDSSA (rw) register accessor: an alias for `Reg<NVDSSA_SPEC>`
pub type NVDSSA = crate::Reg<nvdssa::NVDSSA_SPEC>;
///Non-volatile data segment start address
pub mod nvdssa;
///NVDSL (rw) register accessor: an alias for `Reg<NVDSL_SPEC>`
pub type NVDSL = crate::Reg<nvdsl::NVDSL_SPEC>;
///Non-volatile data segment length
pub mod nvdsl;
///VDSSA (rw) register accessor: an alias for `Reg<VDSSA_SPEC>`
pub type VDSSA = crate::Reg<vdssa::VDSSA_SPEC>;
///Volatile data segment start address
pub mod vdssa;
///VDSL (rw) register accessor: an alias for `Reg<VDSL_SPEC>`
pub type VDSL = crate::Reg<vdsl::VDSL_SPEC>;
///Volatile data segment length
pub mod vdsl;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Configuration register
pub mod cr;
