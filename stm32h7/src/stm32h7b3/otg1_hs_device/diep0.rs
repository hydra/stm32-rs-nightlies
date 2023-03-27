///Register block
#[repr(C)]
pub struct DIEP0 {
    ///0x00 - OTG device endpoint-0 control register
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    ///0x08 - OTG device endpoint-0 interrupt register
    pub int: INT,
    _reserved2: [u8; 0x04],
    ///0x10 - OTG_HS device IN endpoint 0 transfer size register
    pub tsiz: TSIZ,
    ///0x14 - OTG_HS device endpoint-0 DMA address register
    pub dma: DMA,
    ///0x18 - OTG_HS device IN endpoint transmit FIFO status register
    pub txfsts: TXFSTS,
}
///CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
///OTG device endpoint-0 control register
pub mod ctl;
///INT (rw) register accessor: an alias for `Reg<INT_SPEC>`
pub type INT = crate::Reg<int::INT_SPEC>;
///OTG device endpoint-0 interrupt register
pub mod int;
///TSIZ (rw) register accessor: an alias for `Reg<TSIZ_SPEC>`
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
///OTG_HS device IN endpoint 0 transfer size register
pub mod tsiz;
///DMA (rw) register accessor: an alias for `Reg<DMA_SPEC>`
pub type DMA = crate::Reg<dma::DMA_SPEC>;
///OTG_HS device endpoint-0 DMA address register
pub mod dma;
///TXFSTS (r) register accessor: an alias for `Reg<TXFSTS_SPEC>`
pub type TXFSTS = crate::Reg<txfsts::TXFSTS_SPEC>;
///OTG_HS device IN endpoint transmit FIFO status register
pub mod txfsts;
