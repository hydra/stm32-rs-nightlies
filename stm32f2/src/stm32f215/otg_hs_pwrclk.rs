///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Power and clock gating control register
    pub pcgcctl: PCGCCTL,
}
///PCGCCTL (rw) register accessor: an alias for `Reg<PCGCCTL_SPEC>`
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
///Power and clock gating control register
pub mod pcgcctl;
