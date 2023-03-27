///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ADC Common status register
    pub csr: CSR,
    _reserved1: [u8; 0x04],
    ///0x08 - ADC common control register
    pub ccr: CCR,
    ///0x0c - ADC common regular data register for dual and triple modes
    pub cdr: CDR,
}
///CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///ADC Common status register
pub mod csr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///ADC common control register
pub mod ccr;
///CDR (r) register accessor: an alias for `Reg<CDR_SPEC>`
pub type CDR = crate::Reg<cdr::CDR_SPEC>;
///ADC common regular data register for dual and triple modes
pub mod cdr;
