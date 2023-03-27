///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    pub idcode: IDCODE,
    ///0x04 - Debug MCU Configuration Register
    pub cr: CR,
    ///0x08 - APB Low Freeze Register 1
    pub apb1l_fz: APB1L_FZ,
    ///0x0c - APB Low Freeze Register 2
    pub apb1h_fz: APB1H_FZ,
    ///0x10 - APB High Freeze Register
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
///APB1L_FZ (rw) register accessor: an alias for `Reg<APB1L_FZ_SPEC>`
pub type APB1L_FZ = crate::Reg<apb1l_fz::APB1L_FZ_SPEC>;
///APB Low Freeze Register 1
pub mod apb1l_fz;
///APB1H_FZ (rw) register accessor: an alias for `Reg<APB1H_FZ_SPEC>`
pub type APB1H_FZ = crate::Reg<apb1h_fz::APB1H_FZ_SPEC>;
///APB Low Freeze Register 2
pub mod apb1h_fz;
///APB2_FZ (rw) register accessor: an alias for `Reg<APB2_FZ_SPEC>`
pub type APB2_FZ = crate::Reg<apb2_fz::APB2_FZ_SPEC>;
///APB High Freeze Register
pub mod apb2_fz;
