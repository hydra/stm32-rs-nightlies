///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - VREF_BUF Control and Status Register
    pub csr: CSR,
    ///0x04 - VREF_BUF Calibration Control Register
    pub ccr: CCR,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///VREF_BUF Control and Status Register
pub mod csr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///VREF_BUF Calibration Control Register
pub mod ccr;
