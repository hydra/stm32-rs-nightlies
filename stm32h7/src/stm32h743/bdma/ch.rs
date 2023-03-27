///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - DMA channel x configuration register
    pub cr: CR,
    ///0x04 - DMA channel x number of data register
    pub ndtr: NDTR,
    ///0x08 - This register must not be written when the channel is enabled.
    pub par: PAR,
    ///0x0c - This register must not be written when the channel is enabled.
    pub m0ar: M0AR,
    ///0x10 - Channel x memory 1 address register
    pub m1ar: M1AR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DMA channel x configuration register
pub mod cr;
///NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
///DMA channel x number of data register
pub mod ndtr;
///PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`
pub type PAR = crate::Reg<par::PAR_SPEC>;
///This register must not be written when the channel is enabled.
pub mod par;
///M0AR (rw) register accessor: an alias for `Reg<M0AR_SPEC>`
pub type M0AR = crate::Reg<m0ar::M0AR_SPEC>;
///This register must not be written when the channel is enabled.
pub mod m0ar;
///M1AR (rw) register accessor: an alias for `Reg<M1AR_SPEC>`
pub type M1AR = crate::Reg<m1ar::M1AR_SPEC>;
///Channel x memory 1 address register
pub mod m1ar;
