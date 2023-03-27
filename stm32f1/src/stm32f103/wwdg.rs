///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register (WWDG_CR)
    pub cr: CR,
    ///0x04 - Configuration register (WWDG_CFR)
    pub cfr: CFR,
    ///0x08 - Status register (WWDG_SR)
    pub sr: SR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register (WWDG_CR)
pub mod cr;
///CFR (rw) register accessor: an alias for `Reg<CFR_SPEC>`
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
///Configuration register (WWDG_CFR)
pub mod cfr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register (WWDG_SR)
pub mod sr;
