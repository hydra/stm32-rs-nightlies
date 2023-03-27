///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DBGMCU_IDCODE
    pub idcode: IDCODE,
    ///0x04 - Debug MCU configuration register
    pub cr: CR,
    ///0x08 - Debug MCU APB1 freeze register1
    pub apb_fz1: APB_FZ1,
    ///0x0c - Debug MCU APB1 freeze register 2
    pub apb_fz2: APB_FZ2,
}
///IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///DBGMCU_IDCODE
pub mod idcode;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Debug MCU configuration register
pub mod cr;
///APB_FZ1 (rw) register accessor: an alias for `Reg<APB_FZ1_SPEC>`
pub type APB_FZ1 = crate::Reg<apb_fz1::APB_FZ1_SPEC>;
///Debug MCU APB1 freeze register1
pub mod apb_fz1;
///APB_FZ2 (rw) register accessor: an alias for `Reg<APB_FZ2_SPEC>`
pub type APB_FZ2 = crate::Reg<apb_fz2::APB_FZ2_SPEC>;
///Debug MCU APB1 freeze register 2
pub mod apb_fz2;
