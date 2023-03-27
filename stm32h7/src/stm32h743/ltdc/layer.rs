///Register block
#[repr(C)]
pub struct LAYER {
    ///0x00 - Layerx Control Register
    pub cr: CR,
    ///0x04 - Layerx Window Horizontal Position Configuration Register
    pub whpcr: WHPCR,
    ///0x08 - Layerx Window Vertical Position Configuration Register
    pub wvpcr: WVPCR,
    ///0x0c - Layerx Color Keying Configuration Register
    pub ckcr: CKCR,
    ///0x10 - Layerx Pixel Format Configuration Register
    pub pfcr: PFCR,
    ///0x14 - Layerx Constant Alpha Configuration Register
    pub cacr: CACR,
    ///0x18 - Layerx Default Color Configuration Register
    pub dccr: DCCR,
    ///0x1c - Layerx Blending Factors Configuration Register
    pub bfcr: BFCR,
    _reserved8: [u8; 0x08],
    ///0x28 - Layerx Color Frame Buffer Address Register
    pub cfbar: CFBAR,
    ///0x2c - Layerx Color Frame Buffer Length Register
    pub cfblr: CFBLR,
    ///0x30 - Layerx ColorFrame Buffer Line Number Register
    pub cfblnr: CFBLNR,
    _reserved11: [u8; 0x0c],
    ///0x40 - Layerx CLUT Write Register
    pub clutwr: CLUTWR,
    _reserved_end: [u8; 0x3c],
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Layerx Control Register
pub mod cr;
///WHPCR (rw) register accessor: an alias for `Reg<WHPCR_SPEC>`
pub type WHPCR = crate::Reg<whpcr::WHPCR_SPEC>;
///Layerx Window Horizontal Position Configuration Register
pub mod whpcr;
///WVPCR (rw) register accessor: an alias for `Reg<WVPCR_SPEC>`
pub type WVPCR = crate::Reg<wvpcr::WVPCR_SPEC>;
///Layerx Window Vertical Position Configuration Register
pub mod wvpcr;
///CKCR (rw) register accessor: an alias for `Reg<CKCR_SPEC>`
pub type CKCR = crate::Reg<ckcr::CKCR_SPEC>;
///Layerx Color Keying Configuration Register
pub mod ckcr;
///PFCR (rw) register accessor: an alias for `Reg<PFCR_SPEC>`
pub type PFCR = crate::Reg<pfcr::PFCR_SPEC>;
///Layerx Pixel Format Configuration Register
pub mod pfcr;
///CACR (rw) register accessor: an alias for `Reg<CACR_SPEC>`
pub type CACR = crate::Reg<cacr::CACR_SPEC>;
///Layerx Constant Alpha Configuration Register
pub mod cacr;
///DCCR (rw) register accessor: an alias for `Reg<DCCR_SPEC>`
pub type DCCR = crate::Reg<dccr::DCCR_SPEC>;
///Layerx Default Color Configuration Register
pub mod dccr;
///BFCR (rw) register accessor: an alias for `Reg<BFCR_SPEC>`
pub type BFCR = crate::Reg<bfcr::BFCR_SPEC>;
///Layerx Blending Factors Configuration Register
pub mod bfcr;
///CFBAR (rw) register accessor: an alias for `Reg<CFBAR_SPEC>`
pub type CFBAR = crate::Reg<cfbar::CFBAR_SPEC>;
///Layerx Color Frame Buffer Address Register
pub mod cfbar;
///CFBLR (rw) register accessor: an alias for `Reg<CFBLR_SPEC>`
pub type CFBLR = crate::Reg<cfblr::CFBLR_SPEC>;
///Layerx Color Frame Buffer Length Register
pub mod cfblr;
///CFBLNR (rw) register accessor: an alias for `Reg<CFBLNR_SPEC>`
pub type CFBLNR = crate::Reg<cfblnr::CFBLNR_SPEC>;
///Layerx ColorFrame Buffer Line Number Register
pub mod cfblnr;
///CLUTWR (w) register accessor: an alias for `Reg<CLUTWR_SPEC>`
pub type CLUTWR = crate::Reg<clutwr::CLUTWR_SPEC>;
///Layerx CLUT Write Register
pub mod clutwr;
