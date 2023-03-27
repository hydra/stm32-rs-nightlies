///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - QUADSPI control register
    pub quadspi_cr: QUADSPI_CR,
    ///0x04 - QUADSPI device configuration register
    pub quadspi_dcr: QUADSPI_DCR,
    ///0x08 - QUADSPI status register
    pub quadspi_sr: QUADSPI_SR,
    ///0x0c - QUADSPI flag clear register
    pub quadspi_fcr: QUADSPI_FCR,
    ///0x10 - QUADSPI data length register
    pub quadspi_dlr: QUADSPI_DLR,
    ///0x14 - QUADSPI communication configuration register
    pub quadspi_ccr: QUADSPI_CCR,
    ///0x18 - QUADSPI address register
    pub quadspi_ar: QUADSPI_AR,
    ///0x1c - QUADSPI alternate bytes registers
    pub quadspi_abr: QUADSPI_ABR,
    ///0x20 - QUADSPI data register
    pub quadspi_dr: QUADSPI_DR,
    ///0x24 - QUADSPI polling status mask register
    pub quadspi_psmkr: QUADSPI_PSMKR,
    ///0x28 - QUADSPI polling status match register
    pub quadspi_psmar: QUADSPI_PSMAR,
    ///0x2c - QUADSPI polling interval register
    pub quadspi_pir: QUADSPI_PIR,
    ///0x30 - QUADSPI low-power timeout register
    pub quadspi_lptr: QUADSPI_LPTR,
    _reserved13: [u8; 0x03bc],
    ///0x3f0 - QUADSPI HW configuration register
    pub quadspi_hwcfgr: QUADSPI_HWCFGR,
    ///0x3f4 - QUADSPI version register
    pub quadspi_verr: QUADSPI_VERR,
    ///0x3f8 - QUADSPI identification register
    pub quadspi_ipidr: QUADSPI_IPIDR,
    ///0x3fc - QUADSPI size identification register
    pub quadspi_sidr: QUADSPI_SIDR,
}
///QUADSPI_CR (rw) register accessor: an alias for `Reg<QUADSPI_CR_SPEC>`
pub type QUADSPI_CR = crate::Reg<quadspi_cr::QUADSPI_CR_SPEC>;
///QUADSPI control register
pub mod quadspi_cr;
///QUADSPI_DCR (rw) register accessor: an alias for `Reg<QUADSPI_DCR_SPEC>`
pub type QUADSPI_DCR = crate::Reg<quadspi_dcr::QUADSPI_DCR_SPEC>;
///QUADSPI device configuration register
pub mod quadspi_dcr;
///QUADSPI_SR (r) register accessor: an alias for `Reg<QUADSPI_SR_SPEC>`
pub type QUADSPI_SR = crate::Reg<quadspi_sr::QUADSPI_SR_SPEC>;
///QUADSPI status register
pub mod quadspi_sr;
///QUADSPI_FCR (w) register accessor: an alias for `Reg<QUADSPI_FCR_SPEC>`
pub type QUADSPI_FCR = crate::Reg<quadspi_fcr::QUADSPI_FCR_SPEC>;
///QUADSPI flag clear register
pub mod quadspi_fcr;
///QUADSPI_DLR (rw) register accessor: an alias for `Reg<QUADSPI_DLR_SPEC>`
pub type QUADSPI_DLR = crate::Reg<quadspi_dlr::QUADSPI_DLR_SPEC>;
///QUADSPI data length register
pub mod quadspi_dlr;
///QUADSPI_CCR (rw) register accessor: an alias for `Reg<QUADSPI_CCR_SPEC>`
pub type QUADSPI_CCR = crate::Reg<quadspi_ccr::QUADSPI_CCR_SPEC>;
///QUADSPI communication configuration register
pub mod quadspi_ccr;
///QUADSPI_AR (rw) register accessor: an alias for `Reg<QUADSPI_AR_SPEC>`
pub type QUADSPI_AR = crate::Reg<quadspi_ar::QUADSPI_AR_SPEC>;
///QUADSPI address register
pub mod quadspi_ar;
///QUADSPI_ABR (rw) register accessor: an alias for `Reg<QUADSPI_ABR_SPEC>`
pub type QUADSPI_ABR = crate::Reg<quadspi_abr::QUADSPI_ABR_SPEC>;
///QUADSPI alternate bytes registers
pub mod quadspi_abr;
///QUADSPI_DR (rw) register accessor: an alias for `Reg<QUADSPI_DR_SPEC>`
pub type QUADSPI_DR = crate::Reg<quadspi_dr::QUADSPI_DR_SPEC>;
///QUADSPI data register
pub mod quadspi_dr;
///QUADSPI_PSMKR (rw) register accessor: an alias for `Reg<QUADSPI_PSMKR_SPEC>`
pub type QUADSPI_PSMKR = crate::Reg<quadspi_psmkr::QUADSPI_PSMKR_SPEC>;
///QUADSPI polling status mask register
pub mod quadspi_psmkr;
///QUADSPI_PSMAR (rw) register accessor: an alias for `Reg<QUADSPI_PSMAR_SPEC>`
pub type QUADSPI_PSMAR = crate::Reg<quadspi_psmar::QUADSPI_PSMAR_SPEC>;
///QUADSPI polling status match register
pub mod quadspi_psmar;
///QUADSPI_PIR (rw) register accessor: an alias for `Reg<QUADSPI_PIR_SPEC>`
pub type QUADSPI_PIR = crate::Reg<quadspi_pir::QUADSPI_PIR_SPEC>;
///QUADSPI polling interval register
pub mod quadspi_pir;
///QUADSPI_LPTR (rw) register accessor: an alias for `Reg<QUADSPI_LPTR_SPEC>`
pub type QUADSPI_LPTR = crate::Reg<quadspi_lptr::QUADSPI_LPTR_SPEC>;
///QUADSPI low-power timeout register
pub mod quadspi_lptr;
///QUADSPI_HWCFGR (r) register accessor: an alias for `Reg<QUADSPI_HWCFGR_SPEC>`
pub type QUADSPI_HWCFGR = crate::Reg<quadspi_hwcfgr::QUADSPI_HWCFGR_SPEC>;
///QUADSPI HW configuration register
pub mod quadspi_hwcfgr;
///QUADSPI_VERR (r) register accessor: an alias for `Reg<QUADSPI_VERR_SPEC>`
pub type QUADSPI_VERR = crate::Reg<quadspi_verr::QUADSPI_VERR_SPEC>;
///QUADSPI version register
pub mod quadspi_verr;
///QUADSPI_IPIDR (r) register accessor: an alias for `Reg<QUADSPI_IPIDR_SPEC>`
pub type QUADSPI_IPIDR = crate::Reg<quadspi_ipidr::QUADSPI_IPIDR_SPEC>;
///QUADSPI identification register
pub mod quadspi_ipidr;
///QUADSPI_SIDR (r) register accessor: an alias for `Reg<QUADSPI_SIDR_SPEC>`
pub type QUADSPI_SIDR = crate::Reg<quadspi_sidr::QUADSPI_SIDR_SPEC>;
///QUADSPI size identification register
pub mod quadspi_sidr;
