///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Power control register 1
    pub cr1: CR1,
    ///0x04 - Power control register 2
    pub cr2: CR2,
    ///0x08 - Power control register 3
    pub cr3: CR3,
    ///0x0c - Power control register 4
    pub cr4: CR4,
    ///0x10 - Power status register 1
    pub sr1: SR1,
    ///0x14 - Power status register 2
    pub sr2: SR2,
    ///0x18 - Power status clear register
    pub scr: SCR,
    ///0x1c - Power control register 5
    pub cr5: CR5,
    ///0x20 - Power Port A pull-up control register
    pub pucra: PUCRA,
    ///0x24 - Power Port A pull-down control register
    pub pdcra: PDCRA,
    ///0x28 - Power Port B pull-up control register
    pub pucrb: PUCRB,
    ///0x2c - Power Port B pull-down control register
    pub pdcrb: PDCRB,
    ///0x30 - Power Port C pull-up control register
    pub pucrc: PUCRC,
    ///0x34 - Power Port C pull-down control register
    pub pdcrc: PDCRC,
    _reserved14: [u8; 0x20],
    ///0x58 - Power Port H pull-up control register
    pub pucrh: PUCRH,
    ///0x5c - Power Port H pull-down control register
    pub pdcrh: PDCRH,
    _reserved16: [u8; 0x28],
    ///0x88 - Power extended status and status clear register
    pub extscr: EXTSCR,
    _reserved17: [u8; 0x04],
    ///0x90 - Power SPI3 control register
    pub subghzspicr: SUBGHZSPICR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Power control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Power control register 2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///Power control register 3
pub mod cr3;
///CR4 (rw) register accessor: an alias for `Reg<CR4_SPEC>`
pub type CR4 = crate::Reg<cr4::CR4_SPEC>;
///Power control register 4
pub mod cr4;
///SR1 (r) register accessor: an alias for `Reg<SR1_SPEC>`
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
///Power status register 1
pub mod sr1;
///SR2 (r) register accessor: an alias for `Reg<SR2_SPEC>`
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
///Power status register 2
pub mod sr2;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///Power status clear register
pub mod scr;
///CR5 (rw) register accessor: an alias for `Reg<CR5_SPEC>`
pub type CR5 = crate::Reg<cr5::CR5_SPEC>;
///Power control register 5
pub mod cr5;
///PUCRA (rw) register accessor: an alias for `Reg<PUCRA_SPEC>`
pub type PUCRA = crate::Reg<pucra::PUCRA_SPEC>;
///Power Port A pull-up control register
pub mod pucra;
///PDCRA (rw) register accessor: an alias for `Reg<PDCRA_SPEC>`
pub type PDCRA = crate::Reg<pdcra::PDCRA_SPEC>;
///Power Port A pull-down control register
pub mod pdcra;
///PUCRB (rw) register accessor: an alias for `Reg<PUCRB_SPEC>`
pub type PUCRB = crate::Reg<pucrb::PUCRB_SPEC>;
///Power Port B pull-up control register
pub mod pucrb;
///PDCRB (rw) register accessor: an alias for `Reg<PDCRB_SPEC>`
pub type PDCRB = crate::Reg<pdcrb::PDCRB_SPEC>;
///Power Port B pull-down control register
pub mod pdcrb;
///PUCRC (rw) register accessor: an alias for `Reg<PUCRC_SPEC>`
pub type PUCRC = crate::Reg<pucrc::PUCRC_SPEC>;
///Power Port C pull-up control register
pub mod pucrc;
///PDCRC (rw) register accessor: an alias for `Reg<PDCRC_SPEC>`
pub type PDCRC = crate::Reg<pdcrc::PDCRC_SPEC>;
///Power Port C pull-down control register
pub mod pdcrc;
///PUCRH (rw) register accessor: an alias for `Reg<PUCRH_SPEC>`
pub type PUCRH = crate::Reg<pucrh::PUCRH_SPEC>;
///Power Port H pull-up control register
pub mod pucrh;
///PDCRH (rw) register accessor: an alias for `Reg<PDCRH_SPEC>`
pub type PDCRH = crate::Reg<pdcrh::PDCRH_SPEC>;
///Power Port H pull-down control register
pub mod pdcrh;
///EXTSCR (rw) register accessor: an alias for `Reg<EXTSCR_SPEC>`
pub type EXTSCR = crate::Reg<extscr::EXTSCR_SPEC>;
///Power extended status and status clear register
pub mod extscr;
///SUBGHZSPICR (rw) register accessor: an alias for `Reg<SUBGHZSPICR_SPEC>`
pub type SUBGHZSPICR = crate::Reg<subghzspicr::SUBGHZSPICR_SPEC>;
///Power SPI3 control register
pub mod subghzspicr;
