///Register block
#[repr(C)]
pub struct KEY {
    ///0x00 - key registers
    pub klr: KLR,
    ///0x04 - key registers
    pub krr: KRR,
}
///KLR (w) register accessor: an alias for `Reg<KLR_SPEC>`
pub type KLR = crate::Reg<klr::KLR_SPEC>;
///key registers
pub mod klr;
///KRR (w) register accessor: an alias for `Reg<KRR_SPEC>`
pub type KRR = crate::Reg<krr::KRR_SPEC>;
///key registers
pub mod krr;
