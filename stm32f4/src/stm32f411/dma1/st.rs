///Register block
#[repr(C)]
pub struct ST {
    ///0x00 - stream x configuration register
    pub cr: CR,
    ///0x04 - stream x number of data register
    pub ndtr: NDTR,
    ///0x08 - stream x peripheral address register
    pub par: PAR,
    ///0x0c - stream x memory 0 address register
    pub m0ar: M0AR,
    ///0x10 - stream x memory 1 address register
    pub m1ar: M1AR,
    ///0x14 - stream x FIFO control register
    pub fcr: FCR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///stream x configuration register
pub mod cr;
///NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
///stream x number of data register
pub mod ndtr;
///PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`
pub type PAR = crate::Reg<par::PAR_SPEC>;
///stream x peripheral address register
pub mod par;
///M0AR (rw) register accessor: an alias for `Reg<M0AR_SPEC>`
pub type M0AR = crate::Reg<m0ar::M0AR_SPEC>;
///stream x memory 0 address register
pub mod m0ar;
///M1AR (rw) register accessor: an alias for `Reg<M1AR_SPEC>`
pub type M1AR = crate::Reg<m1ar::M1AR_SPEC>;
///stream x memory 1 address register
pub mod m1ar;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///stream x FIFO control register
pub mod fcr;
