///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1c],
    ///0x1c - control and status register
    pub comp1_csr: COMP1_CSR,
    ///0x20 - control and status register
    pub comp2_csr: COMP2_CSR,
    _reserved2: [u8; 0x04],
    ///0x28 - control and status register
    pub comp4_csr: COMP4_CSR,
    _reserved3: [u8; 0x04],
    ///0x30 - control and status register
    pub comp6_csr: COMP6_CSR,
}
///COMP2_CSR (rw) register accessor: an alias for `Reg<COMP2_CSR_SPEC>`
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
///control and status register
pub mod comp2_csr;
///COMP4_CSR (rw) register accessor: an alias for `Reg<COMP4_CSR_SPEC>`
pub type COMP4_CSR = crate::Reg<comp4_csr::COMP4_CSR_SPEC>;
///control and status register
pub mod comp4_csr;
///COMP6_CSR (rw) register accessor: an alias for `Reg<COMP6_CSR_SPEC>`
pub type COMP6_CSR = crate::Reg<comp6_csr::COMP6_CSR_SPEC>;
///control and status register
pub mod comp6_csr;
///COMP1_CSR (rw) register accessor: an alias for `Reg<COMP1_CSR_SPEC>`
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
///control and status register
pub mod comp1_csr;
