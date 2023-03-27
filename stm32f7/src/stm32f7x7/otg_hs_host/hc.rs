///Register block
#[repr(C)]
pub struct HC {
    ///0x00 - OTG_HS host channel-0 characteristics register
    pub char: CHAR,
    ///0x04 - OTG_HS host channel-0 split control register
    pub splt: SPLT,
    ///0x08 - OTG_HS host channel-11 interrupt register
    pub int: INT,
    ///0x0c - OTG_HS host channel-11 interrupt mask register
    pub intmsk: INTMSK,
    ///0x10 - OTG_HS host channel-11 transfer size register
    pub tsiz: TSIZ,
    ///0x14 - OTG_HS host channel-0 DMA address register
    pub dma: DMA,
    _reserved_end: [u8; 0x08],
}
///CHAR (rw) register accessor: an alias for `Reg<CHAR_SPEC>`
pub type CHAR = crate::Reg<char::CHAR_SPEC>;
///OTG_HS host channel-0 characteristics register
pub mod char;
///SPLT (rw) register accessor: an alias for `Reg<SPLT_SPEC>`
pub type SPLT = crate::Reg<splt::SPLT_SPEC>;
///OTG_HS host channel-0 split control register
pub mod splt;
///INT (rw) register accessor: an alias for `Reg<INT_SPEC>`
pub type INT = crate::Reg<int::INT_SPEC>;
///OTG_HS host channel-11 interrupt register
pub mod int;
///INTMSK (rw) register accessor: an alias for `Reg<INTMSK_SPEC>`
pub type INTMSK = crate::Reg<intmsk::INTMSK_SPEC>;
///OTG_HS host channel-11 interrupt mask register
pub mod intmsk;
///TSIZ (rw) register accessor: an alias for `Reg<TSIZ_SPEC>`
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
///OTG_HS host channel-11 transfer size register
pub mod tsiz;
///DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`
pub type DMA = crate::Reg<dma::DMA_SPEC>;
///OTG_HS host channel-0 DMA address register
pub mod dma;
