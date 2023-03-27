///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RNG control register
    pub rng_cr: RNG_CR,
    ///0x04 - RNG status register
    pub rng_sr: RNG_SR,
    ///0x08 - The RNG_DR register is a read-only register.
    pub rng_dr: RNG_DR,
    _reserved3: [u8; 0x03e4],
    ///0x3f0 - RNG hardware configuration register
    pub rng_hwcfgr: RNG_HWCFGR,
    ///0x3f4 - RNG version register
    pub rng_verr: RNG_VERR,
    ///0x3f8 - RNG identification register
    pub rng_ipidr: RNG_IPIDR,
    ///0x3fc - RNG size ID register
    pub rng_sidr: RNG_SIDR,
}
///RNG_CR (rw) register accessor: an alias for `Reg<RNG_CR_SPEC>`
pub type RNG_CR = crate::Reg<rng_cr::RNG_CR_SPEC>;
///RNG control register
pub mod rng_cr;
///RNG_SR (rw) register accessor: an alias for `Reg<RNG_SR_SPEC>`
pub type RNG_SR = crate::Reg<rng_sr::RNG_SR_SPEC>;
///RNG status register
pub mod rng_sr;
///RNG_DR (r) register accessor: an alias for `Reg<RNG_DR_SPEC>`
pub type RNG_DR = crate::Reg<rng_dr::RNG_DR_SPEC>;
///The RNG_DR register is a read-only register.
pub mod rng_dr;
///RNG_HWCFGR (r) register accessor: an alias for `Reg<RNG_HWCFGR_SPEC>`
pub type RNG_HWCFGR = crate::Reg<rng_hwcfgr::RNG_HWCFGR_SPEC>;
///RNG hardware configuration register
pub mod rng_hwcfgr;
///RNG_VERR (r) register accessor: an alias for `Reg<RNG_VERR_SPEC>`
pub type RNG_VERR = crate::Reg<rng_verr::RNG_VERR_SPEC>;
///RNG version register
pub mod rng_verr;
///RNG_IPIDR (r) register accessor: an alias for `Reg<RNG_IPIDR_SPEC>`
pub type RNG_IPIDR = crate::Reg<rng_ipidr::RNG_IPIDR_SPEC>;
///RNG identification register
pub mod rng_ipidr;
///RNG_SIDR (r) register accessor: an alias for `Reg<RNG_SIDR_SPEC>`
pub type RNG_SIDR = crate::Reg<rng_sidr::RNG_SIDR_SPEC>;
///RNG size ID register
pub mod rng_sidr;
