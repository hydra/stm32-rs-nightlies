///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control/status register
    pub csr: CSR,
    ///0x04 - offset trimming register for normal mode
    pub otr: OTR,
    ///0x08 - OPAMP offset trimming register for low power mode
    pub lpotr: LPOTR,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///control/status register
pub mod csr;
///OTR (rw) register accessor: an alias for `Reg<OTR_SPEC>`
pub type OTR = crate::Reg<otr::OTR_SPEC>;
///offset trimming register for normal mode
pub mod otr;
///LPOTR (rw) register accessor: an alias for `Reg<LPOTR_SPEC>`
pub type LPOTR = crate::Reg<lpotr::LPOTR_SPEC>;
///OPAMP offset trimming register for low power mode
pub mod lpotr;
