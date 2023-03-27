///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - comparator control and status register
    pub csr: CSR,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///comparator control and status register
pub mod csr;
