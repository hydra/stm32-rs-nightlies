///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - channel x configuration register
    pub cr: CR,
    ///0x04 - channel x number of data register
    pub ndtr: NDTR,
    ///0x08 - channel x peripheral address register
    pub par: PAR,
    ///0x0c - channel x memory address register
    pub mar: MAR,
    _reserved_end: [u8; 0x04],
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///channel x configuration register
pub mod cr;
///NDTR (rw) register accessor: an alias for `Reg<NDTR_SPEC>`
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
///channel x number of data register
pub mod ndtr;
///PAR (rw) register accessor: an alias for `Reg<PAR_SPEC>`
pub type PAR = crate::Reg<par::PAR_SPEC>;
///channel x peripheral address register
pub mod par;
///MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`
pub type MAR = crate::Reg<mar::MAR_SPEC>;
///channel x memory address register
pub mod mar;
