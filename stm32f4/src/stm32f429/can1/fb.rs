///Register block
#[repr(C)]
pub struct FB {
    ///0x00 - Filter bank 0 register 1
    pub fr1: FR1,
    ///0x04 - Filter bank 0 register 2
    pub fr2: FR2,
}
///FR1 (rw) register accessor: an alias for `Reg<FR1_SPEC>`
pub type FR1 = crate::Reg<fr1::FR1_SPEC>;
///Filter bank 0 register 1
pub mod fr1;
///FR2 (rw) register accessor: an alias for `Reg<FR2_SPEC>`
pub type FR2 = crate::Reg<fr2::FR2_SPEC>;
///Filter bank 0 register 2
pub mod fr2;
