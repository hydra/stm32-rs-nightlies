///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Power and clock gating control register
    pub pcgcr: PCGCR,
}
///PCGCR (rw) register accessor: an alias for `Reg<PCGCR_SPEC>`
pub type PCGCR = crate::Reg<pcgcr::PCGCR_SPEC>;
///Power and clock gating control register
pub mod pcgcr;
