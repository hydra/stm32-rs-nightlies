///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - WWDG control register
    pub cr: CR,
    ///0x04 - WWDG configuration register
    pub cfr: CFR,
    ///0x08 - WWDG status register
    pub sr: SR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///WWDG control register
pub mod cr;
///CFR (rw) register accessor: an alias for `Reg<CFR_SPEC>`
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
///WWDG configuration register
pub mod cfr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///WWDG status register
pub mod sr;
