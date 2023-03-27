///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DBGMCU_IDCODE
    pub idcode: IDCODE,
    ///0x04 - DBGMCU_CR
    pub cr: CR,
}
///IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///DBGMCU_IDCODE
pub mod idcode;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DBGMCU_CR
pub mod cr;
