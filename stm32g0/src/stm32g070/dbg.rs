///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    pub idcode: IDCODE,
    ///0x04 - Debug MCU Configuration Register
    pub cr: CR,
    ///0x08 - DBG APB freeze register 1
    pub apb_fz1: APB_FZ1,
    ///0x0c - DBG APB freeze register 2
    pub apb_fz2: APB_FZ2,
}
///IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///MCU Device ID Code Register
pub mod idcode;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Debug MCU Configuration Register
pub mod cr;
///APB_FZ1 (rw) register accessor: an alias for `Reg<APB_FZ1_SPEC>`
pub type APB_FZ1 = crate::Reg<apb_fz1::APB_FZ1_SPEC>;
///DBG APB freeze register 1
pub mod apb_fz1;
///APB_FZ2 (rw) register accessor: an alias for `Reg<APB_FZ2_SPEC>`
pub type APB_FZ2 = crate::Reg<apb_fz2::APB_FZ2_SPEC>;
///DBG APB freeze register 2
pub mod apb_fz2;
