///Register block
#[repr(C)]
pub struct HC {
    ///0x00 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub char: CHAR,
    _reserved1: [u8; 0x04],
    ///0x08 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub int: INT,
    ///0x0c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub intmsk: INTMSK,
    ///0x10 - OTG_FS host channel-0 transfer size register
    pub tsiz: TSIZ,
    _reserved_end: [u8; 0x0c],
}
///CHAR (rw) register accessor: an alias for `Reg<CHAR_SPEC>`
pub type CHAR = crate::Reg<char::CHAR_SPEC>;
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod char;
///INT (rw) register accessor: an alias for `Reg<INT_SPEC>`
pub type INT = crate::Reg<int::INT_SPEC>;
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod int;
///INTMSK (rw) register accessor: an alias for `Reg<INTMSK_SPEC>`
pub type INTMSK = crate::Reg<intmsk::INTMSK_SPEC>;
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod intmsk;
///TSIZ (rw) register accessor: an alias for `Reg<TSIZ_SPEC>`
pub type TSIZ = crate::Reg<tsiz::TSIZ_SPEC>;
///OTG_FS host channel-0 transfer size register
pub mod tsiz;
