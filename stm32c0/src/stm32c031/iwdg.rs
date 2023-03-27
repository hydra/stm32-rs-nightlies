///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - IWDG key register
    pub kr: KR,
    ///0x04 - IWDG prescaler register
    pub pr: PR,
    ///0x08 - IWDG reload register
    pub rlr: RLR,
    ///0x0c - IWDG status register
    pub sr: SR,
    ///0x10 - IWDG window register
    pub winr: WINR,
}
///KR (w) register accessor: an alias for `Reg<KR_SPEC>`
pub type KR = crate::Reg<kr::KR_SPEC>;
///IWDG key register
pub mod kr;
///PR (rw) register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///IWDG prescaler register
pub mod pr;
///RLR (rw) register accessor: an alias for `Reg<RLR_SPEC>`
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
///IWDG reload register
pub mod rlr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///IWDG status register
pub mod sr;
///WINR (rw) register accessor: an alias for `Reg<WINR_SPEC>`
pub type WINR = crate::Reg<winr::WINR_SPEC>;
///IWDG window register
pub mod winr;
