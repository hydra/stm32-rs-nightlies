///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - VREFBUF control and status register
    pub vrefbuf_csr: VREFBUF_CSR,
    ///0x04 - VREFBUF calibration control register
    pub vrefbuf_ccr: VREFBUF_CCR,
}
///VREFBUF_CSR (rw) register accessor: an alias for `Reg<VREFBUF_CSR_SPEC>`
pub type VREFBUF_CSR = crate::Reg<vrefbuf_csr::VREFBUF_CSR_SPEC>;
///VREFBUF control and status register
pub mod vrefbuf_csr;
///VREFBUF_CCR (rw) register accessor: an alias for `Reg<VREFBUF_CCR_SPEC>`
pub type VREFBUF_CCR = crate::Reg<vrefbuf_ccr::VREFBUF_CCR_SPEC>;
///VREFBUF calibration control register
pub mod vrefbuf_ccr;
