///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Key register
    pub kr: KR,
    ///0x04 - Prescaler register
    pub pr: PR,
    ///0x08 - Reload register
    pub rlr: RLR,
    ///0x0c - Status register
    pub sr: SR,
    ///0x10 - Window register
    pub winr: WINR,
    _reserved5: [u8; 0x03dc],
    ///0x3f0 - hardware configuration register
    pub hwcfgr: HWCFGR,
    ///0x3f4 - EXTI IP Version register
    pub verr: VERR,
    ///0x3f8 - EXTI Identification register
    pub ipidr: IPIDR,
    ///0x3fc - EXTI Size ID register
    pub sidr: SIDR,
}
///KR (w) register accessor: an alias for `Reg<KR_SPEC>`
pub type KR = crate::Reg<kr::KR_SPEC>;
///Key register
pub mod kr;
///PR (rw) register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///Prescaler register
pub mod pr;
///RLR (rw) register accessor: an alias for `Reg<RLR_SPEC>`
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
///Reload register
pub mod rlr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///WINR (rw) register accessor: an alias for `Reg<WINR_SPEC>`
pub type WINR = crate::Reg<winr::WINR_SPEC>;
///Window register
pub mod winr;
///HWCFGR (rw) register accessor: an alias for `Reg<HWCFGR_SPEC>`
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
///hardware configuration register
pub mod hwcfgr;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///EXTI IP Version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///EXTI Identification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///EXTI Size ID register
pub mod sidr;
