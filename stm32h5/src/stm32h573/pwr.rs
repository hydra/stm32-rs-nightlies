///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - PWR power mode control register
    pub pmcr: PMCR,
    ///0x04 - PWR status register
    pub pmsr: PMSR,
    _reserved2: [u8; 0x08],
    ///0x10 - PWR voltage scaling control register
    pub voscr: VOSCR,
    ///0x14 - PWR voltage scaling status register
    pub vossr: VOSSR,
    _reserved4: [u8; 0x08],
    ///0x20 - PWR Backup domain control register
    pub bdcr: BDCR,
    ///0x24 - PWR Backup domain control register
    pub dbpcr: DBPCR,
    ///0x28 - PWR Backup domain status register
    pub bdsr: BDSR,
    ///0x2c - PWR USB Type-C power delivery register
    pub ucpdr: UCPDR,
    ///0x30 - PWR supply configuration control register
    pub sccr: SCCR,
    ///0x34 - PWR voltage monitor control register
    pub vmcr: VMCR,
    ///0x38 - PWR USB supply control register
    pub usbscr: USBSCR,
    ///0x3c - PWR voltage monitor status register
    pub vmsr: VMSR,
    ///0x40 - PWR wakeup status clear register
    pub wuscr: WUSCR,
    ///0x44 - PWR wakeup status register
    pub wusr: WUSR,
    ///0x48 - PWR wakeup configuration register
    pub wucr: WUCR,
    _reserved15: [u8; 0x04],
    ///0x50 - PWR I/O retention register
    pub ioretr: IORETR,
    _reserved16: [u8; 0xac],
    ///0x100 - PWR security configuration register
    pub seccfgr: SECCFGR,
    ///0x104 - PWR privilege configuration register
    pub privcfgr: PRIVCFGR,
}
///PMCR (rw) register accessor: an alias for `Reg<PMCR_SPEC>`
pub type PMCR = crate::Reg<pmcr::PMCR_SPEC>;
///PWR power mode control register
pub mod pmcr;
///PMSR (r) register accessor: an alias for `Reg<PMSR_SPEC>`
pub type PMSR = crate::Reg<pmsr::PMSR_SPEC>;
///PWR status register
pub mod pmsr;
///VOSCR (rw) register accessor: an alias for `Reg<VOSCR_SPEC>`
pub type VOSCR = crate::Reg<voscr::VOSCR_SPEC>;
///PWR voltage scaling control register
pub mod voscr;
///VOSSR (r) register accessor: an alias for `Reg<VOSSR_SPEC>`
pub type VOSSR = crate::Reg<vossr::VOSSR_SPEC>;
///PWR voltage scaling status register
pub mod vossr;
///BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///PWR Backup domain control register
pub mod bdcr;
///DBPCR (rw) register accessor: an alias for `Reg<DBPCR_SPEC>`
pub type DBPCR = crate::Reg<dbpcr::DBPCR_SPEC>;
///PWR Backup domain control register
pub mod dbpcr;
///BDSR (r) register accessor: an alias for `Reg<BDSR_SPEC>`
pub type BDSR = crate::Reg<bdsr::BDSR_SPEC>;
///PWR Backup domain status register
pub mod bdsr;
///UCPDR (rw) register accessor: an alias for `Reg<UCPDR_SPEC>`
pub type UCPDR = crate::Reg<ucpdr::UCPDR_SPEC>;
///PWR USB Type-C power delivery register
pub mod ucpdr;
///SCCR (rw) register accessor: an alias for `Reg<SCCR_SPEC>`
pub type SCCR = crate::Reg<sccr::SCCR_SPEC>;
///PWR supply configuration control register
pub mod sccr;
///VMCR (rw) register accessor: an alias for `Reg<VMCR_SPEC>`
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
///PWR voltage monitor control register
pub mod vmcr;
///USBSCR (rw) register accessor: an alias for `Reg<USBSCR_SPEC>`
pub type USBSCR = crate::Reg<usbscr::USBSCR_SPEC>;
///PWR USB supply control register
pub mod usbscr;
///VMSR (r) register accessor: an alias for `Reg<VMSR_SPEC>`
pub type VMSR = crate::Reg<vmsr::VMSR_SPEC>;
///PWR voltage monitor status register
pub mod vmsr;
///WUSCR (w) register accessor: an alias for `Reg<WUSCR_SPEC>`
pub type WUSCR = crate::Reg<wuscr::WUSCR_SPEC>;
///PWR wakeup status clear register
pub mod wuscr;
///WUSR (r) register accessor: an alias for `Reg<WUSR_SPEC>`
pub type WUSR = crate::Reg<wusr::WUSR_SPEC>;
///PWR wakeup status register
pub mod wusr;
///WUCR (rw) register accessor: an alias for `Reg<WUCR_SPEC>`
pub type WUCR = crate::Reg<wucr::WUCR_SPEC>;
///PWR wakeup configuration register
pub mod wucr;
///IORETR (rw) register accessor: an alias for `Reg<IORETR_SPEC>`
pub type IORETR = crate::Reg<ioretr::IORETR_SPEC>;
///PWR I/O retention register
pub mod ioretr;
///SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
///PWR security configuration register
pub mod seccfgr;
///PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
///PWR privilege configuration register
pub mod privcfgr;
