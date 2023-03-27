///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - PWR control register 1
    pub pwr_cr1: PWR_CR1,
    ///0x04 - PWR control register 2
    pub pwr_cr2: PWR_CR2,
    ///0x08 - PWR control register 3
    pub pwr_cr3: PWR_CR3,
    ///0x0c - PWR voltage scaling register
    pub pwr_vosr: PWR_VOSR,
    ///0x10 - PWR supply voltage monitoring control register
    pub pwr_svmcr: PWR_SVMCR,
    ///0x14 - PWR wakeup control register 1
    pub pwr_wucr1: PWR_WUCR1,
    ///0x18 - PWR wakeup control register 2
    pub pwr_wucr2: PWR_WUCR2,
    ///0x1c - PWR wakeup control register 3
    pub pwr_wucr3: PWR_WUCR3,
    ///0x20 - PWR Backup domain control register 1
    pub pwr_bdcr1: PWR_BDCR1,
    ///0x24 - PWR Backup domain control register 2
    pub pwr_bdcr2: PWR_BDCR2,
    ///0x28 - PWR disable Backup domain register
    pub pwr_dbpr: PWR_DBPR,
    ///0x2c - PWR USB Type-C™ and Power Delivery register
    pub pwr_ucpdr: PWR_UCPDR,
    ///0x30 - PWR security configuration register
    pub pwr_seccfgr: PWR_SECCFGR,
    ///0x34 - PWR privilege control register
    pub pwr_privcfgr: PWR_PRIVCFGR,
    ///0x38 - PWR status register
    pub pwr_sr: PWR_SR,
    ///0x3c -
    pub pwr_svmsr: PWR_SVMSR,
    ///0x40 - PWR Backup domain status register
    pub pwr_bdsr: PWR_BDSR,
    ///0x44 - PWR wakeup status register
    pub pwr_wusr: PWR_WUSR,
    ///0x48 - PWR wakeup status clear register
    pub pwr_wuscr: PWR_WUSCR,
    ///0x4c - PWR apply pull configuration register
    pub pwr_apcr: PWR_APCR,
    ///0x50 - PWR port A pull-up control register
    pub pwr_pucra: PWR_PUCRA,
    ///0x54 - PWR port A pull-down control register
    pub pwr_pdcra: PWR_PDCRA,
    ///0x58 - PWR port B pull-up control register
    pub pwr_pucrb: PWR_PUCRB,
    ///0x5c - PWR port B pull-down control register
    pub pwr_pdcrb: PWR_PDCRB,
    ///0x60 - PWR port C pull-up control register
    pub pwr_pucrc: PWR_PUCRC,
    ///0x64 - PWR port C pull-down control register
    pub pwr_pdcrc: PWR_PDCRC,
    ///0x68 - PWR port D pull-up control register
    pub pwr_pucrd: PWR_PUCRD,
    ///0x6c - PWR port D pull-down control register
    pub pwr_pdcrd: PWR_PDCRD,
    ///0x70 - PWR port E pull-up control register
    pub pwr_pucre: PWR_PUCRE,
    ///0x74 - PWR port E pull-down control register
    pub pwr_pdcre: PWR_PDCRE,
    ///0x78 - PWR port F pull-up control register
    pub pwr_pucrf: PWR_PUCRF,
    ///0x7c - PWR port F pull-down control register
    pub pwr_pdcrf: PWR_PDCRF,
    ///0x80 - PWR port G pull-up control register
    pub pwr_pucrg: PWR_PUCRG,
    ///0x84 - PWR port G pull-down control register
    pub pwr_pdcrg: PWR_PDCRG,
    ///0x88 - PWR port H pull-up control register
    pub pwr_pucrh: PWR_PUCRH,
    ///0x8c - PWR port H pull-down control register
    pub pwr_pdcrh: PWR_PDCRH,
    ///0x90 - PWR port I pull-up control register
    pub pwr_pucri: PWR_PUCRI,
    ///0x94 - PWR port I pull-down control register
    pub pwr_pdcri: PWR_PDCRI,
}
///PWR_CR1 (rw) register accessor: an alias for `Reg<PWR_CR1_SPEC>`
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1_SPEC>;
///PWR control register 1
pub mod pwr_cr1;
///PWR_CR2 (rw) register accessor: an alias for `Reg<PWR_CR2_SPEC>`
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2_SPEC>;
///PWR control register 2
pub mod pwr_cr2;
///PWR_CR3 (rw) register accessor: an alias for `Reg<PWR_CR3_SPEC>`
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3_SPEC>;
///PWR control register 3
pub mod pwr_cr3;
///PWR_VOSR (rw) register accessor: an alias for `Reg<PWR_VOSR_SPEC>`
pub type PWR_VOSR = crate::Reg<pwr_vosr::PWR_VOSR_SPEC>;
///PWR voltage scaling register
pub mod pwr_vosr;
///PWR_SVMCR (rw) register accessor: an alias for `Reg<PWR_SVMCR_SPEC>`
pub type PWR_SVMCR = crate::Reg<pwr_svmcr::PWR_SVMCR_SPEC>;
///PWR supply voltage monitoring control register
pub mod pwr_svmcr;
///PWR_WUCR1 (rw) register accessor: an alias for `Reg<PWR_WUCR1_SPEC>`
pub type PWR_WUCR1 = crate::Reg<pwr_wucr1::PWR_WUCR1_SPEC>;
///PWR wakeup control register 1
pub mod pwr_wucr1;
///PWR_WUCR2 (rw) register accessor: an alias for `Reg<PWR_WUCR2_SPEC>`
pub type PWR_WUCR2 = crate::Reg<pwr_wucr2::PWR_WUCR2_SPEC>;
///PWR wakeup control register 2
pub mod pwr_wucr2;
///PWR_WUCR3 (rw) register accessor: an alias for `Reg<PWR_WUCR3_SPEC>`
pub type PWR_WUCR3 = crate::Reg<pwr_wucr3::PWR_WUCR3_SPEC>;
///PWR wakeup control register 3
pub mod pwr_wucr3;
///PWR_BDCR1 (rw) register accessor: an alias for `Reg<PWR_BDCR1_SPEC>`
pub type PWR_BDCR1 = crate::Reg<pwr_bdcr1::PWR_BDCR1_SPEC>;
///PWR Backup domain control register 1
pub mod pwr_bdcr1;
///PWR_BDCR2 (rw) register accessor: an alias for `Reg<PWR_BDCR2_SPEC>`
pub type PWR_BDCR2 = crate::Reg<pwr_bdcr2::PWR_BDCR2_SPEC>;
///PWR Backup domain control register 2
pub mod pwr_bdcr2;
///PWR_DBPR (rw) register accessor: an alias for `Reg<PWR_DBPR_SPEC>`
pub type PWR_DBPR = crate::Reg<pwr_dbpr::PWR_DBPR_SPEC>;
///PWR disable Backup domain register
pub mod pwr_dbpr;
///PWR_UCPDR (rw) register accessor: an alias for `Reg<PWR_UCPDR_SPEC>`
pub type PWR_UCPDR = crate::Reg<pwr_ucpdr::PWR_UCPDR_SPEC>;
///PWR USB Type-C™ and Power Delivery register
pub mod pwr_ucpdr;
///PWR_SECCFGR (rw) register accessor: an alias for `Reg<PWR_SECCFGR_SPEC>`
pub type PWR_SECCFGR = crate::Reg<pwr_seccfgr::PWR_SECCFGR_SPEC>;
///PWR security configuration register
pub mod pwr_seccfgr;
///PWR_PRIVCFGR (rw) register accessor: an alias for `Reg<PWR_PRIVCFGR_SPEC>`
pub type PWR_PRIVCFGR = crate::Reg<pwr_privcfgr::PWR_PRIVCFGR_SPEC>;
///PWR privilege control register
pub mod pwr_privcfgr;
///PWR_SR (rw) register accessor: an alias for `Reg<PWR_SR_SPEC>`
pub type PWR_SR = crate::Reg<pwr_sr::PWR_SR_SPEC>;
///PWR status register
pub mod pwr_sr;
///PWR_SVMSR (r) register accessor: an alias for `Reg<PWR_SVMSR_SPEC>`
pub type PWR_SVMSR = crate::Reg<pwr_svmsr::PWR_SVMSR_SPEC>;
///
pub mod pwr_svmsr;
///PWR_BDSR (r) register accessor: an alias for `Reg<PWR_BDSR_SPEC>`
pub type PWR_BDSR = crate::Reg<pwr_bdsr::PWR_BDSR_SPEC>;
///PWR Backup domain status register
pub mod pwr_bdsr;
///PWR_WUSR (r) register accessor: an alias for `Reg<PWR_WUSR_SPEC>`
pub type PWR_WUSR = crate::Reg<pwr_wusr::PWR_WUSR_SPEC>;
///PWR wakeup status register
pub mod pwr_wusr;
///PWR_WUSCR (w) register accessor: an alias for `Reg<PWR_WUSCR_SPEC>`
pub type PWR_WUSCR = crate::Reg<pwr_wuscr::PWR_WUSCR_SPEC>;
///PWR wakeup status clear register
pub mod pwr_wuscr;
///PWR_APCR (rw) register accessor: an alias for `Reg<PWR_APCR_SPEC>`
pub type PWR_APCR = crate::Reg<pwr_apcr::PWR_APCR_SPEC>;
///PWR apply pull configuration register
pub mod pwr_apcr;
///PWR_PUCRA (rw) register accessor: an alias for `Reg<PWR_PUCRA_SPEC>`
pub type PWR_PUCRA = crate::Reg<pwr_pucra::PWR_PUCRA_SPEC>;
///PWR port A pull-up control register
pub mod pwr_pucra;
///PWR_PDCRA (rw) register accessor: an alias for `Reg<PWR_PDCRA_SPEC>`
pub type PWR_PDCRA = crate::Reg<pwr_pdcra::PWR_PDCRA_SPEC>;
///PWR port A pull-down control register
pub mod pwr_pdcra;
///PWR_PUCRB (rw) register accessor: an alias for `Reg<PWR_PUCRB_SPEC>`
pub type PWR_PUCRB = crate::Reg<pwr_pucrb::PWR_PUCRB_SPEC>;
///PWR port B pull-up control register
pub mod pwr_pucrb;
///PWR_PDCRB (rw) register accessor: an alias for `Reg<PWR_PDCRB_SPEC>`
pub type PWR_PDCRB = crate::Reg<pwr_pdcrb::PWR_PDCRB_SPEC>;
///PWR port B pull-down control register
pub mod pwr_pdcrb;
///PWR_PUCRC (rw) register accessor: an alias for `Reg<PWR_PUCRC_SPEC>`
pub type PWR_PUCRC = crate::Reg<pwr_pucrc::PWR_PUCRC_SPEC>;
///PWR port C pull-up control register
pub mod pwr_pucrc;
///PWR_PDCRC (rw) register accessor: an alias for `Reg<PWR_PDCRC_SPEC>`
pub type PWR_PDCRC = crate::Reg<pwr_pdcrc::PWR_PDCRC_SPEC>;
///PWR port C pull-down control register
pub mod pwr_pdcrc;
///PWR_PUCRD (rw) register accessor: an alias for `Reg<PWR_PUCRD_SPEC>`
pub type PWR_PUCRD = crate::Reg<pwr_pucrd::PWR_PUCRD_SPEC>;
///PWR port D pull-up control register
pub mod pwr_pucrd;
///PWR_PDCRD (rw) register accessor: an alias for `Reg<PWR_PDCRD_SPEC>`
pub type PWR_PDCRD = crate::Reg<pwr_pdcrd::PWR_PDCRD_SPEC>;
///PWR port D pull-down control register
pub mod pwr_pdcrd;
///PWR_PUCRE (rw) register accessor: an alias for `Reg<PWR_PUCRE_SPEC>`
pub type PWR_PUCRE = crate::Reg<pwr_pucre::PWR_PUCRE_SPEC>;
///PWR port E pull-up control register
pub mod pwr_pucre;
///PWR_PDCRE (rw) register accessor: an alias for `Reg<PWR_PDCRE_SPEC>`
pub type PWR_PDCRE = crate::Reg<pwr_pdcre::PWR_PDCRE_SPEC>;
///PWR port E pull-down control register
pub mod pwr_pdcre;
///PWR_PUCRF (rw) register accessor: an alias for `Reg<PWR_PUCRF_SPEC>`
pub type PWR_PUCRF = crate::Reg<pwr_pucrf::PWR_PUCRF_SPEC>;
///PWR port F pull-up control register
pub mod pwr_pucrf;
///PWR_PDCRF (rw) register accessor: an alias for `Reg<PWR_PDCRF_SPEC>`
pub type PWR_PDCRF = crate::Reg<pwr_pdcrf::PWR_PDCRF_SPEC>;
///PWR port F pull-down control register
pub mod pwr_pdcrf;
///PWR_PUCRG (rw) register accessor: an alias for `Reg<PWR_PUCRG_SPEC>`
pub type PWR_PUCRG = crate::Reg<pwr_pucrg::PWR_PUCRG_SPEC>;
///PWR port G pull-up control register
pub mod pwr_pucrg;
///PWR_PDCRG (rw) register accessor: an alias for `Reg<PWR_PDCRG_SPEC>`
pub type PWR_PDCRG = crate::Reg<pwr_pdcrg::PWR_PDCRG_SPEC>;
///PWR port G pull-down control register
pub mod pwr_pdcrg;
///PWR_PUCRH (rw) register accessor: an alias for `Reg<PWR_PUCRH_SPEC>`
pub type PWR_PUCRH = crate::Reg<pwr_pucrh::PWR_PUCRH_SPEC>;
///PWR port H pull-up control register
pub mod pwr_pucrh;
///PWR_PDCRH (rw) register accessor: an alias for `Reg<PWR_PDCRH_SPEC>`
pub type PWR_PDCRH = crate::Reg<pwr_pdcrh::PWR_PDCRH_SPEC>;
///PWR port H pull-down control register
pub mod pwr_pdcrh;
///PWR_PUCRI (rw) register accessor: an alias for `Reg<PWR_PUCRI_SPEC>`
pub type PWR_PUCRI = crate::Reg<pwr_pucri::PWR_PUCRI_SPEC>;
///PWR port I pull-up control register
pub mod pwr_pucri;
///PWR_PDCRI (rw) register accessor: an alias for `Reg<PWR_PDCRI_SPEC>`
pub type PWR_PDCRI = crate::Reg<pwr_pdcri::PWR_PDCRI_SPEC>;
///PWR port I pull-down control register
pub mod pwr_pdcri;
