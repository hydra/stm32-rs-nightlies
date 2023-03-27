///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SAI global configuration register
    pub gcr: GCR,
    ///0x04 - SAI configuration register 1
    pub acr1: ACR1,
    ///0x08 - SAI configuration register 2
    pub acr2: ACR2,
    ///0x0c - SAI frame configuration register
    pub afrcr: AFRCR,
    ///0x10 - SAI slot register
    pub aslotr: ASLOTR,
    ///0x14 - SAI interrupt mask register
    pub aim: AIM,
    ///0x18 - SAI status register
    pub asr: ASR,
    ///0x1c - SAI clear flag register
    pub aclrfr: ACLRFR,
    ///0x20 - SAI data register
    pub adr: ADR,
    ///0x24 - SAI configuration register 1
    pub bcr1: BCR1,
    ///0x28 - SAI configuration register 2
    pub bcr2: BCR2,
    ///0x2c - SAI frame configuration register
    pub bfrcr: BFRCR,
    ///0x30 - SAI slot register
    pub bslotr: BSLOTR,
    ///0x34 - SAI interrupt mask register
    pub bim: BIM,
    ///0x38 - SAI status register
    pub bsr: BSR,
    ///0x3c - SAI clear flag register
    pub bclrfr: BCLRFR,
    ///0x40 - SAI data register
    pub bdr: BDR,
    ///0x44 - SAI PDM control register
    pub pdmcr: PDMCR,
    ///0x48 - SAI PDM delay register
    pub pdmdly: PDMDLY,
}
///GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
///SAI global configuration register
pub mod gcr;
///ACR1 (rw) register accessor: an alias for `Reg<ACR1_SPEC>`
pub type ACR1 = crate::Reg<acr1::ACR1_SPEC>;
///SAI configuration register 1
pub mod acr1;
///ACR2 (rw) register accessor: an alias for `Reg<ACR2_SPEC>`
pub type ACR2 = crate::Reg<acr2::ACR2_SPEC>;
///SAI configuration register 2
pub mod acr2;
///AFRCR (rw) register accessor: an alias for `Reg<AFRCR_SPEC>`
pub type AFRCR = crate::Reg<afrcr::AFRCR_SPEC>;
///SAI frame configuration register
pub mod afrcr;
///ASLOTR (rw) register accessor: an alias for `Reg<ASLOTR_SPEC>`
pub type ASLOTR = crate::Reg<aslotr::ASLOTR_SPEC>;
///SAI slot register
pub mod aslotr;
///AIM (rw) register accessor: an alias for `Reg<AIM_SPEC>`
pub type AIM = crate::Reg<aim::AIM_SPEC>;
///SAI interrupt mask register
pub mod aim;
///ASR (r) register accessor: an alias for `Reg<ASR_SPEC>`
pub type ASR = crate::Reg<asr::ASR_SPEC>;
///SAI status register
pub mod asr;
///ACLRFR (w) register accessor: an alias for `Reg<ACLRFR_SPEC>`
pub type ACLRFR = crate::Reg<aclrfr::ACLRFR_SPEC>;
///SAI clear flag register
pub mod aclrfr;
///ADR (rw) register accessor: an alias for `Reg<ADR_SPEC>`
pub type ADR = crate::Reg<adr::ADR_SPEC>;
///SAI data register
pub mod adr;
///BCR1 (rw) register accessor: an alias for `Reg<BCR1_SPEC>`
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
///SAI configuration register 1
pub mod bcr1;
///BCR2 (rw) register accessor: an alias for `Reg<BCR2_SPEC>`
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
///SAI configuration register 2
pub mod bcr2;
///BFRCR (rw) register accessor: an alias for `Reg<BFRCR_SPEC>`
pub type BFRCR = crate::Reg<bfrcr::BFRCR_SPEC>;
///SAI frame configuration register
pub mod bfrcr;
///BSLOTR (rw) register accessor: an alias for `Reg<BSLOTR_SPEC>`
pub type BSLOTR = crate::Reg<bslotr::BSLOTR_SPEC>;
///SAI slot register
pub mod bslotr;
///BIM (rw) register accessor: an alias for `Reg<BIM_SPEC>`
pub type BIM = crate::Reg<bim::BIM_SPEC>;
///SAI interrupt mask register
pub mod bim;
///BSR (r) register accessor: an alias for `Reg<BSR_SPEC>`
pub type BSR = crate::Reg<bsr::BSR_SPEC>;
///SAI status register
pub mod bsr;
///BCLRFR (w) register accessor: an alias for `Reg<BCLRFR_SPEC>`
pub type BCLRFR = crate::Reg<bclrfr::BCLRFR_SPEC>;
///SAI clear flag register
pub mod bclrfr;
///BDR (rw) register accessor: an alias for `Reg<BDR_SPEC>`
pub type BDR = crate::Reg<bdr::BDR_SPEC>;
///SAI data register
pub mod bdr;
///PDMCR (rw) register accessor: an alias for `Reg<PDMCR_SPEC>`
pub type PDMCR = crate::Reg<pdmcr::PDMCR_SPEC>;
///SAI PDM control register
pub mod pdmcr;
///PDMDLY (rw) register accessor: an alias for `Reg<PDMDLY_SPEC>`
pub type PDMDLY = crate::Reg<pdmdly::PDMDLY_SPEC>;
///SAI PDM delay register
pub mod pdmdly;
