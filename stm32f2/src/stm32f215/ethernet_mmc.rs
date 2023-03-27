///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Ethernet MMC control register
    pub mmccr: MMCCR,
    ///0x04 - Ethernet MMC receive interrupt register
    pub mmcrir: MMCRIR,
    ///0x08 - Ethernet MMC transmit interrupt register
    pub mmctir: MMCTIR,
    ///0x0c - Ethernet MMC receive interrupt mask register
    pub mmcrimr: MMCRIMR,
    ///0x10 - Ethernet MMC transmit interrupt mask register
    pub mmctimr: MMCTIMR,
    _reserved5: [u8; 0x38],
    ///0x4c - Ethernet MMC transmitted good frames after a single collision counter
    pub mmctgfsccr: MMCTGFSCCR,
    ///0x50 - Ethernet MMC transmitted good frames after more than a single collision
    pub mmctgfmsccr: MMCTGFMSCCR,
    _reserved7: [u8; 0x14],
    ///0x68 - Ethernet MMC transmitted good frames counter register
    pub mmctgfcr: MMCTGFCR,
    _reserved8: [u8; 0x28],
    ///0x94 - Ethernet MMC received frames with CRC error counter register
    pub mmcrfcecr: MMCRFCECR,
    ///0x98 - Ethernet MMC received frames with alignment error counter register
    pub mmcrfaecr: MMCRFAECR,
    _reserved10: [u8; 0x28],
    ///0xc4 - MMC received good unicast frames counter register
    pub mmcrgufcr: MMCRGUFCR,
}
///MMCCR (rw) register accessor: an alias for `Reg<MMCCR_SPEC>`
pub type MMCCR = crate::Reg<mmccr::MMCCR_SPEC>;
///Ethernet MMC control register
pub mod mmccr;
///MMCRIR (rw) register accessor: an alias for `Reg<MMCRIR_SPEC>`
pub type MMCRIR = crate::Reg<mmcrir::MMCRIR_SPEC>;
///Ethernet MMC receive interrupt register
pub mod mmcrir;
///MMCTIR (rw) register accessor: an alias for `Reg<MMCTIR_SPEC>`
pub type MMCTIR = crate::Reg<mmctir::MMCTIR_SPEC>;
///Ethernet MMC transmit interrupt register
pub mod mmctir;
///MMCRIMR (rw) register accessor: an alias for `Reg<MMCRIMR_SPEC>`
pub type MMCRIMR = crate::Reg<mmcrimr::MMCRIMR_SPEC>;
///Ethernet MMC receive interrupt mask register
pub mod mmcrimr;
///MMCTIMR (rw) register accessor: an alias for `Reg<MMCTIMR_SPEC>`
pub type MMCTIMR = crate::Reg<mmctimr::MMCTIMR_SPEC>;
///Ethernet MMC transmit interrupt mask register
pub mod mmctimr;
///MMCTGFSCCR (r) register accessor: an alias for `Reg<MMCTGFSCCR_SPEC>`
pub type MMCTGFSCCR = crate::Reg<mmctgfsccr::MMCTGFSCCR_SPEC>;
///Ethernet MMC transmitted good frames after a single collision counter
pub mod mmctgfsccr;
///MMCTGFMSCCR (r) register accessor: an alias for `Reg<MMCTGFMSCCR_SPEC>`
pub type MMCTGFMSCCR = crate::Reg<mmctgfmsccr::MMCTGFMSCCR_SPEC>;
///Ethernet MMC transmitted good frames after more than a single collision
pub mod mmctgfmsccr;
///MMCTGFCR (r) register accessor: an alias for `Reg<MMCTGFCR_SPEC>`
pub type MMCTGFCR = crate::Reg<mmctgfcr::MMCTGFCR_SPEC>;
///Ethernet MMC transmitted good frames counter register
pub mod mmctgfcr;
///MMCRFCECR (r) register accessor: an alias for `Reg<MMCRFCECR_SPEC>`
pub type MMCRFCECR = crate::Reg<mmcrfcecr::MMCRFCECR_SPEC>;
///Ethernet MMC received frames with CRC error counter register
pub mod mmcrfcecr;
///MMCRFAECR (r) register accessor: an alias for `Reg<MMCRFAECR_SPEC>`
pub type MMCRFAECR = crate::Reg<mmcrfaecr::MMCRFAECR_SPEC>;
///Ethernet MMC received frames with alignment error counter register
pub mod mmcrfaecr;
///MMCRGUFCR (r) register accessor: an alias for `Reg<MMCRGUFCR_SPEC>`
pub type MMCRGUFCR = crate::Reg<mmcrgufcr::MMCRGUFCR_SPEC>;
///MMC received good unicast frames counter register
pub mod mmcrgufcr;
