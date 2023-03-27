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
    ///0x14 - IWDG early wakeup interrupt register
    pub ewcr: EWCR,
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
///EWCR (rw) register accessor: an alias for `Reg<EWCR_SPEC>`
pub type EWCR = crate::Reg<ewcr::EWCR_SPEC>;
///IWDG early wakeup interrupt register
pub mod ewcr;
