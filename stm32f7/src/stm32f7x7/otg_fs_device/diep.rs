///Register block
#[repr(C)]
pub struct DIEP {
    ///0x00 - OTG device endpoint-1 control register
    pub ctl: CTL,
    _reserved1: [u8; 0x04],
    ///0x08 - device endpoint-1 interrupt register
    pub int: INT,
    _reserved2: [u8; 0x04],
    ///0x10 - device endpoint-1 transfer size register
    pub tsiz: TSIZ,
    _reserved3: [u8; 0x04],
    ///0x18 - OTG_FS device IN endpoint transmit FIFO status register
    pub txfsts: TXFSTS,
    _reserved_end: [u8; 0x04],
}
///CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
///OTG device endpoint-1 control register
pub mod ctl;
///INT (rw) register accessor: an alias for `Reg<INT_SPEC>`
pub type INT = crate::Reg<int::INT_SPEC>;
///device endpoint-1 interrupt register
pub mod int;
///TSIZ (rw) register accessor: an alias for `Reg<TSIZ_SPEC>`
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
///device endpoint-1 transfer size register
pub mod tsiz;
///TXFSTS (r) register accessor: an alias for `Reg<TXFSTS_SPEC>`
pub type TXFSTS = crate::Reg<txfsts::TXFSTS_SPEC>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod txfsts;
