///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - Channel x configuration register
    pub cr: CR,
    ///0x04 - Channel x number of data to transfer register
    pub ndtr: NDTR,
    ///0x08 - Channel x peripheral address register
    pub par: PAR,
    ///0x0c - Channel x memory 0 address register
    pub m0ar: M0AR,
    ///0x10 - Channel x memory 1 address register
    pub m1ar: M1AR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Channel x configuration register
pub mod cr;
///NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
///Channel x number of data to transfer register
pub mod ndtr;
///PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`
pub type PAR = crate::Reg<par::PAR_SPEC>;
///Channel x peripheral address register
pub mod par;
///M0AR (rw) register accessor: an alias for `Reg<M0AR_SPEC>`
pub type M0AR = crate::Reg<m0ar::M0AR_SPEC>;
///Channel x memory 0 address register
pub mod m0ar;
///M1AR (rw) register accessor: an alias for `Reg<M1AR_SPEC>`
pub type M1AR = crate::Reg<m1ar::M1AR_SPEC>;
///Channel x memory 1 address register
pub mod m1ar;
