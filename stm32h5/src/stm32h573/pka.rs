///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - PKA control register
    pub cr: CR,
    ///0x04 - PKA status register
    pub sr: SR,
    ///0x08 - PKA clear flag register
    pub clrfr: CLRFR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///PKA control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///PKA status register
pub mod sr;
///CLRFR (w) register accessor: an alias for `Reg<CLRFR_SPEC>`
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
///PKA clear flag register
pub mod clrfr;
