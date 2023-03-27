///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - PWR control register 1
    pub cr1: CR1,
    _reserved1: [u8; 0x04],
    ///0x08 - PWR control register 3
    pub cr3: CR3,
    ///0x0c - PWR control register 4
    pub cr4: CR4,
    ///0x10 - PWR status register 1
    pub sr1: SR1,
    ///0x14 - PWR status register 2
    pub sr2: SR2,
    ///0x18 - PWR status clear register
    pub scr: SCR,
    _reserved6: [u8; 0x04],
    ///0x20 - PWR Port A pull-up control register
    pub pucra: PUCRA,
    ///0x24 - PWR Port A pull-down control register
    pub pdcra: PDCRA,
    ///0x28 - PWR Port B pull-up control register
    pub pucrb: PUCRB,
    ///0x2c - PWR Port B pull-down control register
    pub pdcrb: PDCRB,
    ///0x30 - PWR Port C pull-up control register
    pub pucrc: PUCRC,
    ///0x34 - PWR Port C pull-down control register
    pub pdcrc: PDCRC,
    ///0x38 - PWR Port D pull-up control register
    pub pucrd: PUCRD,
    ///0x3c - PWR Port D pull-down control register
    pub pdcrd: PDCRD,
    _reserved14: [u8; 0x08],
    ///0x48 - PWR Port F pull-up control register
    pub pucrf: PUCRF,
    ///0x4c - PWR Port F pull-down control register
    pub pdcrf: PDCRF,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///PWR control register 1
pub mod cr1;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///PWR control register 3
pub mod cr3;
///CR4 (rw) register accessor: an alias for `Reg<CR4_SPEC>`
pub type CR4 = crate::Reg<cr4::CR4_SPEC>;
///PWR control register 4
pub mod cr4;
///SR1 (r) register accessor: an alias for `Reg<SR1_SPEC>`
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
///PWR status register 1
pub mod sr1;
///SR2 (r) register accessor: an alias for `Reg<SR2_SPEC>`
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
///PWR status register 2
pub mod sr2;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///PWR status clear register
pub mod scr;
///PUCRA (rw) register accessor: an alias for `Reg<PUCRA_SPEC>`
pub type PUCRA = crate::Reg<pucra::PUCRA_SPEC>;
///PWR Port A pull-up control register
pub mod pucra;
///PDCRA (rw) register accessor: an alias for `Reg<PDCRA_SPEC>`
pub type PDCRA = crate::Reg<pdcra::PDCRA_SPEC>;
///PWR Port A pull-down control register
pub mod pdcra;
///PUCRB (rw) register accessor: an alias for `Reg<PUCRB_SPEC>`
pub type PUCRB = crate::Reg<pucrb::PUCRB_SPEC>;
///PWR Port B pull-up control register
pub mod pucrb;
///PDCRB (rw) register accessor: an alias for `Reg<PDCRB_SPEC>`
pub type PDCRB = crate::Reg<pdcrb::PDCRB_SPEC>;
///PWR Port B pull-down control register
pub mod pdcrb;
///PUCRC (rw) register accessor: an alias for `Reg<PUCRC_SPEC>`
pub type PUCRC = crate::Reg<pucrc::PUCRC_SPEC>;
///PWR Port C pull-up control register
pub mod pucrc;
///PDCRC (rw) register accessor: an alias for `Reg<PDCRC_SPEC>`
pub type PDCRC = crate::Reg<pdcrc::PDCRC_SPEC>;
///PWR Port C pull-down control register
pub mod pdcrc;
///PUCRD (rw) register accessor: an alias for `Reg<PUCRD_SPEC>`
pub type PUCRD = crate::Reg<pucrd::PUCRD_SPEC>;
///PWR Port D pull-up control register
pub mod pucrd;
///PDCRD (rw) register accessor: an alias for `Reg<PDCRD_SPEC>`
pub type PDCRD = crate::Reg<pdcrd::PDCRD_SPEC>;
///PWR Port D pull-down control register
pub mod pdcrd;
///PUCRF (rw) register accessor: an alias for `Reg<PUCRF_SPEC>`
pub type PUCRF = crate::Reg<pucrf::PUCRF_SPEC>;
///PWR Port F pull-up control register
pub mod pucrf;
///PDCRF (rw) register accessor: an alias for `Reg<PDCRF_SPEC>`
pub type PDCRF = crate::Reg<pdcrf::PDCRF_SPEC>;
///PWR Port F pull-down control register
pub mod pdcrf;
