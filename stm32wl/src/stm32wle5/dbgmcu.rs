///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DBGMCU Identity Code Register
    pub idcoder: IDCODER,
    ///0x04 - DBGMCU Configuration Register
    pub cr: CR,
    _reserved2: [u8; 0x34],
    ///0x3c - DBGMCU CPU1 APB1 Peripheral Freeze Register 1
    pub apb1fzr1: APB1FZR1,
    _reserved3: [u8; 0x04],
    ///0x44 - DBGMCU CPU1 APB1 Peripheral Freeze Register 2
    pub apb1fzr2: APB1FZR2,
    _reserved4: [u8; 0x04],
    ///0x4c - DBGMCU CPU1 APB2 Peripheral Freeze Register
    pub apb2fzr: APB2FZR,
}
///IDCODER (r) register accessor: an alias for `Reg<IDCODER_SPEC>`
pub type IDCODER = crate::Reg<idcoder::IDCODER_SPEC>;
///DBGMCU Identity Code Register
pub mod idcoder;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DBGMCU Configuration Register
pub mod cr;
///APB1FZR1 (rw) register accessor: an alias for `Reg<APB1FZR1_SPEC>`
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1_SPEC>;
///DBGMCU CPU1 APB1 Peripheral Freeze Register 1
pub mod apb1fzr1;
///APB1FZR2 (rw) register accessor: an alias for `Reg<APB1FZR2_SPEC>`
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2_SPEC>;
///DBGMCU CPU1 APB1 Peripheral Freeze Register 2
pub mod apb1fzr2;
///APB2FZR (rw) register accessor: an alias for `Reg<APB2FZR_SPEC>`
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
///DBGMCU CPU1 APB2 Peripheral Freeze Register
pub mod apb2fzr;
