///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Global configuration register
    pub gcr: GCR,
    ///0x04 - A Configuration register 1
    pub acr1: ACR1,
    ///0x08 - A Configuration register 2
    pub acr2: ACR2,
    ///0x0c - A frame configuration register
    pub afrcr: AFRCR,
    ///0x10 - A Slot register
    pub aslotr: ASLOTR,
    ///0x14 - A Interrupt mask register
    pub aim: AIM,
    ///0x18 - A Status register
    pub asr: ASR,
    ///0x1c - A Clear flag register
    pub aclrfr: ACLRFR,
    ///0x20 - A Data register
    pub adr: ADR,
    ///0x24 - B Configuration register 1
    pub bcr1: BCR1,
    ///0x28 - B Configuration register 2
    pub bcr2: BCR2,
    ///0x2c - B frame configuration register
    pub bfrcr: BFRCR,
    ///0x30 - B Slot register
    pub bslotr: BSLOTR,
    ///0x34 - B Interrupt mask register
    pub bim: BIM,
    ///0x38 - B Status register
    pub bsr: BSR,
    ///0x3c - B Clear flag register
    pub bclrfr: BCLRFR,
    ///0x40 - B Data register
    pub bdr: BDR,
    ///0x44 - PDM control register
    pub pdmcr: PDMCR,
    ///0x48 - PDM delay register
    pub pdmdly: PDMDLY,
}
///GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
///Global configuration register
pub mod gcr;
///ACR1 (rw) register accessor: an alias for `Reg<ACR1_SPEC>`
pub type ACR1 = crate::Reg<acr1::ACR1_SPEC>;
///A Configuration register 1
pub mod acr1;
///BCR1 (rw) register accessor: an alias for `Reg<BCR1_SPEC>`
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
///B Configuration register 1
pub mod bcr1;
///ACR2 (rw) register accessor: an alias for `Reg<ACR2_SPEC>`
pub type ACR2 = crate::Reg<acr2::ACR2_SPEC>;
///A Configuration register 2
pub mod acr2;
///BCR2 (rw) register accessor: an alias for `Reg<BCR2_SPEC>`
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
///B Configuration register 2
pub mod bcr2;
///AFRCR (rw) register accessor: an alias for `Reg<AFRCR_SPEC>`
pub type AFRCR = crate::Reg<afrcr::AFRCR_SPEC>;
///A frame configuration register
pub mod afrcr;
///BFRCR (rw) register accessor: an alias for `Reg<BFRCR_SPEC>`
pub type BFRCR = crate::Reg<bfrcr::BFRCR_SPEC>;
///B frame configuration register
pub mod bfrcr;
///ASLOTR (rw) register accessor: an alias for `Reg<ASLOTR_SPEC>`
pub type ASLOTR = crate::Reg<aslotr::ASLOTR_SPEC>;
///A Slot register
pub mod aslotr;
///BSLOTR (rw) register accessor: an alias for `Reg<BSLOTR_SPEC>`
pub type BSLOTR = crate::Reg<bslotr::BSLOTR_SPEC>;
///B Slot register
pub mod bslotr;
///AIM (rw) register accessor: an alias for `Reg<AIM_SPEC>`
pub type AIM = crate::Reg<aim::AIM_SPEC>;
///A Interrupt mask register
pub mod aim;
///BIM (rw) register accessor: an alias for `Reg<BIM_SPEC>`
pub type BIM = crate::Reg<bim::BIM_SPEC>;
///B Interrupt mask register
pub mod bim;
///ASR (r) register accessor: an alias for `Reg<ASR_SPEC>`
pub type ASR = crate::Reg<asr::ASR_SPEC>;
///A Status register
pub mod asr;
///BSR (r) register accessor: an alias for `Reg<BSR_SPEC>`
pub type BSR = crate::Reg<bsr::BSR_SPEC>;
///B Status register
pub mod bsr;
///ACLRFR (w) register accessor: an alias for `Reg<ACLRFR_SPEC>`
pub type ACLRFR = crate::Reg<aclrfr::ACLRFR_SPEC>;
///A Clear flag register
pub mod aclrfr;
///BCLRFR (w) register accessor: an alias for `Reg<BCLRFR_SPEC>`
pub type BCLRFR = crate::Reg<bclrfr::BCLRFR_SPEC>;
///B Clear flag register
pub mod bclrfr;
///ADR (rw) register accessor: an alias for `Reg<ADR_SPEC>`
pub type ADR = crate::Reg<adr::ADR_SPEC>;
///A Data register
pub mod adr;
///BDR (rw) register accessor: an alias for `Reg<BDR_SPEC>`
pub type BDR = crate::Reg<bdr::BDR_SPEC>;
///B Data register
pub mod bdr;
///PDMCR (rw) register accessor: an alias for `Reg<PDMCR_SPEC>`
pub type PDMCR = crate::Reg<pdmcr::PDMCR_SPEC>;
///PDM control register
pub mod pdmcr;
///PDMDLY (rw) register accessor: an alias for `Reg<PDMDLY_SPEC>`
pub type PDMDLY = crate::Reg<pdmdly::PDMDLY_SPEC>;
///PDM delay register
pub mod pdmdly;
