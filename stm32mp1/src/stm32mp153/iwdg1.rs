///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Key register
    pub iwdg_kr: IWDG_KR,
    ///0x04 - Prescaler register
    pub iwdg_pr: IWDG_PR,
    ///0x08 - Reload register
    pub iwdg_rlr: IWDG_RLR,
    ///0x0c - Status register
    pub iwdg_sr: IWDG_SR,
    ///0x10 - Window register
    pub iwdg_winr: IWDG_WINR,
    _reserved5: [u8; 0x03dc],
    ///0x3f0 - IWDG hardware configuration register
    pub iwdg_hwcfgr: IWDG_HWCFGR,
    ///0x3f4 - IWDG version register
    pub iwdg_verr: IWDG_VERR,
    ///0x3f8 - IWDG identification register
    pub iwdg_idr: IWDG_IDR,
    ///0x3fc - IWDG size identification register
    pub iwdg_sidr: IWDG_SIDR,
}
///IWDG_KR (w) register accessor: an alias for `Reg<IWDG_KR_SPEC>`
pub type IWDG_KR = crate::Reg<iwdg_kr::IWDG_KR_SPEC>;
///Key register
pub mod iwdg_kr;
///IWDG_PR (rw) register accessor: an alias for `Reg<IWDG_PR_SPEC>`
pub type IWDG_PR = crate::Reg<iwdg_pr::IWDG_PR_SPEC>;
///Prescaler register
pub mod iwdg_pr;
///IWDG_RLR (rw) register accessor: an alias for `Reg<IWDG_RLR_SPEC>`
pub type IWDG_RLR = crate::Reg<iwdg_rlr::IWDG_RLR_SPEC>;
///Reload register
pub mod iwdg_rlr;
///IWDG_SR (r) register accessor: an alias for `Reg<IWDG_SR_SPEC>`
pub type IWDG_SR = crate::Reg<iwdg_sr::IWDG_SR_SPEC>;
///Status register
pub mod iwdg_sr;
///IWDG_WINR (rw) register accessor: an alias for `Reg<IWDG_WINR_SPEC>`
pub type IWDG_WINR = crate::Reg<iwdg_winr::IWDG_WINR_SPEC>;
///Window register
pub mod iwdg_winr;
///IWDG_HWCFGR (r) register accessor: an alias for `Reg<IWDG_HWCFGR_SPEC>`
pub type IWDG_HWCFGR = crate::Reg<iwdg_hwcfgr::IWDG_HWCFGR_SPEC>;
///IWDG hardware configuration register
pub mod iwdg_hwcfgr;
///IWDG_VERR (r) register accessor: an alias for `Reg<IWDG_VERR_SPEC>`
pub type IWDG_VERR = crate::Reg<iwdg_verr::IWDG_VERR_SPEC>;
///IWDG version register
pub mod iwdg_verr;
///IWDG_IDR (r) register accessor: an alias for `Reg<IWDG_IDR_SPEC>`
pub type IWDG_IDR = crate::Reg<iwdg_idr::IWDG_IDR_SPEC>;
///IWDG identification register
pub mod iwdg_idr;
///IWDG_SIDR (r) register accessor: an alias for `Reg<IWDG_SIDR_SPEC>`
pub type IWDG_SIDR = crate::Reg<iwdg_sidr::IWDG_SIDR_SPEC>;
///IWDG size identification register
pub mod iwdg_sidr;
