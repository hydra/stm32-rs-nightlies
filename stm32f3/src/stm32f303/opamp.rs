///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x38],
    ///0x38 - OPAMP1 control register
    pub opamp1_csr: OPAMP1_CSR,
    ///0x3c - OPAMP2 control register
    pub opamp2_csr: OPAMP2_CSR,
    ///0x40 - OPAMP3 control register
    pub opamp3_csr: OPAMP3_CSR,
    ///0x44 - OPAMP4 control register
    pub opamp4_csr: OPAMP4_CSR,
}
///OPAMP2_CSR (rw) register accessor: an alias for `Reg<OPAMP2_CSR_SPEC>`
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>;
///OPAMP2 control register
pub mod opamp2_csr;
///OPAMP3_CSR (rw) register accessor: an alias for `Reg<OPAMP3_CSR_SPEC>`
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSR_SPEC>;
///OPAMP3 control register
pub mod opamp3_csr;
///OPAMP4_CSR (rw) register accessor: an alias for `Reg<OPAMP4_CSR_SPEC>`
pub type OPAMP4_CSR = crate::Reg<opamp4_csr::OPAMP4_CSR_SPEC>;
///OPAMP4 control register
pub mod opamp4_csr;
///OPAMP1_CSR (rw) register accessor: an alias for `Reg<OPAMP1_CSR_SPEC>`
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
///OPAMP1 control register
pub mod opamp1_csr;
