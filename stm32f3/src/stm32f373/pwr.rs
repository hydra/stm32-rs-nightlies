///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - power control register
    pub cr: CR,
    ///0x04 - power control/status register
    pub csr: CSR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///power control register
pub mod cr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///power control/status register
pub mod csr;
