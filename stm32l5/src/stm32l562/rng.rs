///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RNG control register
    pub cr: CR,
    ///0x04 - RNG status register
    pub sr: SR,
    ///0x08 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.
    pub dr: DR,
    _reserved3: [u8; 0x04],
    ///0x10 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.
    pub htcr: HTCR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///RNG control register
pub mod cr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///RNG status register
pub mod sr;
///DR (r) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.
pub mod dr;
///HTCR (rw) register accessor: an alias for `Reg<HTCR_SPEC>`
pub type HTCR = crate::Reg<htcr::HTCR_SPEC>;
///The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.
pub mod htcr;
