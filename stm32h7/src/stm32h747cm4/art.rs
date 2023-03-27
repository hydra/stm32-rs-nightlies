///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub ctr: CTR,
}
///CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
///control register
pub mod ctr;
