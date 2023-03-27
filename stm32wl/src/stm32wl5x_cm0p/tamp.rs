///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - TAMP control register 3
    pub cr3: CR3,
    ///0x0c - TAMP filter control register
    pub fltcr: FLTCR,
    _reserved4: [u8; 0x1c],
    ///0x2c - TAMP interrupt enable register
    pub ier: IER,
    ///0x30 - TAMP status register
    pub sr: SR,
    ///0x34 - TAMP masked interrupt status register
    pub misr: MISR,
    _reserved7: [u8; 0x04],
    ///0x3c - TAMP status clear register
    pub scr: SCR,
    ///0x40 - monotonic counter register
    pub countr: COUNTR,
    _reserved9: [u8; 0xbc],
    ///0x100..0x150 - TAMP backup register
    pub bkpr: [BKPR; 20],
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///TAMP control register 3
pub mod cr3;
///FLTCR (rw) register accessor: an alias for `Reg<FLTCR_SPEC>`
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
///TAMP filter control register
pub mod fltcr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///TAMP interrupt enable register
pub mod ier;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///TAMP status register
pub mod sr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///TAMP masked interrupt status register
pub mod misr;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///TAMP status clear register
pub mod scr;
///COUNTR (r) register accessor: an alias for `Reg<COUNTR_SPEC>`
pub type COUNTR = crate::Reg<countr::COUNTR_SPEC>;
///monotonic counter register
pub mod countr;
///BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
///TAMP backup register
pub mod bkpr;
