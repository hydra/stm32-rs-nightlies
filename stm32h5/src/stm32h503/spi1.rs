///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SPI/I2S control register 1
    pub cr1: CR1,
    ///0x04 - SPI/I2S control register 2
    pub cr2: CR2,
    ///0x08 - SPI/I2S configuration register 1
    pub cfg1: CFG1,
    ///0x0c - SPI/I2S configuration register 2
    pub cfg2: CFG2,
    ///0x10 - SPI/I2S interrupt enable register
    pub ier: IER,
    ///0x14 - SPI/I2S status register
    pub sr: SR,
    ///0x18 - SPI/I2S interrupt/status flags clear register
    pub ifcr: IFCR,
    _reserved7: [u8; 0x04],
    ///0x20 - SPI/I2S transmit data register
    pub txdr: TXDR,
    _reserved8: [u8; 0x0c],
    ///0x30 - SPI/I2S receive data register
    pub rxdr: RXDR,
    _reserved9: [u8; 0x0c],
    ///0x40 - SPI/I2S polynomial register
    pub crcpoly: CRCPOLY,
    ///0x44 - SPI/I2S transmitter CRC register
    pub txcrc: TXCRC,
    ///0x48 - SPI/I2S receiver CRC register
    pub rxcrc: RXCRC,
    ///0x4c - SPI/I2S underrun data register
    pub udrdr: UDRDR,
    ///0x50 - SPI/I2S configuration register
    pub i2scfgr: I2SCFGR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///SPI/I2S control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///SPI/I2S control register 2
pub mod cr2;
///CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
///SPI/I2S configuration register 1
pub mod cfg1;
///CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
///SPI/I2S configuration register 2
pub mod cfg2;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///SPI/I2S interrupt enable register
pub mod ier;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///SPI/I2S status register
pub mod sr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///SPI/I2S interrupt/status flags clear register
pub mod ifcr;
///TXDR (w) register accessor: an alias for `Reg<TXDR_SPEC>`
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
///SPI/I2S transmit data register
pub mod txdr;
///RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
///SPI/I2S receive data register
pub mod rxdr;
///CRCPOLY (rw) register accessor: an alias for `Reg<CRCPOLY_SPEC>`
pub type CRCPOLY = crate::Reg<crcpoly::CRCPOLY_SPEC>;
///SPI/I2S polynomial register
pub mod crcpoly;
///TXCRC (r) register accessor: an alias for `Reg<TXCRC_SPEC>`
pub type TXCRC = crate::Reg<txcrc::TXCRC_SPEC>;
///SPI/I2S transmitter CRC register
pub mod txcrc;
///RXCRC (r) register accessor: an alias for `Reg<RXCRC_SPEC>`
pub type RXCRC = crate::Reg<rxcrc::RXCRC_SPEC>;
///SPI/I2S receiver CRC register
pub mod rxcrc;
///UDRDR (rw) register accessor: an alias for `Reg<UDRDR_SPEC>`
pub type UDRDR = crate::Reg<udrdr::UDRDR_SPEC>;
///SPI/I2S underrun data register
pub mod udrdr;
///I2SCFGR (rw) register accessor: an alias for `Reg<I2SCFGR_SPEC>`
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
///SPI/I2S configuration register
pub mod i2scfgr;
