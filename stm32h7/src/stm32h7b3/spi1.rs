///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - configuration register 1
    pub cfg1: CFG1,
    ///0x0c - configuration register 2
    pub cfg2: CFG2,
    ///0x10 - Interrupt Enable Register
    pub ier: IER,
    ///0x14 - Status Register
    pub sr: SR,
    ///0x18 - Interrupt/Status Flags Clear Register
    pub ifcr: IFCR,
    _reserved7: [u8; 0x04],
    ///0x20 - Transmit Data Register
    pub txdr: TXDR,
    _reserved8: [u8; 0x0c],
    ///0x30 - Receive Data Register
    pub rxdr: RXDR,
    _reserved9: [u8; 0x0c],
    ///0x40 - Polynomial Register
    pub crcpoly: CRCPOLY,
    ///0x44 - Transmitter CRC Register
    pub txcrc: TXCRC,
    ///0x48 - Receiver CRC Register
    pub rxcrc: RXCRC,
    ///0x4c - Underrun Data Register
    pub udrdr: UDRDR,
    ///0x50 - configuration register
    pub i2scfgr: I2SCFGR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
///configuration register 1
pub mod cfg1;
///CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
///configuration register 2
pub mod cfg2;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///Interrupt Enable Register
pub mod ier;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status Register
pub mod sr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///Interrupt/Status Flags Clear Register
pub mod ifcr;
///TXDR (w) register accessor: an alias for `Reg<TXDR_SPEC>`
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
///Transmit Data Register
pub mod txdr;
///RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
///Receive Data Register
pub mod rxdr;
///CRCPOLY (rw) register accessor: an alias for `Reg<CRCPOLY_SPEC>`
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
///Polynomial Register
pub mod crcpoly;
///TXCRC (rw) register accessor: an alias for `Reg<TXCRC_SPEC>`
pub type TXCRC = crate::Reg<txcrc::TXCRC_SPEC>;
///Transmitter CRC Register
pub mod txcrc;
///RXCRC (rw) register accessor: an alias for `Reg<RXCRC_SPEC>`
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
///Receiver CRC Register
pub mod rxcrc;
///UDRDR (rw) register accessor: an alias for `Reg<UDRDR_SPEC>`
pub type UDRDR = crate::Reg<udrdr::UDRDR_SPEC>;
///Underrun Data Register
pub mod udrdr;
///I2SCFGR (rw) register accessor: an alias for `Reg<I2SCFGR_SPEC>`
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
///configuration register
pub mod i2scfgr;
