///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TAMP control register 1
    pub cr1: CR1,
    ///0x04 - TAMP control register 2
    pub cr2: CR2,
    ///0x08 - TAMP control register 3
    pub cr3: CR3,
    ///0x0c - TAMP filter control register
    pub fltcr: FLTCR,
    ///0x10 - TAMP active tamper control register 1
    pub atcr1: ATCR1,
    ///0x14 - TAMP active tamper seed register
    pub atseedr: ATSEEDR,
    ///0x18 - TAMP active tamper output register
    pub ator: ATOR,
    ///0x1c - TAMP active tamper control register 2
    pub atcr2: ATCR2,
    ///0x20 - TAMP secure mode register
    pub seccfgr: SECCFGR,
    ///0x24 - TAMP privilege mode control register
    pub privcfgr: PRIVCFGR,
    _reserved10: [u8; 0x04],
    ///0x2c - TAMP interrupt enable register
    pub ier: IER,
    ///0x30 - TAMP status register
    pub sr: SR,
    ///0x34 - TAMP non-secure masked interrupt status register
    pub misr: MISR,
    ///0x38 - TAMP secure masked interrupt status register
    pub smisr: SMISR,
    ///0x3c - TAMP status clear register
    pub scr: SCR,
    ///0x40 - TAMP monotonic counter 1 register
    pub count1r: COUNT1R,
    _reserved16: [u8; 0x0c],
    ///0x50 - TAMP option register
    pub or: OR,
    ///0x54 - TAMP erase configuration register
    pub ercfgr: ERCFGR,
    _reserved18: [u8; 0xa8],
    ///0x100 - TAMP backup 0 register
    pub bkp0r: BKP0R,
    ///0x104 - TAMP backup 1 register
    pub bkp1r: BKP1R,
    ///0x108 - TAMP backup 2 register
    pub bkp2r: BKP2R,
    ///0x10c - TAMP backup 3 register
    pub bkp3r: BKP3R,
    ///0x110 - TAMP backup 4 register
    pub bkp4r: BKP4R,
    ///0x114 - TAMP backup 5 register
    pub bkp5r: BKP5R,
    ///0x118 - TAMP backup 6 register
    pub bkp6r: BKP6R,
    ///0x11c - TAMP backup 7 register
    pub bkp7r: BKP7R,
    ///0x120 - TAMP backup 8 register
    pub bkp8r: BKP8R,
    ///0x124 - TAMP backup 9 register
    pub bkp9r: BKP9R,
    ///0x128 - TAMP backup 10 register
    pub bkp10r: BKP10R,
    ///0x12c - TAMP backup 11 register
    pub bkp11r: BKP11R,
    ///0x130 - TAMP backup 12 register
    pub bkp12r: BKP12R,
    ///0x134 - TAMP backup 13 register
    pub bkp13r: BKP13R,
    ///0x138 - TAMP backup 14 register
    pub bkp14r: BKP14R,
    ///0x13c - TAMP backup 15 register
    pub bkp15r: BKP15R,
    ///0x140 - TAMP backup 16 register
    pub bkp16r: BKP16R,
    ///0x144 - TAMP backup 17 register
    pub bkp17r: BKP17R,
    ///0x148 - TAMP backup 18 register
    pub bkp18r: BKP18R,
    ///0x14c - TAMP backup 19 register
    pub bkp19r: BKP19R,
    ///0x150 - TAMP backup 20 register
    pub bkp20r: BKP20R,
    ///0x154 - TAMP backup 21 register
    pub bkp21r: BKP21R,
    ///0x158 - TAMP backup 22 register
    pub bkp22r: BKP22R,
    ///0x15c - TAMP backup 23 register
    pub bkp23r: BKP23R,
    ///0x160 - TAMP backup 24 register
    pub bkp24r: BKP24R,
    ///0x164 - TAMP backup 25 register
    pub bkp25r: BKP25R,
    ///0x168 - TAMP backup 26 register
    pub bkp26r: BKP26R,
    ///0x16c - TAMP backup 27 register
    pub bkp27r: BKP27R,
    ///0x170 - TAMP backup 28 register
    pub bkp28r: BKP28R,
    ///0x174 - TAMP backup 29 register
    pub bkp29r: BKP29R,
    ///0x178 - TAMP backup 30 register
    pub bkp30r: BKP30R,
    ///0x17c - TAMP backup 31 register
    pub bkp31r: BKP31R,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///TAMP control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///TAMP control register 2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///TAMP control register 3
pub mod cr3;
///FLTCR (rw) register accessor: an alias for `Reg<FLTCR_SPEC>`
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
///TAMP filter control register
pub mod fltcr;
///ATCR1 (rw) register accessor: an alias for `Reg<ATCR1_SPEC>`
pub type ATCR1 = crate::Reg<atcr1::ATCR1_SPEC>;
///TAMP active tamper control register 1
pub mod atcr1;
///ATSEEDR (w) register accessor: an alias for `Reg<ATSEEDR_SPEC>`
pub type ATSEEDR = crate::Reg<atseedr::ATSEEDR_SPEC>;
///TAMP active tamper seed register
pub mod atseedr;
///ATOR (r) register accessor: an alias for `Reg<ATOR_SPEC>`
pub type ATOR = crate::Reg<ator::ATOR_SPEC>;
///TAMP active tamper output register
pub mod ator;
///ATCR2 (rw) register accessor: an alias for `Reg<ATCR2_SPEC>`
pub type ATCR2 = crate::Reg<atcr2::ATCR2_SPEC>;
///TAMP active tamper control register 2
pub mod atcr2;
///SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
///TAMP secure mode register
pub mod seccfgr;
///PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
///TAMP privilege mode control register
pub mod privcfgr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///TAMP interrupt enable register
pub mod ier;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///TAMP status register
pub mod sr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///TAMP non-secure masked interrupt status register
pub mod misr;
///SMISR (r) register accessor: an alias for `Reg<SMISR_SPEC>`
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
///TAMP secure masked interrupt status register
pub mod smisr;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///TAMP status clear register
pub mod scr;
///COUNT1R (r) register accessor: an alias for `Reg<COUNT1R_SPEC>`
pub type COUNT1R = crate::Reg<count1r::COUNT1R_SPEC>;
///TAMP monotonic counter 1 register
pub mod count1r;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///TAMP option register
pub mod or;
///ERCFGR (rw) register accessor: an alias for `Reg<ERCFGR_SPEC>`
pub type ERCFGR = crate::Reg<ercfgr::ERCFGR_SPEC>;
///TAMP erase configuration register
pub mod ercfgr;
///BKP0R (rw) register accessor: an alias for `Reg<BKP0R_SPEC>`
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
///TAMP backup 0 register
pub mod bkp0r;
///BKP1R (rw) register accessor: an alias for `Reg<BKP1R_SPEC>`
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
///TAMP backup 1 register
pub mod bkp1r;
///BKP2R (rw) register accessor: an alias for `Reg<BKP2R_SPEC>`
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
///TAMP backup 2 register
pub mod bkp2r;
///BKP3R (rw) register accessor: an alias for `Reg<BKP3R_SPEC>`
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
///TAMP backup 3 register
pub mod bkp3r;
///BKP4R (rw) register accessor: an alias for `Reg<BKP4R_SPEC>`
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
///TAMP backup 4 register
pub mod bkp4r;
///BKP5R (rw) register accessor: an alias for `Reg<BKP5R_SPEC>`
pub type BKP5R = crate::Reg<bkp5r::BKP5R_SPEC>;
///TAMP backup 5 register
pub mod bkp5r;
///BKP6R (rw) register accessor: an alias for `Reg<BKP6R_SPEC>`
pub type BKP6R = crate::Reg<bkp6r::BKP6R_SPEC>;
///TAMP backup 6 register
pub mod bkp6r;
///BKP7R (rw) register accessor: an alias for `Reg<BKP7R_SPEC>`
pub type BKP7R = crate::Reg<bkp7r::BKP7R_SPEC>;
///TAMP backup 7 register
pub mod bkp7r;
///BKP8R (rw) register accessor: an alias for `Reg<BKP8R_SPEC>`
pub type BKP8R = crate::Reg<bkp8r::BKP8R_SPEC>;
///TAMP backup 8 register
pub mod bkp8r;
///BKP9R (rw) register accessor: an alias for `Reg<BKP9R_SPEC>`
pub type BKP9R = crate::Reg<bkp9r::BKP9R_SPEC>;
///TAMP backup 9 register
pub mod bkp9r;
///BKP10R (rw) register accessor: an alias for `Reg<BKP10R_SPEC>`
pub type BKP10R = crate::Reg<bkp10r::BKP10R_SPEC>;
///TAMP backup 10 register
pub mod bkp10r;
///BKP11R (rw) register accessor: an alias for `Reg<BKP11R_SPEC>`
pub type BKP11R = crate::Reg<bkp11r::BKP11R_SPEC>;
///TAMP backup 11 register
pub mod bkp11r;
///BKP12R (rw) register accessor: an alias for `Reg<BKP12R_SPEC>`
pub type BKP12R = crate::Reg<bkp12r::BKP12R_SPEC>;
///TAMP backup 12 register
pub mod bkp12r;
///BKP13R (rw) register accessor: an alias for `Reg<BKP13R_SPEC>`
pub type BKP13R = crate::Reg<bkp13r::BKP13R_SPEC>;
///TAMP backup 13 register
pub mod bkp13r;
///BKP14R (rw) register accessor: an alias for `Reg<BKP14R_SPEC>`
pub type BKP14R = crate::Reg<bkp14r::BKP14R_SPEC>;
///TAMP backup 14 register
pub mod bkp14r;
///BKP15R (rw) register accessor: an alias for `Reg<BKP15R_SPEC>`
pub type BKP15R = crate::Reg<bkp15r::BKP15R_SPEC>;
///TAMP backup 15 register
pub mod bkp15r;
///BKP16R (rw) register accessor: an alias for `Reg<BKP16R_SPEC>`
pub type BKP16R = crate::Reg<bkp16r::BKP16R_SPEC>;
///TAMP backup 16 register
pub mod bkp16r;
///BKP17R (rw) register accessor: an alias for `Reg<BKP17R_SPEC>`
pub type BKP17R = crate::Reg<bkp17r::BKP17R_SPEC>;
///TAMP backup 17 register
pub mod bkp17r;
///BKP18R (rw) register accessor: an alias for `Reg<BKP18R_SPEC>`
pub type BKP18R = crate::Reg<bkp18r::BKP18R_SPEC>;
///TAMP backup 18 register
pub mod bkp18r;
///BKP19R (rw) register accessor: an alias for `Reg<BKP19R_SPEC>`
pub type BKP19R = crate::Reg<bkp19r::BKP19R_SPEC>;
///TAMP backup 19 register
pub mod bkp19r;
///BKP20R (rw) register accessor: an alias for `Reg<BKP20R_SPEC>`
pub type BKP20R = crate::Reg<bkp20r::BKP20R_SPEC>;
///TAMP backup 20 register
pub mod bkp20r;
///BKP21R (rw) register accessor: an alias for `Reg<BKP21R_SPEC>`
pub type BKP21R = crate::Reg<bkp21r::BKP21R_SPEC>;
///TAMP backup 21 register
pub mod bkp21r;
///BKP22R (rw) register accessor: an alias for `Reg<BKP22R_SPEC>`
pub type BKP22R = crate::Reg<bkp22r::BKP22R_SPEC>;
///TAMP backup 22 register
pub mod bkp22r;
///BKP23R (rw) register accessor: an alias for `Reg<BKP23R_SPEC>`
pub type BKP23R = crate::Reg<bkp23r::BKP23R_SPEC>;
///TAMP backup 23 register
pub mod bkp23r;
///BKP24R (rw) register accessor: an alias for `Reg<BKP24R_SPEC>`
pub type BKP24R = crate::Reg<bkp24r::BKP24R_SPEC>;
///TAMP backup 24 register
pub mod bkp24r;
///BKP25R (rw) register accessor: an alias for `Reg<BKP25R_SPEC>`
pub type BKP25R = crate::Reg<bkp25r::BKP25R_SPEC>;
///TAMP backup 25 register
pub mod bkp25r;
///BKP26R (rw) register accessor: an alias for `Reg<BKP26R_SPEC>`
pub type BKP26R = crate::Reg<bkp26r::BKP26R_SPEC>;
///TAMP backup 26 register
pub mod bkp26r;
///BKP27R (rw) register accessor: an alias for `Reg<BKP27R_SPEC>`
pub type BKP27R = crate::Reg<bkp27r::BKP27R_SPEC>;
///TAMP backup 27 register
pub mod bkp27r;
///BKP28R (rw) register accessor: an alias for `Reg<BKP28R_SPEC>`
pub type BKP28R = crate::Reg<bkp28r::BKP28R_SPEC>;
///TAMP backup 28 register
pub mod bkp28r;
///BKP29R (rw) register accessor: an alias for `Reg<BKP29R_SPEC>`
pub type BKP29R = crate::Reg<bkp29r::BKP29R_SPEC>;
///TAMP backup 29 register
pub mod bkp29r;
///BKP30R (rw) register accessor: an alias for `Reg<BKP30R_SPEC>`
pub type BKP30R = crate::Reg<bkp30r::BKP30R_SPEC>;
///TAMP backup 30 register
pub mod bkp30r;
///BKP31R (rw) register accessor: an alias for `Reg<BKP31R_SPEC>`
pub type BKP31R = crate::Reg<bkp31r::BKP31R_SPEC>;
///TAMP backup 31 register
pub mod bkp31r;
