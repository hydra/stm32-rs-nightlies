///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub cr1: CR1,
    ///0x04 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub cr2: CR2,
    _reserved2: [u8; 0x04],
    ///0x0c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub fltcr: FLTCR,
    ///0x10 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub atcr1: ATCR1,
    ///0x14 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub atseedr: ATSEEDR,
    ///0x18 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub ator: ATOR,
    _reserved6: [u8; 0x04],
    ///0x20 - This register can be written only when the APB access is secure.
    pub smcr: SMCR,
    _reserved7: [u8; 0x08],
    ///0x2c - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub ier: IER,
    ///0x30 - This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
    pub sr: SR,
    ///0x34 - TAMP non-secure masked interrupt status register
    pub misr: MISR,
    ///0x38 - TAMP secure masked interrupt status register
    pub smisr: SMISR,
    ///0x3c - TAMP status clear register
    pub scr: SCR,
    ///0x40 - TAMP monotonic counter register
    pub countr: COUNTR,
    _reserved13: [u8; 0x0c],
    ///0x50 - TAMP configuration register
    pub cfgr: CFGR,
    _reserved14: [u8; 0xac],
    ///0x100..0x180 - TAMP backup %s register
    pub bkpr: [BKPR; 32],
    _reserved15: [u8; 0x026c],
    ///0x3ec - TAMP hardware configuration register 2
    pub hwcfgr2: HWCFGR2,
    ///0x3f0 - TAMP hardware configuration register 1
    pub hwcfgr1: HWCFGR1,
    ///0x3f4 - TAMP version register
    pub verr: VERR,
    ///0x3f8 - TAMP identification register
    pub ipidr: IPIDR,
    ///0x3fc - TAMP size identification register
    pub sidr: SIDR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod cr2;
///FLTCR (rw) register accessor: an alias for `Reg<FLTCR_SPEC>`
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod fltcr;
///ATCR1 (rw) register accessor: an alias for `Reg<ATCR1_SPEC>`
pub type ATCR1 = crate::Reg<atcr1::ATCR1_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod atcr1;
///ATSEEDR (w) register accessor: an alias for `Reg<ATSEEDR_SPEC>`
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDR_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod atseedr;
///ATOR (r) register accessor: an alias for `Reg<ATOR_SPEC>`
pub type ATOR = crate::Reg<ator::ATOR_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod ator;
///SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
///This register can be written only when the APB access is secure.
pub mod smcr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod ier;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
pub mod sr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///TAMP non-secure masked interrupt status register
pub mod misr;
///SMISR (r) register accessor: an alias for `Reg<SMISR_SPEC>`
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
///TAMP secure masked interrupt status register
pub mod smisr;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///TAMP status clear register
pub mod scr;
///COUNTR (r) register accessor: an alias for `Reg<COUNTR_SPEC>`
pub type COUNTR = crate::Reg<countr::COUNTR_SPEC>;
///TAMP monotonic counter register
pub mod countr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///TAMP configuration register
pub mod cfgr;
///BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
///TAMP backup %s register
pub mod bkpr;
///HWCFGR2 (r) register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///TAMP hardware configuration register 2
pub mod hwcfgr2;
///HWCFGR1 (r) register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///TAMP hardware configuration register 1
pub mod hwcfgr1;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///TAMP version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///TAMP identification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///TAMP size identification register
pub mod sidr;
