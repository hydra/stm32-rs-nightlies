///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Floating-point context control register
    pub fpccr: FPCCR,
    ///0x04 - Floating-point context address register
    pub fpcar: FPCAR,
    ///0x08 - Floating-point status control register
    pub fpscr: FPSCR,
}
///FPCCR (rw) register accessor: an alias for `Reg<FPCCR_SPEC>`
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
///Floating-point context control register
pub mod fpccr;
///FPCAR (rw) register accessor: an alias for `Reg<FPCAR_SPEC>`
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
///Floating-point context address register
pub mod fpcar;
///FPSCR (rw) register accessor: an alias for `Reg<FPSCR_SPEC>`
pub type FPSCR = crate::Reg<fpscr::FPSCR_SPEC>;
///Floating-point status control register
pub mod fpscr;
