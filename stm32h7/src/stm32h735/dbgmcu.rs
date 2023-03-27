///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DBGMCU Identity Code Register
    pub idc: IDC,
    ///0x04 - DBGMCU Configuration Register
    pub cr: CR,
    _reserved2: [u8; 0x2c],
    ///0x34 - DBGMCU APB3 peripheral freeze register CPU1
    pub apb3fz1: APB3FZ1,
    ///0x38 - DBGMCU APB3 peripheral freeze register CPU2
    pub apb3fz2: APB3FZ2,
    ///0x3c - DBGMCU APB1L peripheral freeze register
    pub apb1lfz1: APB1LFZ1,
    ///0x40 - DBGMCU APB1L peripheral freeze register CPU2
    pub apb1lfz2: APB1LFZ2,
    _reserved6: [u8; 0x08],
    ///0x4c - DBGMCU APB2 peripheral freeze register
    pub apb2fz1: APB2FZ1,
    ///0x50 - DBGMCU APB2 peripheral freeze register CPU2
    pub apb2fz2: APB2FZ2,
    ///0x54 - DBGMCU APB4 peripheral freeze register
    pub apb4fz1: APB4FZ1,
    ///0x58 - DBGMCU APB4 peripheral freeze register CPU2
    pub apb4fz2: APB4FZ2,
}
///IDC (r) register accessor: an alias for `Reg<IDC_SPEC>`
pub type IDC = crate::Reg<idc::IDC_SPEC>;
///DBGMCU Identity Code Register
pub mod idc;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DBGMCU Configuration Register
pub mod cr;
///APB3FZ1 (rw) register accessor: an alias for `Reg<APB3FZ1_SPEC>`
pub type APB3FZ1 = crate::Reg<apb3fz1::APB3FZ1_SPEC>;
///DBGMCU APB3 peripheral freeze register CPU1
pub mod apb3fz1;
///APB3FZ2 (rw) register accessor: an alias for `Reg<APB3FZ2_SPEC>`
pub type APB3FZ2 = crate::Reg<apb3fz2::APB3FZ2_SPEC>;
///DBGMCU APB3 peripheral freeze register CPU2
pub mod apb3fz2;
///APB1LFZ1 (rw) register accessor: an alias for `Reg<APB1LFZ1_SPEC>`
pub type APB1LFZ1 = crate::Reg<apb1lfz1::APB1LFZ1_SPEC>;
///DBGMCU APB1L peripheral freeze register
pub mod apb1lfz1;
///APB1LFZ2 (rw) register accessor: an alias for `Reg<APB1LFZ2_SPEC>`
pub type APB1LFZ2 = crate::Reg<apb1lfz2::APB1LFZ2_SPEC>;
///DBGMCU APB1L peripheral freeze register CPU2
pub mod apb1lfz2;
///APB2FZ1 (rw) register accessor: an alias for `Reg<APB2FZ1_SPEC>`
pub type APB2FZ1 = crate::Reg<apb2fz1::APB2FZ1_SPEC>;
///DBGMCU APB2 peripheral freeze register
pub mod apb2fz1;
///APB2FZ2 (rw) register accessor: an alias for `Reg<APB2FZ2_SPEC>`
pub type APB2FZ2 = crate::Reg<apb2fz2::APB2FZ2_SPEC>;
///DBGMCU APB2 peripheral freeze register CPU2
pub mod apb2fz2;
///APB4FZ1 (rw) register accessor: an alias for `Reg<APB4FZ1_SPEC>`
pub type APB4FZ1 = crate::Reg<apb4fz1::APB4FZ1_SPEC>;
///DBGMCU APB4 peripheral freeze register
pub mod apb4fz1;
///APB4FZ2 (rw) register accessor: an alias for `Reg<APB4FZ2_SPEC>`
pub type APB4FZ2 = crate::Reg<apb4fz2::APB4FZ2_SPEC>;
///DBGMCU APB4 peripheral freeze register CPU2
pub mod apb4fz2;
