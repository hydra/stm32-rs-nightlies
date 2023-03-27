///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub wwdg_cr: WWDG_CR,
    ///0x04 - Configuration register
    pub wwdg_cfr: WWDG_CFR,
    ///0x08 - Status register
    pub wwdg_sr: WWDG_SR,
    _reserved3: [u8; 0x03e4],
    ///0x3f0 - WWDG hardware configuration register
    pub wwdg_hwcfgr: WWDG_HWCFGR,
    ///0x3f4 - WWDG version register
    pub wwdg_verr: WWDG_VERR,
    ///0x3f8 - WWDG ID register
    pub wwdg_ipidr: WWDG_IPIDR,
    ///0x3fc - WWDG size ID register
    pub wwdg_sidr: WWDG_SIDR,
}
///WWDG_CR (rw) register accessor: an alias for `Reg<WWDG_CR_SPEC>`
pub type WWDG_CR = crate::Reg<wwdg_cr::WWDG_CR_SPEC>;
///Control register
pub mod wwdg_cr;
///WWDG_CFR (rw) register accessor: an alias for `Reg<WWDG_CFR_SPEC>`
pub type WWDG_CFR = crate::Reg<wwdg_cfr::WWDG_CFR_SPEC>;
///Configuration register
pub mod wwdg_cfr;
///WWDG_SR (rw) register accessor: an alias for `Reg<WWDG_SR_SPEC>`
pub type WWDG_SR = crate::Reg<wwdg_sr::WWDG_SR_SPEC>;
///Status register
pub mod wwdg_sr;
///WWDG_HWCFGR (r) register accessor: an alias for `Reg<WWDG_HWCFGR_SPEC>`
pub type WWDG_HWCFGR = crate::Reg<wwdg_hwcfgr::WWDG_HWCFGR_SPEC>;
///WWDG hardware configuration register
pub mod wwdg_hwcfgr;
///WWDG_VERR (r) register accessor: an alias for `Reg<WWDG_VERR_SPEC>`
pub type WWDG_VERR = crate::Reg<wwdg_verr::WWDG_VERR_SPEC>;
///WWDG version register
pub mod wwdg_verr;
///WWDG_IPIDR (r) register accessor: an alias for `Reg<WWDG_IPIDR_SPEC>`
pub type WWDG_IPIDR = crate::Reg<wwdg_ipidr::WWDG_IPIDR_SPEC>;
///WWDG ID register
pub mod wwdg_ipidr;
///WWDG_SIDR (r) register accessor: an alias for `Reg<WWDG_SIDR_SPEC>`
pub type WWDG_SIDR = crate::Reg<wwdg_sidr::WWDG_SIDR_SPEC>;
///WWDG size ID register
pub mod wwdg_sidr;
