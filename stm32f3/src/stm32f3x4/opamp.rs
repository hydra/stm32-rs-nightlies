///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x3c],
    ///0x3c - OPAMP2 control register
    pub opamp2_csr: OPAMP2_CSR,
}
///OPAMP2_CSR (rw) register accessor: an alias for `Reg<OPAMP2_CSR_SPEC>`
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>;
///OPAMP2 control register
pub mod opamp2_csr;
