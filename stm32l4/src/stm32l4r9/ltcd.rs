///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ///0x08 - LTDC Synchronization Size Configuration Register
    pub sscr: SSCR,
    ///0x0c - LTDC Back Porch Configuration Register
    pub bpcr: BPCR,
    ///0x10 - LTDC Active Width Configuration Register
    pub awcr: AWCR,
    ///0x14 - LTDC Total Width Configuration Register
    pub twcr: TWCR,
    ///0x18 - LTDC Global Control Register
    pub gcr: GCR,
    _reserved5: [u8; 0x08],
    ///0x24 - LTDC Shadow Reload Configuration Register
    pub srcr: SRCR,
    _reserved6: [u8; 0x04],
    ///0x2c - LTDC Background Color Configuration Register
    pub bccr: BCCR,
    _reserved7: [u8; 0x04],
    ///0x34 - LTDC Interrupt Enable Register
    pub ier: IER,
    ///0x38 - LTDC Interrupt Status Register
    pub isr: ISR,
    ///0x3c - LTDC Interrupt Clear Register
    pub icr: ICR,
    ///0x40 - LTDC Line Interrupt Position Configuration Register
    pub lipcr: LIPCR,
    ///0x44 - LTDC Current Position Status Register
    pub cpsr: CPSR,
    ///0x48 - LTDC Current Display Status Register
    pub cdsr: CDSR,
    _reserved13: [u8; 0x38],
    ///0x84 - LTDC Layer Control Register
    pub l1cr: L1CR,
    ///0x88 - LTDC Layer Window Horizontal Position Configuration Register
    pub l1whpcr: L1WHPCR,
    ///0x8c - LTDC Layer Window Vertical Position Configuration Register
    pub l1wvpcr: L1WVPCR,
    ///0x90 - LTDC Layer Color Keying Configuration Register
    pub l1ckcr: L1CKCR,
    ///0x94 - LTDC Layer Pixel Format Configuration Register
    pub l1pfcr: L1PFCR,
    ///0x98 - LTDC Layer Constant Alpha Configuration Register
    pub l1cacr: L1CACR,
    ///0x9c - LTDC Layer Default Color Configuration Register
    pub l1dccr: L1DCCR,
    ///0xa0 - LTDC Layer Blending Factors Configuration Register
    pub l1bfcr: L1BFCR,
    _reserved21: [u8; 0x08],
    ///0xac - LTDC Layer Color Frame Buffer Address Register
    pub l1cfbar: L1CFBAR,
    ///0xb0 - LTDC Layer Color Frame Buffer Length Register
    pub l1cfblr: L1CFBLR,
    ///0xb4 - LTDC Layer ColorFrame Buffer Line Number Register
    pub l1cfblnr: L1CFBLNR,
    _reserved24: [u8; 0x0c],
    ///0xc4 - LTDC Layerx CLUT Write Register
    pub l1clutwr: L1CLUTWR,
    _reserved25: [u8; 0x3c],
    ///0x104 - LTDC Layer Control Register
    pub l2cr: L2CR,
    ///0x108 - LTDC Layerx Window Horizontal Position Configuration Register
    pub l2whpcr: L2WHPCR,
    ///0x10c - LTDC Layer Window Vertical Position Configuration Register
    pub l2wvpcr: L2WVPCR,
    ///0x110 - LTDC Layer Color Keying Configuration Register
    pub l2ckcr: L2CKCR,
    ///0x114 - LTDC Layer Pixel Format Configuration Register
    pub l2pfcr: L2PFCR,
    ///0x118 - LTDC Layer Constant Alpha Configuration Register
    pub l2cacr: L2CACR,
    ///0x11c - LTDC Layer Default Color Configuration Register
    pub l2dccr: L2DCCR,
    _reserved32: [u8; 0x04],
    ///0x124 - LTDC Layer Blending Factors Configuration Register
    pub l2bfcr: L2BFCR,
    _reserved33: [u8; 0x04],
    ///0x12c - LTDC Layer Color Frame Buffer Address Register
    pub l2cfbar: L2CFBAR,
    ///0x130 - LTDC Layer Color Frame Buffer Length Register
    pub l2cfblr: L2CFBLR,
    ///0x134 - LTDC Layer ColorFrame Buffer Line Number Register
    pub l2cfblnr: L2CFBLNR,
    _reserved36: [u8; 0x0c],
    ///0x144 - LTDC Layerx CLUT Write Register
    pub l2clutwr: L2CLUTWR,
}
///SSCR (rw) register accessor: an alias for `Reg<SSCR_SPEC>`
pub type SSCR = crate::Reg<sscr::SSCR_SPEC>;
///LTDC Synchronization Size Configuration Register
pub mod sscr;
///BPCR (rw) register accessor: an alias for `Reg<BPCR_SPEC>`
pub type BPCR = crate::Reg<bpcr::BPCR_SPEC>;
///LTDC Back Porch Configuration Register
pub mod bpcr;
///AWCR (rw) register accessor: an alias for `Reg<AWCR_SPEC>`
pub type AWCR = crate::Reg<awcr::AWCR_SPEC>;
///LTDC Active Width Configuration Register
pub mod awcr;
///TWCR (rw) register accessor: an alias for `Reg<TWCR_SPEC>`
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
///LTDC Total Width Configuration Register
pub mod twcr;
///GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
///LTDC Global Control Register
pub mod gcr;
///SRCR (rw) register accessor: an alias for `Reg<SRCR_SPEC>`
pub type SRCR = crate::Reg<srcr::SRCR_SPEC>;
///LTDC Shadow Reload Configuration Register
pub mod srcr;
///BCCR (rw) register accessor: an alias for `Reg<BCCR_SPEC>`
pub type BCCR = crate::Reg<bccr::BCCR_SPEC>;
///LTDC Background Color Configuration Register
pub mod bccr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///LTDC Interrupt Enable Register
pub mod ier;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///LTDC Interrupt Status Register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///LTDC Interrupt Clear Register
pub mod icr;
///LIPCR (rw) register accessor: an alias for `Reg<LIPCR_SPEC>`
pub type LIPCR = crate::Reg<lipcr::LIPCR_SPEC>;
///LTDC Line Interrupt Position Configuration Register
pub mod lipcr;
///CPSR (r) register accessor: an alias for `Reg<CPSR_SPEC>`
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
///LTDC Current Position Status Register
pub mod cpsr;
///CDSR (r) register accessor: an alias for `Reg<CDSR_SPEC>`
pub type CDSR = crate::Reg<cdsr::CDSR_SPEC>;
///LTDC Current Display Status Register
pub mod cdsr;
///L1CR (rw) register accessor: an alias for `Reg<L1CR_SPEC>`
pub type L1CR = crate::Reg<l1cr::L1CR_SPEC>;
///LTDC Layer Control Register
pub mod l1cr;
///L2CR (rw) register accessor: an alias for `Reg<L2CR_SPEC>`
pub type L2CR = crate::Reg<l2cr::L2CR_SPEC>;
///LTDC Layer Control Register
pub mod l2cr;
///L1WHPCR (rw) register accessor: an alias for `Reg<L1WHPCR_SPEC>`
pub type L1WHPCR = crate::Reg<l1whpcr::L1WHPCR_SPEC>;
///LTDC Layer Window Horizontal Position Configuration Register
pub mod l1whpcr;
///L2WHPCR (rw) register accessor: an alias for `Reg<L2WHPCR_SPEC>`
pub type L2WHPCR = crate::Reg<l2whpcr::L2WHPCR_SPEC>;
///LTDC Layerx Window Horizontal Position Configuration Register
pub mod l2whpcr;
///L1WVPCR (rw) register accessor: an alias for `Reg<L1WVPCR_SPEC>`
pub type L1WVPCR = crate::Reg<l1wvpcr::L1WVPCR_SPEC>;
///LTDC Layer Window Vertical Position Configuration Register
pub mod l1wvpcr;
///L2WVPCR (rw) register accessor: an alias for `Reg<L2WVPCR_SPEC>`
pub type L2WVPCR = crate::Reg<l2wvpcr::L2WVPCR_SPEC>;
///LTDC Layer Window Vertical Position Configuration Register
pub mod l2wvpcr;
///L1CKCR (rw) register accessor: an alias for `Reg<L1CKCR_SPEC>`
pub type L1CKCR = crate::Reg<l1ckcr::L1CKCR_SPEC>;
///LTDC Layer Color Keying Configuration Register
pub mod l1ckcr;
///L2CKCR (rw) register accessor: an alias for `Reg<L2CKCR_SPEC>`
pub type L2CKCR = crate::Reg<l2ckcr::L2CKCR_SPEC>;
///LTDC Layer Color Keying Configuration Register
pub mod l2ckcr;
///L1PFCR (rw) register accessor: an alias for `Reg<L1PFCR_SPEC>`
pub type L1PFCR = crate::Reg<l1pfcr::L1PFCR_SPEC>;
///LTDC Layer Pixel Format Configuration Register
pub mod l1pfcr;
///L2PFCR (rw) register accessor: an alias for `Reg<L2PFCR_SPEC>`
pub type L2PFCR = crate::Reg<l2pfcr::L2PFCR_SPEC>;
///LTDC Layer Pixel Format Configuration Register
pub mod l2pfcr;
///L1CACR (rw) register accessor: an alias for `Reg<L1CACR_SPEC>`
pub type L1CACR = crate::Reg<l1cacr::L1CACR_SPEC>;
///LTDC Layer Constant Alpha Configuration Register
pub mod l1cacr;
///L2CACR (rw) register accessor: an alias for `Reg<L2CACR_SPEC>`
pub type L2CACR = crate::Reg<l2cacr::L2CACR_SPEC>;
///LTDC Layer Constant Alpha Configuration Register
pub mod l2cacr;
///L1DCCR (rw) register accessor: an alias for `Reg<L1DCCR_SPEC>`
pub type L1DCCR = crate::Reg<l1dccr::L1DCCR_SPEC>;
///LTDC Layer Default Color Configuration Register
pub mod l1dccr;
///L2DCCR (rw) register accessor: an alias for `Reg<L2DCCR_SPEC>`
pub type L2DCCR = crate::Reg<l2dccr::L2DCCR_SPEC>;
///LTDC Layer Default Color Configuration Register
pub mod l2dccr;
///L1BFCR (rw) register accessor: an alias for `Reg<L1BFCR_SPEC>`
pub type L1BFCR = crate::Reg<l1bfcr::L1BFCR_SPEC>;
///LTDC Layer Blending Factors Configuration Register
pub mod l1bfcr;
///L2BFCR (rw) register accessor: an alias for `Reg<L2BFCR_SPEC>`
pub type L2BFCR = crate::Reg<l2bfcr::L2BFCR_SPEC>;
///LTDC Layer Blending Factors Configuration Register
pub mod l2bfcr;
///L1CFBAR (rw) register accessor: an alias for `Reg<L1CFBAR_SPEC>`
pub type L1CFBAR = crate::Reg<l1cfbar::L1CFBAR_SPEC>;
///LTDC Layer Color Frame Buffer Address Register
pub mod l1cfbar;
///L2CFBAR (rw) register accessor: an alias for `Reg<L2CFBAR_SPEC>`
pub type L2CFBAR = crate::Reg<l2cfbar::L2CFBAR_SPEC>;
///LTDC Layer Color Frame Buffer Address Register
pub mod l2cfbar;
///L1CFBLR (rw) register accessor: an alias for `Reg<L1CFBLR_SPEC>`
pub type L1CFBLR = crate::Reg<l1cfblr::L1CFBLR_SPEC>;
///LTDC Layer Color Frame Buffer Length Register
pub mod l1cfblr;
///L2CFBLR (rw) register accessor: an alias for `Reg<L2CFBLR_SPEC>`
pub type L2CFBLR = crate::Reg<l2cfblr::L2CFBLR_SPEC>;
///LTDC Layer Color Frame Buffer Length Register
pub mod l2cfblr;
///L1CFBLNR (rw) register accessor: an alias for `Reg<L1CFBLNR_SPEC>`
pub type L1CFBLNR = crate::Reg<l1cfblnr::L1CFBLNR_SPEC>;
///LTDC Layer ColorFrame Buffer Line Number Register
pub mod l1cfblnr;
///L2CFBLNR (rw) register accessor: an alias for `Reg<L2CFBLNR_SPEC>`
pub type L2CFBLNR = crate::Reg<l2cfblnr::L2CFBLNR_SPEC>;
///LTDC Layer ColorFrame Buffer Line Number Register
pub mod l2cfblnr;
///L1CLUTWR (w) register accessor: an alias for `Reg<L1CLUTWR_SPEC>`
pub type L1CLUTWR = crate::Reg<l1clutwr::L1CLUTWR_SPEC>;
///LTDC Layerx CLUT Write Register
pub mod l1clutwr;
///L2CLUTWR (w) register accessor: an alias for `Reg<L2CLUTWR_SPEC>`
pub type L2CLUTWR = crate::Reg<l2clutwr::L2CLUTWR_SPEC>;
///LTDC Layerx CLUT Write Register
pub mod l2clutwr;
