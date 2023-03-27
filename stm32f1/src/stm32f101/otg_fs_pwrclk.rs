///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS power and clock gating control register
    pub pcgcctl: PCGCCTL,
}
///PCGCCTL (rw) register accessor: an alias for `Reg<PCGCCTL_SPEC>`
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
///OTG_FS power and clock gating control register
pub mod pcgcctl;
