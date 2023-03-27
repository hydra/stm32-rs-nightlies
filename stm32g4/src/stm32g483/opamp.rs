///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OPAMP1 control/status register
    pub opamp1_csr: OPAMP1_CSR,
    ///0x04 - OPAMP2 control/status register
    pub opamp2_csr: OPAMP2_CSR,
    ///0x08 - OPAMP3 control/status register
    pub opamp3_csr: OPAMP3_CSR,
    ///0x0c - OPAMP4 control/status register
    pub opamp4_csr: OPAMP4_CSR,
    ///0x10 - OPAMP5 control/status register
    pub opamp5_csr: OPAMP5_CSR,
    ///0x14 - OPAMP6 control/status register
    pub opamp6_csr: OPAMP6_CSR,
    ///0x18 - OPAMP1 control/status register
    pub opamp1_tcmr: OPAMP1_TCMR,
    ///0x1c - OPAMP2 control/status register
    pub opamp2_tcmr: OPAMP2_TCMR,
    ///0x20 - OPAMP3 control/status register
    pub opamp3_tcmr: OPAMP3_TCMR,
    ///0x24 - OPAMP4 control/status register
    pub opamp4_tcmr: OPAMP4_TCMR,
    ///0x28 - OPAMP5 control/status register
    pub opamp5_tcmr: OPAMP5_TCMR,
    ///0x2c - OPAMP6 control/status register
    pub opamp6_tcmr: OPAMP6_TCMR,
}
///OPAMP1_CSR (rw) register accessor: an alias for `Reg<OPAMP1_CSR_SPEC>`
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
///OPAMP1 control/status register
pub mod opamp1_csr;
///OPAMP2_CSR (rw) register accessor: an alias for `Reg<OPAMP2_CSR_SPEC>`
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>;
///OPAMP2 control/status register
pub mod opamp2_csr;
///OPAMP3_CSR (rw) register accessor: an alias for `Reg<OPAMP3_CSR_SPEC>`
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSR_SPEC>;
///OPAMP3 control/status register
pub mod opamp3_csr;
///OPAMP4_CSR (rw) register accessor: an alias for `Reg<OPAMP4_CSR_SPEC>`
pub type OPAMP4_CSR = crate::Reg<opamp4_csr::OPAMP4_CSR_SPEC>;
///OPAMP4 control/status register
pub mod opamp4_csr;
///OPAMP5_CSR (rw) register accessor: an alias for `Reg<OPAMP5_CSR_SPEC>`
pub type OPAMP5_CSR = crate::Reg<opamp5_csr::OPAMP5_CSR_SPEC>;
///OPAMP5 control/status register
pub mod opamp5_csr;
///OPAMP6_CSR (rw) register accessor: an alias for `Reg<OPAMP6_CSR_SPEC>`
pub type OPAMP6_CSR = crate::Reg<opamp6_csr::OPAMP6_CSR_SPEC>;
///OPAMP6 control/status register
pub mod opamp6_csr;
///OPAMP1_TCMR (rw) register accessor: an alias for `Reg<OPAMP1_TCMR_SPEC>`
pub type OPAMP1_TCMR = crate::Reg<opamp1_tcmr::OPAMP1_TCMR_SPEC>;
///OPAMP1 control/status register
pub mod opamp1_tcmr;
///OPAMP2_TCMR (rw) register accessor: an alias for `Reg<OPAMP2_TCMR_SPEC>`
pub type OPAMP2_TCMR = crate::Reg<opamp2_tcmr::OPAMP2_TCMR_SPEC>;
///OPAMP2 control/status register
pub mod opamp2_tcmr;
///OPAMP3_TCMR (rw) register accessor: an alias for `Reg<OPAMP3_TCMR_SPEC>`
pub type OPAMP3_TCMR = crate::Reg<opamp3_tcmr::OPAMP3_TCMR_SPEC>;
///OPAMP3 control/status register
pub mod opamp3_tcmr;
///OPAMP4_TCMR (rw) register accessor: an alias for `Reg<OPAMP4_TCMR_SPEC>`
pub type OPAMP4_TCMR = crate::Reg<opamp4_tcmr::OPAMP4_TCMR_SPEC>;
///OPAMP4 control/status register
pub mod opamp4_tcmr;
///OPAMP5_TCMR (rw) register accessor: an alias for `Reg<OPAMP5_TCMR_SPEC>`
pub type OPAMP5_TCMR = crate::Reg<opamp5_tcmr::OPAMP5_TCMR_SPEC>;
///OPAMP5 control/status register
pub mod opamp5_tcmr;
///OPAMP6_TCMR (rw) register accessor: an alias for `Reg<OPAMP6_TCMR_SPEC>`
pub type OPAMP6_TCMR = crate::Reg<opamp6_tcmr::OPAMP6_TCMR_SPEC>;
///OPAMP6 control/status register
pub mod opamp6_tcmr;
