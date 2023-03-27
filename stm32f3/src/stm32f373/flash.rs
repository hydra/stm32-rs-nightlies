///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Flash access control register
    pub acr: ACR,
    ///0x04 - Flash key register
    pub keyr: KEYR,
    ///0x08 - Flash option key register
    pub optkeyr: OPTKEYR,
    ///0x0c - Flash status register
    pub sr: SR,
    ///0x10 - Flash control register
    pub cr: CR,
    ///0x14 - Flash address register
    pub ar: AR,
    _reserved6: [u8; 0x04],
    ///0x1c - Option byte register
    pub obr: OBR,
    ///0x20 - Write protection register
    pub wrpr: WRPR,
}
///ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///Flash access control register
pub mod acr;
///KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
///Flash key register
pub mod keyr;
///OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///Flash option key register
pub mod optkeyr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Flash status register
pub mod sr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Flash control register
pub mod cr;
///AR (w) register accessor: an alias for `Reg<AR_SPEC>`
pub type AR = crate::Reg<ar::AR_SPEC>;
///Flash address register
pub mod ar;
///OBR (r) register accessor: an alias for `Reg<OBR_SPEC>`
pub type OBR = crate::Reg<obr::OBR_SPEC>;
///Option byte register
pub mod obr;
///WRPR (r) register accessor: an alias for `Reg<WRPR_SPEC>`
pub type WRPR = crate::Reg<wrpr::WRPR_SPEC>;
///Write protection register
pub mod wrpr;
