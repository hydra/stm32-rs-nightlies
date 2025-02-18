///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    ///0x1c - control and status register
    pub csr: CSR,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///control and status register
pub mod csr;
