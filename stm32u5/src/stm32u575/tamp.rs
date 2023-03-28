///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TAMP control register 1
    pub tamp_cr1: TAMP_CR1,
    ///0x04 - TAMP control register 2
    pub tamp_cr2: TAMP_CR2,
    ///0x08 - TAMP control register 3
    pub tamp_cr3: TAMP_CR3,
    ///0x0c - TAMP filter control register
    pub tamp_fltcr: TAMP_FLTCR,
    ///0x10 - TAMP active tamper control register 1
    pub tamp_atcr1: TAMP_ATCR1,
    ///0x14 - TAMP active tamper seed register
    pub tamp_atseedr: TAMP_ATSEEDR,
    ///0x18 - TAMP active tamper output register
    pub tamp_ator: TAMP_ATOR,
    ///0x1c - TAMP active tamper control register 2
    pub tamp_atcr2: TAMP_ATCR2,
    ///0x20 - TAMP secure mode register
    pub tamp_seccfgr: TAMP_SECCFGR,
    ///0x24 - TAMP privilege mode control register
    pub tamp_privcr: TAMP_PRIVCR,
    _reserved10: [u8; 0x04],
    ///0x2c - TAMP interrupt enable register
    pub tamp_ier: TAMP_IER,
    ///0x30 - TAMP status register
    pub tamp_sr: TAMP_SR,
    ///0x34 - TAMP non-secure masked interrupt status register
    pub tamp_misr: TAMP_MISR,
    ///0x38 - TAMP secure masked interrupt status register
    pub tamp_smisr: TAMP_SMISR,
    ///0x3c - TAMP status clear register
    pub tamp_scr: TAMP_SCR,
    ///0x40 - TAMP monotonic counter 1 register
    pub tamp_count1r: TAMP_COUNT1R,
    _reserved16: [u8; 0x10],
    ///0x54 - TAMP erase configuration register
    pub tamp_ercfgr: TAMP_ERCFGR,
    _reserved17: [u8; 0xa8],
    ///0x100 - TAMP backup 0 register
    pub tamp_bkp0r: TAMP_BKP0R,
    ///0x104 - TAMP backup 1 register
    pub tamp_bkp1r: TAMP_BKP1R,
    ///0x108 - TAMP backup 2 register
    pub tamp_bkp2r: TAMP_BKP2R,
    ///0x10c - TAMP backup 3 register
    pub tamp_bkp3r: TAMP_BKP3R,
    ///0x110 - TAMP backup 4 register
    pub tamp_bkp4r: TAMP_BKP4R,
    ///0x114 - TAMP backup 5 register
    pub tamp_bkp5r: TAMP_BKP5R,
    ///0x118 - TAMP backup 6 register
    pub tamp_bkp6r: TAMP_BKP6R,
    ///0x11c - TAMP backup 7 register
    pub tamp_bkp7r: TAMP_BKP7R,
    ///0x120 - TAMP backup 8 register
    pub tamp_bkp8r: TAMP_BKP8R,
    ///0x124 - TAMP backup 9 register
    pub tamp_bkp9r: TAMP_BKP9R,
    ///0x128 - TAMP backup 10 register
    pub tamp_bkp10r: TAMP_BKP10R,
    ///0x12c - TAMP backup 11 register
    pub tamp_bkp11r: TAMP_BKP11R,
    ///0x130 - TAMP backup 12 register
    pub tamp_bkp12r: TAMP_BKP12R,
    ///0x134 - TAMP backup 13 register
    pub tamp_bkp13r: TAMP_BKP13R,
    ///0x138 - TAMP backup 14 register
    pub tamp_bkp14r: TAMP_BKP14R,
    ///0x13c - TAMP backup 15 register
    pub tamp_bkp15r: TAMP_BKP15R,
    ///0x140 - TAMP backup 16 register
    pub tamp_bkp16r: TAMP_BKP16R,
    ///0x144 - TAMP backup 17 register
    pub tamp_bkp17r: TAMP_BKP17R,
    ///0x148 - TAMP backup 18 register
    pub tamp_bkp18r: TAMP_BKP18R,
    ///0x14c - TAMP backup 19 register
    pub tamp_bkp19r: TAMP_BKP19R,
    ///0x150 - TAMP backup 20 register
    pub tamp_bkp20r: TAMP_BKP20R,
    ///0x154 - TAMP backup 21 register
    pub tamp_bkp21r: TAMP_BKP21R,
    ///0x158 - TAMP backup 22 register
    pub tamp_bkp22r: TAMP_BKP22R,
    ///0x15c - TAMP backup 23 register
    pub tamp_bkp23r: TAMP_BKP23R,
    ///0x160 - TAMP backup 24 register
    pub tamp_bkp24r: TAMP_BKP24R,
    ///0x164 - TAMP backup 25 register
    pub tamp_bkp25r: TAMP_BKP25R,
    ///0x168 - TAMP backup 26 register
    pub tamp_bkp26r: TAMP_BKP26R,
    ///0x16c - TAMP backup 27 register
    pub tamp_bkp27r: TAMP_BKP27R,
    ///0x170 - TAMP backup 28 register
    pub tamp_bkp28r: TAMP_BKP28R,
    ///0x174 - TAMP backup 29 register
    pub tamp_bkp29r: TAMP_BKP29R,
    ///0x178 - TAMP backup 30 register
    pub tamp_bkp30r: TAMP_BKP30R,
    ///0x17c - TAMP backup 31 register
    pub tamp_bkp31r: TAMP_BKP31R,
}
///TAMP_CR1 (rw) register accessor: an alias for `Reg<TAMP_CR1_SPEC>`
pub type TAMP_CR1 = crate::Reg<tamp_cr1::TAMP_CR1_SPEC>;
///TAMP control register 1
pub mod tamp_cr1;
///TAMP_CR2 (rw) register accessor: an alias for `Reg<TAMP_CR2_SPEC>`
pub type TAMP_CR2 = crate::Reg<tamp_cr2::TAMP_CR2_SPEC>;
///TAMP control register 2
pub mod tamp_cr2;
///TAMP_CR3 (rw) register accessor: an alias for `Reg<TAMP_CR3_SPEC>`
pub type TAMP_CR3 = crate::Reg<tamp_cr3::TAMP_CR3_SPEC>;
///TAMP control register 3
pub mod tamp_cr3;
///TAMP_FLTCR (rw) register accessor: an alias for `Reg<TAMP_FLTCR_SPEC>`
pub type TAMP_FLTCR = crate::Reg<tamp_fltcr::TAMP_FLTCR_SPEC>;
///TAMP filter control register
pub mod tamp_fltcr;
///TAMP_ATCR1 (rw) register accessor: an alias for `Reg<TAMP_ATCR1_SPEC>`
pub type TAMP_ATCR1 = crate::Reg<tamp_atcr1::TAMP_ATCR1_SPEC>;
///TAMP active tamper control register 1
pub mod tamp_atcr1;
///TAMP_ATSEEDR (w) register accessor: an alias for `Reg<TAMP_ATSEEDR_SPEC>`
pub type TAMP_ATSEEDR = crate::Reg<tamp_atseedr::TAMP_ATSEEDR_SPEC>;
///TAMP active tamper seed register
pub mod tamp_atseedr;
///TAMP_ATOR (r) register accessor: an alias for `Reg<TAMP_ATOR_SPEC>`
pub type TAMP_ATOR = crate::Reg<tamp_ator::TAMP_ATOR_SPEC>;
///TAMP active tamper output register
pub mod tamp_ator;
///TAMP_ATCR2 (rw) register accessor: an alias for `Reg<TAMP_ATCR2_SPEC>`
pub type TAMP_ATCR2 = crate::Reg<tamp_atcr2::TAMP_ATCR2_SPEC>;
///TAMP active tamper control register 2
pub mod tamp_atcr2;
///TAMP_SECCFGR (rw) register accessor: an alias for `Reg<TAMP_SECCFGR_SPEC>`
pub type TAMP_SECCFGR = crate::Reg<tamp_seccfgr::TAMP_SECCFGR_SPEC>;
///TAMP secure mode register
pub mod tamp_seccfgr;
///TAMP_PRIVCR (rw) register accessor: an alias for `Reg<TAMP_PRIVCR_SPEC>`
pub type TAMP_PRIVCR = crate::Reg<tamp_privcr::TAMP_PRIVCR_SPEC>;
///TAMP privilege mode control register
pub mod tamp_privcr;
///TAMP_IER (rw) register accessor: an alias for `Reg<TAMP_IER_SPEC>`
pub type TAMP_IER = crate::Reg<tamp_ier::TAMP_IER_SPEC>;
///TAMP interrupt enable register
pub mod tamp_ier;
///TAMP_SR (r) register accessor: an alias for `Reg<TAMP_SR_SPEC>`
pub type TAMP_SR = crate::Reg<tamp_sr::TAMP_SR_SPEC>;
///TAMP status register
pub mod tamp_sr;
///TAMP_MISR (r) register accessor: an alias for `Reg<TAMP_MISR_SPEC>`
pub type TAMP_MISR = crate::Reg<tamp_misr::TAMP_MISR_SPEC>;
///TAMP non-secure masked interrupt status register
pub mod tamp_misr;
///TAMP_SMISR (r) register accessor: an alias for `Reg<TAMP_SMISR_SPEC>`
pub type TAMP_SMISR = crate::Reg<tamp_smisr::TAMP_SMISR_SPEC>;
///TAMP secure masked interrupt status register
pub mod tamp_smisr;
///TAMP_SCR (w) register accessor: an alias for `Reg<TAMP_SCR_SPEC>`
pub type TAMP_SCR = crate::Reg<tamp_scr::TAMP_SCR_SPEC>;
///TAMP status clear register
pub mod tamp_scr;
///TAMP_COUNT1R (r) register accessor: an alias for `Reg<TAMP_COUNT1R_SPEC>`
pub type TAMP_COUNT1R = crate::Reg<tamp_count1r::TAMP_COUNT1R_SPEC>;
///TAMP monotonic counter 1 register
pub mod tamp_count1r;
///TAMP_ERCFGR (rw) register accessor: an alias for `Reg<TAMP_ERCFGR_SPEC>`
pub type TAMP_ERCFGR = crate::Reg<tamp_ercfgr::TAMP_ERCFGR_SPEC>;
///TAMP erase configuration register
pub mod tamp_ercfgr;
///TAMP_BKP0R (rw) register accessor: an alias for `Reg<TAMP_BKP0R_SPEC>`
pub type TAMP_BKP0R = crate::Reg<tamp_bkp0r::TAMP_BKP0R_SPEC>;
///TAMP backup 0 register
pub mod tamp_bkp0r;
///TAMP_BKP1R (rw) register accessor: an alias for `Reg<TAMP_BKP1R_SPEC>`
pub type TAMP_BKP1R = crate::Reg<tamp_bkp1r::TAMP_BKP1R_SPEC>;
///TAMP backup 1 register
pub mod tamp_bkp1r;
///TAMP_BKP2R (rw) register accessor: an alias for `Reg<TAMP_BKP2R_SPEC>`
pub type TAMP_BKP2R = crate::Reg<tamp_bkp2r::TAMP_BKP2R_SPEC>;
///TAMP backup 2 register
pub mod tamp_bkp2r;
///TAMP_BKP3R (rw) register accessor: an alias for `Reg<TAMP_BKP3R_SPEC>`
pub type TAMP_BKP3R = crate::Reg<tamp_bkp3r::TAMP_BKP3R_SPEC>;
///TAMP backup 3 register
pub mod tamp_bkp3r;
///TAMP_BKP4R (rw) register accessor: an alias for `Reg<TAMP_BKP4R_SPEC>`
pub type TAMP_BKP4R = crate::Reg<tamp_bkp4r::TAMP_BKP4R_SPEC>;
///TAMP backup 4 register
pub mod tamp_bkp4r;
///TAMP_BKP5R (rw) register accessor: an alias for `Reg<TAMP_BKP5R_SPEC>`
pub type TAMP_BKP5R = crate::Reg<tamp_bkp5r::TAMP_BKP5R_SPEC>;
///TAMP backup 5 register
pub mod tamp_bkp5r;
///TAMP_BKP6R (rw) register accessor: an alias for `Reg<TAMP_BKP6R_SPEC>`
pub type TAMP_BKP6R = crate::Reg<tamp_bkp6r::TAMP_BKP6R_SPEC>;
///TAMP backup 6 register
pub mod tamp_bkp6r;
///TAMP_BKP7R (rw) register accessor: an alias for `Reg<TAMP_BKP7R_SPEC>`
pub type TAMP_BKP7R = crate::Reg<tamp_bkp7r::TAMP_BKP7R_SPEC>;
///TAMP backup 7 register
pub mod tamp_bkp7r;
///TAMP_BKP8R (rw) register accessor: an alias for `Reg<TAMP_BKP8R_SPEC>`
pub type TAMP_BKP8R = crate::Reg<tamp_bkp8r::TAMP_BKP8R_SPEC>;
///TAMP backup 8 register
pub mod tamp_bkp8r;
///TAMP_BKP9R (rw) register accessor: an alias for `Reg<TAMP_BKP9R_SPEC>`
pub type TAMP_BKP9R = crate::Reg<tamp_bkp9r::TAMP_BKP9R_SPEC>;
///TAMP backup 9 register
pub mod tamp_bkp9r;
///TAMP_BKP10R (rw) register accessor: an alias for `Reg<TAMP_BKP10R_SPEC>`
pub type TAMP_BKP10R = crate::Reg<tamp_bkp10r::TAMP_BKP10R_SPEC>;
///TAMP backup 10 register
pub mod tamp_bkp10r;
///TAMP_BKP11R (rw) register accessor: an alias for `Reg<TAMP_BKP11R_SPEC>`
pub type TAMP_BKP11R = crate::Reg<tamp_bkp11r::TAMP_BKP11R_SPEC>;
///TAMP backup 11 register
pub mod tamp_bkp11r;
///TAMP_BKP12R (rw) register accessor: an alias for `Reg<TAMP_BKP12R_SPEC>`
pub type TAMP_BKP12R = crate::Reg<tamp_bkp12r::TAMP_BKP12R_SPEC>;
///TAMP backup 12 register
pub mod tamp_bkp12r;
///TAMP_BKP13R (rw) register accessor: an alias for `Reg<TAMP_BKP13R_SPEC>`
pub type TAMP_BKP13R = crate::Reg<tamp_bkp13r::TAMP_BKP13R_SPEC>;
///TAMP backup 13 register
pub mod tamp_bkp13r;
///TAMP_BKP14R (rw) register accessor: an alias for `Reg<TAMP_BKP14R_SPEC>`
pub type TAMP_BKP14R = crate::Reg<tamp_bkp14r::TAMP_BKP14R_SPEC>;
///TAMP backup 14 register
pub mod tamp_bkp14r;
///TAMP_BKP15R (rw) register accessor: an alias for `Reg<TAMP_BKP15R_SPEC>`
pub type TAMP_BKP15R = crate::Reg<tamp_bkp15r::TAMP_BKP15R_SPEC>;
///TAMP backup 15 register
pub mod tamp_bkp15r;
///TAMP_BKP16R (rw) register accessor: an alias for `Reg<TAMP_BKP16R_SPEC>`
pub type TAMP_BKP16R = crate::Reg<tamp_bkp16r::TAMP_BKP16R_SPEC>;
///TAMP backup 16 register
pub mod tamp_bkp16r;
///TAMP_BKP17R (rw) register accessor: an alias for `Reg<TAMP_BKP17R_SPEC>`
pub type TAMP_BKP17R = crate::Reg<tamp_bkp17r::TAMP_BKP17R_SPEC>;
///TAMP backup 17 register
pub mod tamp_bkp17r;
///TAMP_BKP18R (rw) register accessor: an alias for `Reg<TAMP_BKP18R_SPEC>`
pub type TAMP_BKP18R = crate::Reg<tamp_bkp18r::TAMP_BKP18R_SPEC>;
///TAMP backup 18 register
pub mod tamp_bkp18r;
///TAMP_BKP19R (rw) register accessor: an alias for `Reg<TAMP_BKP19R_SPEC>`
pub type TAMP_BKP19R = crate::Reg<tamp_bkp19r::TAMP_BKP19R_SPEC>;
///TAMP backup 19 register
pub mod tamp_bkp19r;
///TAMP_BKP20R (rw) register accessor: an alias for `Reg<TAMP_BKP20R_SPEC>`
pub type TAMP_BKP20R = crate::Reg<tamp_bkp20r::TAMP_BKP20R_SPEC>;
///TAMP backup 20 register
pub mod tamp_bkp20r;
///TAMP_BKP21R (rw) register accessor: an alias for `Reg<TAMP_BKP21R_SPEC>`
pub type TAMP_BKP21R = crate::Reg<tamp_bkp21r::TAMP_BKP21R_SPEC>;
///TAMP backup 21 register
pub mod tamp_bkp21r;
///TAMP_BKP22R (rw) register accessor: an alias for `Reg<TAMP_BKP22R_SPEC>`
pub type TAMP_BKP22R = crate::Reg<tamp_bkp22r::TAMP_BKP22R_SPEC>;
///TAMP backup 22 register
pub mod tamp_bkp22r;
///TAMP_BKP23R (rw) register accessor: an alias for `Reg<TAMP_BKP23R_SPEC>`
pub type TAMP_BKP23R = crate::Reg<tamp_bkp23r::TAMP_BKP23R_SPEC>;
///TAMP backup 23 register
pub mod tamp_bkp23r;
///TAMP_BKP24R (rw) register accessor: an alias for `Reg<TAMP_BKP24R_SPEC>`
pub type TAMP_BKP24R = crate::Reg<tamp_bkp24r::TAMP_BKP24R_SPEC>;
///TAMP backup 24 register
pub mod tamp_bkp24r;
///TAMP_BKP25R (rw) register accessor: an alias for `Reg<TAMP_BKP25R_SPEC>`
pub type TAMP_BKP25R = crate::Reg<tamp_bkp25r::TAMP_BKP25R_SPEC>;
///TAMP backup 25 register
pub mod tamp_bkp25r;
///TAMP_BKP26R (rw) register accessor: an alias for `Reg<TAMP_BKP26R_SPEC>`
pub type TAMP_BKP26R = crate::Reg<tamp_bkp26r::TAMP_BKP26R_SPEC>;
///TAMP backup 26 register
pub mod tamp_bkp26r;
///TAMP_BKP27R (rw) register accessor: an alias for `Reg<TAMP_BKP27R_SPEC>`
pub type TAMP_BKP27R = crate::Reg<tamp_bkp27r::TAMP_BKP27R_SPEC>;
///TAMP backup 27 register
pub mod tamp_bkp27r;
///TAMP_BKP28R (rw) register accessor: an alias for `Reg<TAMP_BKP28R_SPEC>`
pub type TAMP_BKP28R = crate::Reg<tamp_bkp28r::TAMP_BKP28R_SPEC>;
///TAMP backup 28 register
pub mod tamp_bkp28r;
///TAMP_BKP29R (rw) register accessor: an alias for `Reg<TAMP_BKP29R_SPEC>`
pub type TAMP_BKP29R = crate::Reg<tamp_bkp29r::TAMP_BKP29R_SPEC>;
///TAMP backup 29 register
pub mod tamp_bkp29r;
///TAMP_BKP30R (rw) register accessor: an alias for `Reg<TAMP_BKP30R_SPEC>`
pub type TAMP_BKP30R = crate::Reg<tamp_bkp30r::TAMP_BKP30R_SPEC>;
///TAMP backup 30 register
pub mod tamp_bkp30r;
///TAMP_BKP31R (rw) register accessor: an alias for `Reg<TAMP_BKP31R_SPEC>`
pub type TAMP_BKP31R = crate::Reg<tamp_bkp31r::TAMP_BKP31R_SPEC>;
///TAMP backup 31 register
pub mod tamp_bkp31r;
