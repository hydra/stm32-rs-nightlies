///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Power control register (PWR_CR)
    pub cr: CR,
    ///0x04 - Power control register (PWR_CR)
    pub csr: CSR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Power control register (PWR_CR)
pub mod cr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Power control register (PWR_CR)
pub mod csr;
