///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Flash access control register
    pub acr: ACR,
    ///0x04 - Flash key register
    pub keyr: KEYR,
    ///0x08 - Flash option key register
    pub optkeyr: OPTKEYR,
    ///0x0c - Status register
    pub sr: SR,
    ///0x10 - Control register
    pub cr: CR,
    ///0x14 - Flash option control register
    pub optcr: OPTCR,
    ///0x18 - Flash option control register 1
    pub optcr1: OPTCR1,
    ///0x1c - Flash option control register
    pub optcr2: OPTCR2,
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
///Status register
pub mod sr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///OPTCR (rw) register accessor: an alias for `Reg<OPTCR_SPEC>`
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
///Flash option control register
pub mod optcr;
///OPTCR1 (rw) register accessor: an alias for `Reg<OPTCR1_SPEC>`
pub type OPTCR1 = crate::Reg<optcr1::OPTCR1_SPEC>;
///Flash option control register 1
pub mod optcr1;
///OPTCR2 (rw) register accessor: an alias for `Reg<OPTCR2_SPEC>`
pub type OPTCR2 = crate::Reg<optcr2::OPTCR2_SPEC>;
///Flash option control register
pub mod optcr2;
