///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - COMP1_CSR
    pub comp1_csr: COMP1_CSR,
    ///0x04 - COMP2_CSR
    pub comp2_csr: COMP2_CSR,
}
///COMP1_CSR (rw) register accessor: an alias for `Reg<COMP1_CSR_SPEC>`
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
///COMP1_CSR
pub mod comp1_csr;
///COMP2_CSR (rw) register accessor: an alias for `Reg<COMP2_CSR_SPEC>`
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
///COMP2_CSR
pub mod comp2_csr;
