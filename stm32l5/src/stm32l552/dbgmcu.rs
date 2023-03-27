///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DBGMCU_IDCODE
    pub idcode: IDCODE,
    ///0x04 - Debug MCU configuration register
    pub cr: CR,
    ///0x08 - Debug MCU APB1 freeze register1
    pub apb1lfzr: APB1LFZR,
    ///0x0c - Debug MCU APB1 freeze register 2
    pub apb1hfzr: APB1HFZR,
    ///0x10 - Debug MCU APB2 freeze register
    pub apb2fzr: APB2FZR,
}
///IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///DBGMCU_IDCODE
pub mod idcode;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Debug MCU configuration register
pub mod cr;
///APB1LFZR (rw) register accessor: an alias for `Reg<APB1LFZR_SPEC>`
pub type APB1LFZR = crate::Reg<apb1lfzr::APB1LFZR_SPEC>;
///Debug MCU APB1 freeze register1
pub mod apb1lfzr;
///APB1HFZR (rw) register accessor: an alias for `Reg<APB1HFZR_SPEC>`
pub type APB1HFZR = crate::Reg<apb1hfzr::APB1HFZR_SPEC>;
///Debug MCU APB1 freeze register 2
pub mod apb1hfzr;
///APB2FZR (rw) register accessor: an alias for `Reg<APB2FZR_SPEC>`
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
///Debug MCU APB2 freeze register
pub mod apb2fzr;
