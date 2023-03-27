///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Key register (IWDG_KR)
    pub kr: KR,
    ///0x04 - Prescaler register (IWDG_PR)
    pub pr: PR,
    ///0x08 - Reload register (IWDG_RLR)
    pub rlr: RLR,
    ///0x0c - Status register (IWDG_SR)
    pub sr: SR,
}
///KR (w) register accessor: an alias for `Reg<KR_SPEC>`
pub type KR = crate::Reg<kr::KR_SPEC>;
///Key register (IWDG_KR)
pub mod kr;
///PR (rw) register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///Prescaler register (IWDG_PR)
pub mod pr;
///RLR (rw) register accessor: an alias for `Reg<RLR_SPEC>`
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
///Reload register (IWDG_RLR)
pub mod rlr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register (IWDG_SR)
pub mod sr;
