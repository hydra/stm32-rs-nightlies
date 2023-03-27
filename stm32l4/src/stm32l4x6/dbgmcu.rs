///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    pub idcode: IDCODE,
    ///0x04 - Debug MCU Configuration Register
    pub cr: CR,
    ///0x08 - APB Low Freeze Register 1
    pub apb1fzr1: APB1FZR1,
    ///0x0c - APB Low Freeze Register 2
    pub apb1fzr2: APB1FZR2,
    ///0x10 - APB High Freeze Register
    pub apb2fzr: APB2FZR,
}
///IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///MCU Device ID Code Register
pub mod idcode;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Debug MCU Configuration Register
pub mod cr;
///APB1FZR1 (rw) register accessor: an alias for `Reg<APB1FZR1_SPEC>`
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1_SPEC>;
///APB Low Freeze Register 1
pub mod apb1fzr1;
///APB1FZR2 (rw) register accessor: an alias for `Reg<APB1FZR2_SPEC>`
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2_SPEC>;
///APB Low Freeze Register 2
pub mod apb1fzr2;
///APB2FZR (rw) register accessor: an alias for `Reg<APB2FZR_SPEC>`
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
///APB High Freeze Register
pub mod apb2fzr;
