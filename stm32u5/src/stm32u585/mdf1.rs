///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MDF global control register
    pub gcr: GCR,
    ///0x04 - MDF clock generator control register
    pub ckgcr: CKGCR,
    _reserved2: [u8; 0x78],
    ///0x80 - This register is used to control the serial interfaces (SITFx).
    pub mdf_sitf0cr: MDF_SITF0CR,
    ///0x84 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    pub mdf_bsmx0cr: MDF_BSMX0CR,
    ///0x88 - This register is used to control the digital filter x.
    pub mdf_dflt0cr: MDF_DFLT0CR,
    ///0x8c - This register is used to control the main CIC filter.
    pub mdf_dflt0cicr: MDF_DFLT0CICR,
    ///0x90 - This register is used to control the reshape and HPF filters.
    pub mdf_dflt0rsfr: MDF_DFLT0RSFR,
    ///0x94 - This register is used to the integrator (INT) settings.
    pub mdf_dflt0intr: MDF_DFLT0INTR,
    ///0x98 - This register is used to configure the Out-of Limit Detector function.
    pub mdf_old0cr: MDF_OLD0CR,
    ///0x9c - This register is used for the adjustment of the Out-off Limit low threshold.
    pub mdf_old0thlr: MDF_OLD0THLR,
    ///0xa0 - This register is used for the adjustment of the Out-off Limit high threshold.
    pub mdf_old0thhr: MDF_OLD0THHR,
    ///0xa4 - This register is used for the adjustment stream delays.
    pub mdf_dly0cr: MDF_DLY0CR,
    ///0xa8 - This register is used for the adjustment stream delays.
    pub mdf_scd0cr: MDF_SCD0CR,
    ///0xac - This register is used for allowing or not the events to generate an interrupt.
    pub mdf_dflt0ier: MDF_DFLT0IER,
    ///0xb0 - MDF DFLT0 interrupt status register 0
    pub mdf_dflt0isr: MDF_DFLT0ISR,
    ///0xb4 - This register contains the offset compensation value.
    pub mdf_oec0cr: MDF_OEC0CR,
    _reserved16: [u8; 0x34],
    ///0xec - This register is used to read the data processed by each digital filter in snapshot mode.
    pub mdf_snps0dr: MDF_SNPS0DR,
    ///0xf0 - This register is used to read the data processed by each digital filter.
    pub mdf_dflt0dr: MDF_DFLT0DR,
    _reserved18: [u8; 0x0c],
    ///0x100 - This register is used to control the serial interfaces (SITFx).
    pub mdf_sitf1cr: MDF_SITF1CR,
    ///0x104 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    pub mdf_bsmx1cr: MDF_BSMX1CR,
    ///0x108 - This register is used to control the digital filter x.
    pub mdf_dflt1cr: MDF_DFLT1CR,
    ///0x10c - This register is used to control the main CIC filter.
    pub mdf_dflt1cicr: MDF_DFLT1CICR,
    ///0x110 - This register is used to control the reshape and HPF filters.
    pub mdf_dflt1rsfr: MDF_DFLT1RSFR,
    ///0x114 - This register is used to the integrator (INT) settings.
    pub mdf_dflt1intr: MDF_DFLT1INTR,
    ///0x118 - This register is used to configure the Out-of Limit Detector function.
    pub mdf_old1cr: MDF_OLD1CR,
    ///0x11c - This register is used for the adjustment of the Out-off Limit low threshold.
    pub mdf_old1thlr: MDF_OLD1THLR,
    ///0x120 - This register is used for the adjustment of the Out-off Limit high threshold.
    pub mdf_old1thhr: MDF_OLD1THHR,
    ///0x124 - This register is used for the adjustment stream delays.
    pub mdf_dly1cr: MDF_DLY1CR,
    ///0x128 - This register is used for the adjustment stream delays.
    pub mdf_scd1cr: MDF_SCD1CR,
    ///0x12c - MDF DFLTx interrupt enable register x
    pub mdf_dflt1ier: MDF_DFLT1IER,
    ///0x130 - This register contains the status flags for each digital filter path.
    pub mdf_dflt1isr: MDF_DFLT1ISR,
    ///0x134 - This register contains the offset compensation value.
    pub mdf_oec1cr: MDF_OEC1CR,
    _reserved32: [u8; 0x34],
    ///0x16c - This register is used to read the data processed by each digital filter in snapshot mode.
    pub mdf_snps1dr: MDF_SNPS1DR,
    ///0x170 - This register is used to read the data processed by each digital filter.
    pub mdf_dflt1dr: MDF_DFLT1DR,
    _reserved34: [u8; 0x0c],
    ///0x180 - This register is used to control the serial interfaces (SITFx).
    pub mdf_sitf2cr: MDF_SITF2CR,
    ///0x184 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    pub mdf_bsmx2cr: MDF_BSMX2CR,
    ///0x188 - This register is used to control the digital filter 2.
    pub mdf_dflt2cr: MDF_DFLT2CR,
    ///0x18c - This register is used to control the main CIC filter.
    pub mdf_dflt2cicr: MDF_DFLT2CICR,
    ///0x190 - This register is used to control the reshape and HPF filters.
    pub mdf_dflt2rsfr: MDF_DFLT2RSFR,
    ///0x194 - This register is used to the integrator (INT) settings.
    pub mdf_dflt2intr: MDF_DFLT2INTR,
    ///0x198 - This register is used to configure the Out-of Limit Detector function.
    pub mdf_old2cr: MDF_OLD2CR,
    ///0x19c - This register is used for the adjustment of the Out-off Limit low threshold.
    pub mdf_old2thlr: MDF_OLD2THLR,
    ///0x1a0 - This register is used for the adjustment of the Out-off Limit high threshold.
    pub mdf_old2thhr: MDF_OLD2THHR,
    ///0x1a4 - This register is used for the adjustment stream delays.
    pub mdf_dly2cr: MDF_DLY2CR,
    ///0x1a8 - This register is used for the adjustment stream delays.
    pub mdf_scd2cr: MDF_SCD2CR,
    ///0x1ac - MDF DFLTx interrupt enable register x
    pub mdf_dflt2ier: MDF_DFLT2IER,
    ///0x1b0 - This register contains the status flags for each digital filter path.
    pub mdf_dflt2isr: MDF_DFLT2ISR,
    ///0x1b4 - This register contains the offset compensation value.
    pub mdf_oec2cr: MDF_OEC2CR,
    _reserved48: [u8; 0x34],
    ///0x1ec - This register is used to read the data processed by each digital filter in snapshot mode.
    pub mdf_snps2dr: MDF_SNPS2DR,
    ///0x1f0 - This register is used to read the data processed by each digital filter.
    pub mdf_dflt2dr: MDF_DFLT2DR,
    _reserved50: [u8; 0x0c],
    ///0x200 - This register is used to control the serial interfaces (SITFx).
    pub mdf_sitf3cr: MDF_SITF3CR,
    ///0x204 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    pub mdf_bsmx3cr: MDF_BSMX3CR,
    ///0x208 - This register is used to control the digital filter 3.
    pub mdf_dflt3cr: MDF_DFLT3CR,
    ///0x20c - This register is used to control the main CIC filter.
    pub mdf_dflt3cicr: MDF_DFLT3CICR,
    ///0x210 - This register is used to control the reshape and HPF filters.
    pub mdf_dflt3rsfr: MDF_DFLT3RSFR,
    ///0x214 - This register is used to the integrator (INT) settings.
    pub mdf_dflt3intr: MDF_DFLT3INTR,
    ///0x218 - This register is used to configure the Out-of Limit Detector function.
    pub mdf_old3cr: MDF_OLD3CR,
    ///0x21c - This register is used for the adjustment of the Out-off Limit low threshold.
    pub mdf_old3thlr: MDF_OLD3THLR,
    ///0x220 - This register is used for the adjustment of the Out-off Limit high threshold.
    pub mdf_old3thhr: MDF_OLD3THHR,
    ///0x224 - This register is used for the adjustment stream delays.
    pub mdf_dly3cr: MDF_DLY3CR,
    ///0x228 - This register is used for the adjustment stream delays.
    pub mdf_scd3cr: MDF_SCD3CR,
    ///0x22c - MDF DFLTx interrupt enable register x
    pub mdf_dflt3ier: MDF_DFLT3IER,
    ///0x230 - This register contains the status flags for each digital filter path.
    pub mdf_dflt3isr: MDF_DFLT3ISR,
    ///0x234 - This register contains the offset compensation value.
    pub mdf_oec3cr: MDF_OEC3CR,
    _reserved64: [u8; 0x34],
    ///0x26c - This register is used to read the data processed by each digital filter in snapshot mode.
    pub mdf_snps3dr: MDF_SNPS3DR,
    ///0x270 - This register is used to read the data processed by each digital filter.
    pub mdf_dflt3dr: MDF_DFLT3DR,
    _reserved66: [u8; 0x0c],
    ///0x280 - This register is used to control the serial interfaces (SITFx).
    pub mdf_sitf4cr: MDF_SITF4CR,
    ///0x284 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    pub mdf_bsmx4cr: MDF_BSMX4CR,
    ///0x288 - This register is used to control the digital filter 4.
    pub mdf_dflt4cr: MDF_DFLT4CR,
    ///0x28c - This register is used to control the main CIC filter.
    pub mdf_dflt4cicr: MDF_DFLT4CICR,
    ///0x290 - This register is used to control the reshape and HPF filters.
    pub mdf_dflt4rsfr: MDF_DFLT4RSFR,
    ///0x294 - This register is used to the integrator (INT) settings.
    pub mdf_dflt4intr: MDF_DFLT4INTR,
    ///0x298 - This register is used to configure the Out-of Limit Detector function.
    pub mdf_old4cr: MDF_OLD4CR,
    ///0x29c - This register is used for the adjustment of the Out-off Limit low threshold.
    pub mdf_old4thlr: MDF_OLD4THLR,
    ///0x2a0 - This register is used for the adjustment of the Out-off Limit high threshold.
    pub mdf_old4thhr: MDF_OLD4THHR,
    ///0x2a4 - This register is used for the adjustment stream delays.
    pub mdf_dly4cr: MDF_DLY4CR,
    ///0x2a8 - This register is used for the adjustment stream delays.
    pub mdf_scd4cr: MDF_SCD4CR,
    ///0x2ac - MDF DFLTx interrupt enable register x
    pub mdf_dflt4ier: MDF_DFLT4IER,
    ///0x2b0 - This register contains the status flags for each digital filter path.
    pub mdf_dflt4isr: MDF_DFLT4ISR,
    ///0x2b4 - This register contains the offset compensation value.
    pub mdf_oec4cr: MDF_OEC4CR,
    _reserved80: [u8; 0x34],
    ///0x2ec - This register is used to read the data processed by each digital filter in snapshot mode.
    pub mdf_snps4dr: MDF_SNPS4DR,
    ///0x2f0 - This register is used to read the data processed by each digital filter.
    pub mdf_dflt4dr: MDF_DFLT4DR,
    _reserved82: [u8; 0x0c],
    ///0x300 - This register is used to control the serial interfaces (SITFx).
    pub mdf_sitf5cr: MDF_SITF5CR,
    ///0x304 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    pub mdf_bsmx5cr: MDF_BSMX5CR,
    ///0x308 - This register is used to control the digital filter x.
    pub mdf_dflt5cr: MDF_DFLT5CR,
    ///0x30c - This register is used to control the main CIC filter.
    pub mdf_dflt5cicr: MDF_DFLT5CICR,
    ///0x310 - This register is used to control the reshape and HPF filters.
    pub mdf_dflt5rsfr: MDF_DFLT5RSFR,
    ///0x314 - This register is used to the integrator (INT) settings.
    pub mdf_dflt5intr: MDF_DFLT5INTR,
    ///0x318 - This register is used to configure the Out-of Limit Detector function.
    pub mdf_old5cr: MDF_OLD5CR,
    ///0x31c - This register is used for the adjustment of the Out-off Limit low threshold.
    pub mdf_old5thlr: MDF_OLD5THLR,
    ///0x320 - This register is used for the adjustment of the Out-off Limit high threshold.
    pub mdf_old5thhr: MDF_OLD5THHR,
    ///0x324 - This register is used for the adjustment stream delays.
    pub mdf_dly5cr: MDF_DLY5CR,
    ///0x328 - This register is used for the adjustment stream delays.
    pub mdf_scd5cr: MDF_SCD5CR,
    ///0x32c - MDF DFLTx interrupt enable register x
    pub mdf_dflt5ier: MDF_DFLT5IER,
    ///0x330 - This register contains the status flags for each digital filter path.
    pub mdf_dflt5isr: MDF_DFLT5ISR,
    ///0x334 - This register contains the offset compensation value.
    pub mdf_oec5cr: MDF_OEC5CR,
    _reserved96: [u8; 0x34],
    ///0x36c - This register is used to read the data processed by each digital filter in snapshot mode.
    pub mdf_snps5dr: MDF_SNPS5DR,
    ///0x370 - This register is used to read the data processed by each digital filter.
    pub mdf_dflt5dr: MDF_DFLT5DR,
}
///GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
///MDF global control register
pub mod gcr;
///CKGCR (rw) register accessor: an alias for `Reg<CKGCR_SPEC>`
pub type CKGCR = crate::Reg<ckgcr::CKGCR_SPEC>;
///MDF clock generator control register
pub mod ckgcr;
///MDF_SITF0CR (rw) register accessor: an alias for `Reg<MDF_SITF0CR_SPEC>`
pub type MDF_SITF0CR = crate::Reg<mdf_sitf0cr::MDF_SITF0CR_SPEC>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf0cr;
///MDF_SITF1CR (rw) register accessor: an alias for `Reg<MDF_SITF1CR_SPEC>`
pub type MDF_SITF1CR = crate::Reg<mdf_sitf1cr::MDF_SITF1CR_SPEC>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf1cr;
///MDF_SITF2CR (rw) register accessor: an alias for `Reg<MDF_SITF2CR_SPEC>`
pub type MDF_SITF2CR = crate::Reg<mdf_sitf2cr::MDF_SITF2CR_SPEC>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf2cr;
///MDF_SITF3CR (rw) register accessor: an alias for `Reg<MDF_SITF3CR_SPEC>`
pub type MDF_SITF3CR = crate::Reg<mdf_sitf3cr::MDF_SITF3CR_SPEC>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf3cr;
///MDF_SITF4CR (rw) register accessor: an alias for `Reg<MDF_SITF4CR_SPEC>`
pub type MDF_SITF4CR = crate::Reg<mdf_sitf4cr::MDF_SITF4CR_SPEC>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf4cr;
///MDF_SITF5CR (rw) register accessor: an alias for `Reg<MDF_SITF5CR_SPEC>`
pub type MDF_SITF5CR = crate::Reg<mdf_sitf5cr::MDF_SITF5CR_SPEC>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf5cr;
///MDF_BSMX0CR (rw) register accessor: an alias for `Reg<MDF_BSMX0CR_SPEC>`
pub type MDF_BSMX0CR = crate::Reg<mdf_bsmx0cr::MDF_BSMX0CR_SPEC>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx0cr;
///MDF_BSMX1CR (rw) register accessor: an alias for `Reg<MDF_BSMX1CR_SPEC>`
pub type MDF_BSMX1CR = crate::Reg<mdf_bsmx1cr::MDF_BSMX1CR_SPEC>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx1cr;
///MDF_BSMX2CR (rw) register accessor: an alias for `Reg<MDF_BSMX2CR_SPEC>`
pub type MDF_BSMX2CR = crate::Reg<mdf_bsmx2cr::MDF_BSMX2CR_SPEC>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx2cr;
///MDF_BSMX3CR (rw) register accessor: an alias for `Reg<MDF_BSMX3CR_SPEC>`
pub type MDF_BSMX3CR = crate::Reg<mdf_bsmx3cr::MDF_BSMX3CR_SPEC>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx3cr;
///MDF_BSMX4CR (rw) register accessor: an alias for `Reg<MDF_BSMX4CR_SPEC>`
pub type MDF_BSMX4CR = crate::Reg<mdf_bsmx4cr::MDF_BSMX4CR_SPEC>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx4cr;
///MDF_BSMX5CR (rw) register accessor: an alias for `Reg<MDF_BSMX5CR_SPEC>`
pub type MDF_BSMX5CR = crate::Reg<mdf_bsmx5cr::MDF_BSMX5CR_SPEC>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx5cr;
///MDF_DFLT0CR (rw) register accessor: an alias for `Reg<MDF_DFLT0CR_SPEC>`
pub type MDF_DFLT0CR = crate::Reg<mdf_dflt0cr::MDF_DFLT0CR_SPEC>;
///This register is used to control the digital filter x.
pub mod mdf_dflt0cr;
///MDF_DFLT1CR (rw) register accessor: an alias for `Reg<MDF_DFLT1CR_SPEC>`
pub type MDF_DFLT1CR = crate::Reg<mdf_dflt1cr::MDF_DFLT1CR_SPEC>;
///This register is used to control the digital filter x.
pub mod mdf_dflt1cr;
///MDF_DFLT2CR (rw) register accessor: an alias for `Reg<MDF_DFLT2CR_SPEC>`
pub type MDF_DFLT2CR = crate::Reg<mdf_dflt2cr::MDF_DFLT2CR_SPEC>;
///This register is used to control the digital filter 2.
pub mod mdf_dflt2cr;
///MDF_DFLT3CR (rw) register accessor: an alias for `Reg<MDF_DFLT3CR_SPEC>`
pub type MDF_DFLT3CR = crate::Reg<mdf_dflt3cr::MDF_DFLT3CR_SPEC>;
///This register is used to control the digital filter 3.
pub mod mdf_dflt3cr;
///MDF_DFLT4CR (rw) register accessor: an alias for `Reg<MDF_DFLT4CR_SPEC>`
pub type MDF_DFLT4CR = crate::Reg<mdf_dflt4cr::MDF_DFLT4CR_SPEC>;
///This register is used to control the digital filter 4.
pub mod mdf_dflt4cr;
///MDF_DFLT5CR (rw) register accessor: an alias for `Reg<MDF_DFLT5CR_SPEC>`
pub type MDF_DFLT5CR = crate::Reg<mdf_dflt5cr::MDF_DFLT5CR_SPEC>;
///This register is used to control the digital filter x.
pub mod mdf_dflt5cr;
///MDF_DFLT0CICR (rw) register accessor: an alias for `Reg<MDF_DFLT0CICR_SPEC>`
pub type MDF_DFLT0CICR = crate::Reg<mdf_dflt0cicr::MDF_DFLT0CICR_SPEC>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt0cicr;
///MDF_DFLT1CICR (rw) register accessor: an alias for `Reg<MDF_DFLT1CICR_SPEC>`
pub type MDF_DFLT1CICR = crate::Reg<mdf_dflt1cicr::MDF_DFLT1CICR_SPEC>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt1cicr;
///MDF_DFLT2CICR (rw) register accessor: an alias for `Reg<MDF_DFLT2CICR_SPEC>`
pub type MDF_DFLT2CICR = crate::Reg<mdf_dflt2cicr::MDF_DFLT2CICR_SPEC>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt2cicr;
///MDF_DFLT3CICR (rw) register accessor: an alias for `Reg<MDF_DFLT3CICR_SPEC>`
pub type MDF_DFLT3CICR = crate::Reg<mdf_dflt3cicr::MDF_DFLT3CICR_SPEC>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt3cicr;
///MDF_DFLT4CICR (rw) register accessor: an alias for `Reg<MDF_DFLT4CICR_SPEC>`
pub type MDF_DFLT4CICR = crate::Reg<mdf_dflt4cicr::MDF_DFLT4CICR_SPEC>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt4cicr;
///MDF_DFLT5CICR (rw) register accessor: an alias for `Reg<MDF_DFLT5CICR_SPEC>`
pub type MDF_DFLT5CICR = crate::Reg<mdf_dflt5cicr::MDF_DFLT5CICR_SPEC>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt5cicr;
///MDF_DFLT0RSFR (rw) register accessor: an alias for `Reg<MDF_DFLT0RSFR_SPEC>`
pub type MDF_DFLT0RSFR = crate::Reg<mdf_dflt0rsfr::MDF_DFLT0RSFR_SPEC>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt0rsfr;
///MDF_DFLT1RSFR (rw) register accessor: an alias for `Reg<MDF_DFLT1RSFR_SPEC>`
pub type MDF_DFLT1RSFR = crate::Reg<mdf_dflt1rsfr::MDF_DFLT1RSFR_SPEC>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt1rsfr;
///MDF_DFLT2RSFR (rw) register accessor: an alias for `Reg<MDF_DFLT2RSFR_SPEC>`
pub type MDF_DFLT2RSFR = crate::Reg<mdf_dflt2rsfr::MDF_DFLT2RSFR_SPEC>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt2rsfr;
///MDF_DFLT3RSFR (rw) register accessor: an alias for `Reg<MDF_DFLT3RSFR_SPEC>`
pub type MDF_DFLT3RSFR = crate::Reg<mdf_dflt3rsfr::MDF_DFLT3RSFR_SPEC>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt3rsfr;
///MDF_DFLT4RSFR (rw) register accessor: an alias for `Reg<MDF_DFLT4RSFR_SPEC>`
pub type MDF_DFLT4RSFR = crate::Reg<mdf_dflt4rsfr::MDF_DFLT4RSFR_SPEC>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt4rsfr;
///MDF_DFLT5RSFR (rw) register accessor: an alias for `Reg<MDF_DFLT5RSFR_SPEC>`
pub type MDF_DFLT5RSFR = crate::Reg<mdf_dflt5rsfr::MDF_DFLT5RSFR_SPEC>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt5rsfr;
///MDF_DFLT0INTR (rw) register accessor: an alias for `Reg<MDF_DFLT0INTR_SPEC>`
pub type MDF_DFLT0INTR = crate::Reg<mdf_dflt0intr::MDF_DFLT0INTR_SPEC>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt0intr;
///MDF_DFLT1INTR (rw) register accessor: an alias for `Reg<MDF_DFLT1INTR_SPEC>`
pub type MDF_DFLT1INTR = crate::Reg<mdf_dflt1intr::MDF_DFLT1INTR_SPEC>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt1intr;
///MDF_DFLT2INTR (rw) register accessor: an alias for `Reg<MDF_DFLT2INTR_SPEC>`
pub type MDF_DFLT2INTR = crate::Reg<mdf_dflt2intr::MDF_DFLT2INTR_SPEC>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt2intr;
///MDF_DFLT3INTR (rw) register accessor: an alias for `Reg<MDF_DFLT3INTR_SPEC>`
pub type MDF_DFLT3INTR = crate::Reg<mdf_dflt3intr::MDF_DFLT3INTR_SPEC>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt3intr;
///MDF_DFLT4INTR (rw) register accessor: an alias for `Reg<MDF_DFLT4INTR_SPEC>`
pub type MDF_DFLT4INTR = crate::Reg<mdf_dflt4intr::MDF_DFLT4INTR_SPEC>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt4intr;
///MDF_DFLT5INTR (rw) register accessor: an alias for `Reg<MDF_DFLT5INTR_SPEC>`
pub type MDF_DFLT5INTR = crate::Reg<mdf_dflt5intr::MDF_DFLT5INTR_SPEC>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt5intr;
///MDF_OLD0CR (rw) register accessor: an alias for `Reg<MDF_OLD0CR_SPEC>`
pub type MDF_OLD0CR = crate::Reg<mdf_old0cr::MDF_OLD0CR_SPEC>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old0cr;
///MDF_OLD1CR (rw) register accessor: an alias for `Reg<MDF_OLD1CR_SPEC>`
pub type MDF_OLD1CR = crate::Reg<mdf_old1cr::MDF_OLD1CR_SPEC>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old1cr;
///MDF_OLD2CR (rw) register accessor: an alias for `Reg<MDF_OLD2CR_SPEC>`
pub type MDF_OLD2CR = crate::Reg<mdf_old2cr::MDF_OLD2CR_SPEC>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old2cr;
///MDF_OLD3CR (rw) register accessor: an alias for `Reg<MDF_OLD3CR_SPEC>`
pub type MDF_OLD3CR = crate::Reg<mdf_old3cr::MDF_OLD3CR_SPEC>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old3cr;
///MDF_OLD4CR (rw) register accessor: an alias for `Reg<MDF_OLD4CR_SPEC>`
pub type MDF_OLD4CR = crate::Reg<mdf_old4cr::MDF_OLD4CR_SPEC>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old4cr;
///MDF_OLD5CR (rw) register accessor: an alias for `Reg<MDF_OLD5CR_SPEC>`
pub type MDF_OLD5CR = crate::Reg<mdf_old5cr::MDF_OLD5CR_SPEC>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old5cr;
///MDF_OLD0THLR (rw) register accessor: an alias for `Reg<MDF_OLD0THLR_SPEC>`
pub type MDF_OLD0THLR = crate::Reg<mdf_old0thlr::MDF_OLD0THLR_SPEC>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old0thlr;
///MDF_OLD1THLR (rw) register accessor: an alias for `Reg<MDF_OLD1THLR_SPEC>`
pub type MDF_OLD1THLR = crate::Reg<mdf_old1thlr::MDF_OLD1THLR_SPEC>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old1thlr;
///MDF_OLD2THLR (rw) register accessor: an alias for `Reg<MDF_OLD2THLR_SPEC>`
pub type MDF_OLD2THLR = crate::Reg<mdf_old2thlr::MDF_OLD2THLR_SPEC>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old2thlr;
///MDF_OLD3THLR (rw) register accessor: an alias for `Reg<MDF_OLD3THLR_SPEC>`
pub type MDF_OLD3THLR = crate::Reg<mdf_old3thlr::MDF_OLD3THLR_SPEC>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old3thlr;
///MDF_OLD4THLR (rw) register accessor: an alias for `Reg<MDF_OLD4THLR_SPEC>`
pub type MDF_OLD4THLR = crate::Reg<mdf_old4thlr::MDF_OLD4THLR_SPEC>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old4thlr;
///MDF_OLD5THLR (rw) register accessor: an alias for `Reg<MDF_OLD5THLR_SPEC>`
pub type MDF_OLD5THLR = crate::Reg<mdf_old5thlr::MDF_OLD5THLR_SPEC>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old5thlr;
///MDF_OLD0THHR (rw) register accessor: an alias for `Reg<MDF_OLD0THHR_SPEC>`
pub type MDF_OLD0THHR = crate::Reg<mdf_old0thhr::MDF_OLD0THHR_SPEC>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old0thhr;
///MDF_OLD1THHR (rw) register accessor: an alias for `Reg<MDF_OLD1THHR_SPEC>`
pub type MDF_OLD1THHR = crate::Reg<mdf_old1thhr::MDF_OLD1THHR_SPEC>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old1thhr;
///MDF_OLD2THHR (rw) register accessor: an alias for `Reg<MDF_OLD2THHR_SPEC>`
pub type MDF_OLD2THHR = crate::Reg<mdf_old2thhr::MDF_OLD2THHR_SPEC>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old2thhr;
///MDF_OLD3THHR (rw) register accessor: an alias for `Reg<MDF_OLD3THHR_SPEC>`
pub type MDF_OLD3THHR = crate::Reg<mdf_old3thhr::MDF_OLD3THHR_SPEC>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old3thhr;
///MDF_OLD4THHR (rw) register accessor: an alias for `Reg<MDF_OLD4THHR_SPEC>`
pub type MDF_OLD4THHR = crate::Reg<mdf_old4thhr::MDF_OLD4THHR_SPEC>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old4thhr;
///MDF_OLD5THHR (rw) register accessor: an alias for `Reg<MDF_OLD5THHR_SPEC>`
pub type MDF_OLD5THHR = crate::Reg<mdf_old5thhr::MDF_OLD5THHR_SPEC>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old5thhr;
///MDF_DLY0CR (rw) register accessor: an alias for `Reg<MDF_DLY0CR_SPEC>`
pub type MDF_DLY0CR = crate::Reg<mdf_dly0cr::MDF_DLY0CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly0cr;
///MDF_DLY1CR (rw) register accessor: an alias for `Reg<MDF_DLY1CR_SPEC>`
pub type MDF_DLY1CR = crate::Reg<mdf_dly1cr::MDF_DLY1CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly1cr;
///MDF_DLY2CR (rw) register accessor: an alias for `Reg<MDF_DLY2CR_SPEC>`
pub type MDF_DLY2CR = crate::Reg<mdf_dly2cr::MDF_DLY2CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly2cr;
///MDF_DLY3CR (rw) register accessor: an alias for `Reg<MDF_DLY3CR_SPEC>`
pub type MDF_DLY3CR = crate::Reg<mdf_dly3cr::MDF_DLY3CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly3cr;
///MDF_DLY4CR (rw) register accessor: an alias for `Reg<MDF_DLY4CR_SPEC>`
pub type MDF_DLY4CR = crate::Reg<mdf_dly4cr::MDF_DLY4CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly4cr;
///MDF_DLY5CR (rw) register accessor: an alias for `Reg<MDF_DLY5CR_SPEC>`
pub type MDF_DLY5CR = crate::Reg<mdf_dly5cr::MDF_DLY5CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly5cr;
///MDF_SCD0CR (rw) register accessor: an alias for `Reg<MDF_SCD0CR_SPEC>`
pub type MDF_SCD0CR = crate::Reg<mdf_scd0cr::MDF_SCD0CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd0cr;
///MDF_SCD1CR (rw) register accessor: an alias for `Reg<MDF_SCD1CR_SPEC>`
pub type MDF_SCD1CR = crate::Reg<mdf_scd1cr::MDF_SCD1CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd1cr;
///MDF_SCD2CR (rw) register accessor: an alias for `Reg<MDF_SCD2CR_SPEC>`
pub type MDF_SCD2CR = crate::Reg<mdf_scd2cr::MDF_SCD2CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd2cr;
///MDF_SCD3CR (rw) register accessor: an alias for `Reg<MDF_SCD3CR_SPEC>`
pub type MDF_SCD3CR = crate::Reg<mdf_scd3cr::MDF_SCD3CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd3cr;
///MDF_SCD4CR (rw) register accessor: an alias for `Reg<MDF_SCD4CR_SPEC>`
pub type MDF_SCD4CR = crate::Reg<mdf_scd4cr::MDF_SCD4CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd4cr;
///MDF_SCD5CR (rw) register accessor: an alias for `Reg<MDF_SCD5CR_SPEC>`
pub type MDF_SCD5CR = crate::Reg<mdf_scd5cr::MDF_SCD5CR_SPEC>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd5cr;
///MDF_DFLT0IER (rw) register accessor: an alias for `Reg<MDF_DFLT0IER_SPEC>`
pub type MDF_DFLT0IER = crate::Reg<mdf_dflt0ier::MDF_DFLT0IER_SPEC>;
///This register is used for allowing or not the events to generate an interrupt.
pub mod mdf_dflt0ier;
///MDF_DFLT0ISR (rw) register accessor: an alias for `Reg<MDF_DFLT0ISR_SPEC>`
pub type MDF_DFLT0ISR = crate::Reg<mdf_dflt0isr::MDF_DFLT0ISR_SPEC>;
///MDF DFLT0 interrupt status register 0
pub mod mdf_dflt0isr;
///MDF_DFLT1IER (rw) register accessor: an alias for `Reg<MDF_DFLT1IER_SPEC>`
pub type MDF_DFLT1IER = crate::Reg<mdf_dflt1ier::MDF_DFLT1IER_SPEC>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt1ier;
///MDF_DFLT2IER (rw) register accessor: an alias for `Reg<MDF_DFLT2IER_SPEC>`
pub type MDF_DFLT2IER = crate::Reg<mdf_dflt2ier::MDF_DFLT2IER_SPEC>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt2ier;
///MDF_DFLT3IER (rw) register accessor: an alias for `Reg<MDF_DFLT3IER_SPEC>`
pub type MDF_DFLT3IER = crate::Reg<mdf_dflt3ier::MDF_DFLT3IER_SPEC>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt3ier;
///MDF_DFLT4IER (rw) register accessor: an alias for `Reg<MDF_DFLT4IER_SPEC>`
pub type MDF_DFLT4IER = crate::Reg<mdf_dflt4ier::MDF_DFLT4IER_SPEC>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt4ier;
///MDF_DFLT5IER (rw) register accessor: an alias for `Reg<MDF_DFLT5IER_SPEC>`
pub type MDF_DFLT5IER = crate::Reg<mdf_dflt5ier::MDF_DFLT5IER_SPEC>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt5ier;
///MDF_DFLT1ISR (rw) register accessor: an alias for `Reg<MDF_DFLT1ISR_SPEC>`
pub type MDF_DFLT1ISR = crate::Reg<mdf_dflt1isr::MDF_DFLT1ISR_SPEC>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt1isr;
///MDF_DFLT2ISR (rw) register accessor: an alias for `Reg<MDF_DFLT2ISR_SPEC>`
pub type MDF_DFLT2ISR = crate::Reg<mdf_dflt2isr::MDF_DFLT2ISR_SPEC>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt2isr;
///MDF_DFLT3ISR (rw) register accessor: an alias for `Reg<MDF_DFLT3ISR_SPEC>`
pub type MDF_DFLT3ISR = crate::Reg<mdf_dflt3isr::MDF_DFLT3ISR_SPEC>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt3isr;
///MDF_DFLT4ISR (rw) register accessor: an alias for `Reg<MDF_DFLT4ISR_SPEC>`
pub type MDF_DFLT4ISR = crate::Reg<mdf_dflt4isr::MDF_DFLT4ISR_SPEC>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt4isr;
///MDF_DFLT5ISR (rw) register accessor: an alias for `Reg<MDF_DFLT5ISR_SPEC>`
pub type MDF_DFLT5ISR = crate::Reg<mdf_dflt5isr::MDF_DFLT5ISR_SPEC>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt5isr;
///MDF_OEC0CR (rw) register accessor: an alias for `Reg<MDF_OEC0CR_SPEC>`
pub type MDF_OEC0CR = crate::Reg<mdf_oec0cr::MDF_OEC0CR_SPEC>;
///This register contains the offset compensation value.
pub mod mdf_oec0cr;
///MDF_OEC1CR (rw) register accessor: an alias for `Reg<MDF_OEC1CR_SPEC>`
pub type MDF_OEC1CR = crate::Reg<mdf_oec1cr::MDF_OEC1CR_SPEC>;
///This register contains the offset compensation value.
pub mod mdf_oec1cr;
///MDF_OEC2CR (rw) register accessor: an alias for `Reg<MDF_OEC2CR_SPEC>`
pub type MDF_OEC2CR = crate::Reg<mdf_oec2cr::MDF_OEC2CR_SPEC>;
///This register contains the offset compensation value.
pub mod mdf_oec2cr;
///MDF_OEC3CR (rw) register accessor: an alias for `Reg<MDF_OEC3CR_SPEC>`
pub type MDF_OEC3CR = crate::Reg<mdf_oec3cr::MDF_OEC3CR_SPEC>;
///This register contains the offset compensation value.
pub mod mdf_oec3cr;
///MDF_OEC4CR (rw) register accessor: an alias for `Reg<MDF_OEC4CR_SPEC>`
pub type MDF_OEC4CR = crate::Reg<mdf_oec4cr::MDF_OEC4CR_SPEC>;
///This register contains the offset compensation value.
pub mod mdf_oec4cr;
///MDF_OEC5CR (rw) register accessor: an alias for `Reg<MDF_OEC5CR_SPEC>`
pub type MDF_OEC5CR = crate::Reg<mdf_oec5cr::MDF_OEC5CR_SPEC>;
///This register contains the offset compensation value.
pub mod mdf_oec5cr;
///MDF_SNPS0DR (r) register accessor: an alias for `Reg<MDF_SNPS0DR_SPEC>`
pub type MDF_SNPS0DR = crate::Reg<mdf_snps0dr::MDF_SNPS0DR_SPEC>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps0dr;
///MDF_SNPS1DR (r) register accessor: an alias for `Reg<MDF_SNPS1DR_SPEC>`
pub type MDF_SNPS1DR = crate::Reg<mdf_snps1dr::MDF_SNPS1DR_SPEC>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps1dr;
///MDF_SNPS2DR (r) register accessor: an alias for `Reg<MDF_SNPS2DR_SPEC>`
pub type MDF_SNPS2DR = crate::Reg<mdf_snps2dr::MDF_SNPS2DR_SPEC>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps2dr;
///MDF_SNPS3DR (r) register accessor: an alias for `Reg<MDF_SNPS3DR_SPEC>`
pub type MDF_SNPS3DR = crate::Reg<mdf_snps3dr::MDF_SNPS3DR_SPEC>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps3dr;
///MDF_SNPS4DR (r) register accessor: an alias for `Reg<MDF_SNPS4DR_SPEC>`
pub type MDF_SNPS4DR = crate::Reg<mdf_snps4dr::MDF_SNPS4DR_SPEC>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps4dr;
///MDF_SNPS5DR (r) register accessor: an alias for `Reg<MDF_SNPS5DR_SPEC>`
pub type MDF_SNPS5DR = crate::Reg<mdf_snps5dr::MDF_SNPS5DR_SPEC>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps5dr;
///MDF_DFLT0DR (r) register accessor: an alias for `Reg<MDF_DFLT0DR_SPEC>`
pub type MDF_DFLT0DR = crate::Reg<mdf_dflt0dr::MDF_DFLT0DR_SPEC>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt0dr;
///MDF_DFLT1DR (r) register accessor: an alias for `Reg<MDF_DFLT1DR_SPEC>`
pub type MDF_DFLT1DR = crate::Reg<mdf_dflt1dr::MDF_DFLT1DR_SPEC>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt1dr;
///MDF_DFLT2DR (r) register accessor: an alias for `Reg<MDF_DFLT2DR_SPEC>`
pub type MDF_DFLT2DR = crate::Reg<mdf_dflt2dr::MDF_DFLT2DR_SPEC>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt2dr;
///MDF_DFLT3DR (r) register accessor: an alias for `Reg<MDF_DFLT3DR_SPEC>`
pub type MDF_DFLT3DR = crate::Reg<mdf_dflt3dr::MDF_DFLT3DR_SPEC>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt3dr;
///MDF_DFLT4DR (r) register accessor: an alias for `Reg<MDF_DFLT4DR_SPEC>`
pub type MDF_DFLT4DR = crate::Reg<mdf_dflt4dr::MDF_DFLT4DR_SPEC>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt4dr;
///MDF_DFLT5DR (r) register accessor: an alias for `Reg<MDF_DFLT5DR_SPEC>`
pub type MDF_DFLT5DR = crate::Reg<mdf_dflt5dr::MDF_DFLT5DR_SPEC>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt5dr;
