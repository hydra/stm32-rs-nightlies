///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - DMA channel 1 configuration register
    pub cr: CR,
    ///0x04 - DMA channel 1 number of data tegister
    pub ndtr: NDTR,
    ///0x08 - DMA channel 1 peripheral address
    pub par: PAR,
    ///0x0c - DMA channel 1 memory address
    pub mar: MAR,
    _reserved_end: [u8; 0x04],
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DMA channel 1 configuration register
pub mod cr;
///NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
///DMA channel 1 number of data tegister
pub mod ndtr;
///PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`
pub type PAR = crate::Reg<par::PAR_SPEC>;
///DMA channel 1 peripheral address
pub mod par;
///MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`
pub type MAR = crate::Reg<mar::MAR_SPEC>;
///DMA channel 1 memory address
pub mod mar;
