///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OPAMP1 control/status register
    pub opamp1_csr: OPAMP1_CSR,
    ///0x04 - OPAMP1 trimming register in normal mode
    pub opamp1_otr: OPAMP1_OTR,
    ///0x08 - OPAMP1 trimming register in high-speed mode
    pub opamp1_hsotr: OPAMP1_HSOTR,
    ///0x0c - OPAMP option register
    pub opamp_or: OPAMP_OR,
}
///OPAMP1_CSR (rw) register accessor: an alias for `Reg<OPAMP1_CSR_SPEC>`
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
///OPAMP1 control/status register
pub mod opamp1_csr;
///OPAMP1_OTR (rw) register accessor: an alias for `Reg<OPAMP1_OTR_SPEC>`
pub type OPAMP1_OTR = crate::Reg<opamp1_otr::OPAMP1_OTR_SPEC>;
///OPAMP1 trimming register in normal mode
pub mod opamp1_otr;
///OPAMP1_HSOTR (rw) register accessor: an alias for `Reg<OPAMP1_HSOTR_SPEC>`
pub type OPAMP1_HSOTR = crate::Reg<opamp1_hsotr::OPAMP1_HSOTR_SPEC>;
///OPAMP1 trimming register in high-speed mode
pub mod opamp1_hsotr;
///OPAMP_OR (rw) register accessor: an alias for `Reg<OPAMP_OR_SPEC>`
pub type OPAMP_OR = crate::Reg<opamp_or::OPAMP_OR_SPEC>;
///OPAMP option register
pub mod opamp_or;
