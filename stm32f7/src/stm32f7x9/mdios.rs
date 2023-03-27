///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MDIOS configuration register
    pub cr: CR,
    ///0x04 - MDIOS write flag register
    pub wrfr: WRFR,
    ///0x08 - MDIOS clear write flag register
    pub cwrfr: CWRFR,
    ///0x0c - MDIOS read flag register
    pub rdfr: RDFR,
    ///0x10 - MDIOS clear read flag register
    pub crdfr: CRDFR,
    ///0x14 - MDIOS status register
    pub sr: SR,
    ///0x18 - MDIOS clear flag register
    pub clrfr: CLRFR,
    ///0x1c..0x9c - MDIOS input data register %s
    pub dinr: [DINR; 32],
    ///0x9c..0x11c - MDIOS output data register %s
    pub doutr: [DOUTR; 32],
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///MDIOS configuration register
pub mod cr;
///WRFR (r) register accessor: an alias for `Reg<WRFR_SPEC>`
pub type WRFR = crate::Reg<wrfr::WRFR_SPEC>;
///MDIOS write flag register
pub mod wrfr;
///CWRFR (rw) register accessor: an alias for `Reg<CWRFR_SPEC>`
pub type CWRFR = crate::Reg<cwrfr::CWRFR_SPEC>;
///MDIOS clear write flag register
pub mod cwrfr;
///RDFR (r) register accessor: an alias for `Reg<RDFR_SPEC>`
pub type RDFR = crate::Reg<rdfr::RDFR_SPEC>;
///MDIOS read flag register
pub mod rdfr;
///CRDFR (rw) register accessor: an alias for `Reg<CRDFR_SPEC>`
pub type CRDFR = crate::Reg<crdfr::CRDFR_SPEC>;
///MDIOS clear read flag register
pub mod crdfr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///MDIOS status register
pub mod sr;
///CLRFR (rw) register accessor: an alias for `Reg<CLRFR_SPEC>`
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
///MDIOS clear flag register
pub mod clrfr;
///DINR (r) register accessor: an alias for `Reg<DINR_SPEC>`
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
///MDIOS input data register %s
pub mod dinr;
///DOUTR (rw) register accessor: an alias for `Reg<DOUTR_SPEC>`
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
///MDIOS output data register %s
pub mod doutr;
