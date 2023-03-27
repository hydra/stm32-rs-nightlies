///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    pub idcode: IDCODE,
    ///0x04 - Debug MCU Configuration Register
    pub cr: CR,
    ///0x08 - Debug MCU APB1 freeze register
    pub apb1_fz: APB1_FZ,
    ///0x0c - Debug MCU APB2 freeze register
    pub apb2_fz: APB2_FZ,
}
///IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///MCU Device ID Code Register
pub mod idcode;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Debug MCU Configuration Register
pub mod cr;
///APB1_FZ (rw) register accessor: an alias for `Reg<APB1_FZ_SPEC>`
pub type APB1_FZ = crate::Reg<apb1_fz::APB1_FZ_SPEC>;
///Debug MCU APB1 freeze register
pub mod apb1_fz;
///APB2_FZ (rw) register accessor: an alias for `Reg<APB2_FZ_SPEC>`
pub type APB2_FZ = crate::Reg<apb2_fz::APB2_FZ_SPEC>;
///Debug MCU APB2 freeze register
pub mod apb2_fz;
