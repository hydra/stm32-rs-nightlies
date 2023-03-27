///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 -
    pub spi_cr1: SPI_CR1,
    ///0x04 -
    pub spi_cr2: SPI_CR2,
    ///0x08 - SPI configuration register 1
    pub spi_cfg1: SPI_CFG1,
    ///0x0c - SPI configuration register 2
    pub spi_cfg2: SPI_CFG2,
    ///0x10 -
    pub spi_ier: SPI_IER,
    ///0x14 -
    pub spi_sr: SPI_SR,
    ///0x18 -
    pub spi_ifcr: SPI_IFCR,
    ///0x1c -
    pub spi_autocr: SPI_AUTOCR,
    ///0x20 -
    pub spi_txdr: SPI_TXDR,
    _reserved9: [u8; 0x0c],
    ///0x30 -
    pub spi_rxdr: SPI_RXDR,
    _reserved10: [u8; 0x0c],
    ///0x40 - SPI polynomial register
    pub spi_crcpoly: SPI_CRCPOLY,
    ///0x44 -
    pub spi_txcrc: SPI_TXCRC,
    ///0x48 -
    pub spi_rxcrc: SPI_RXCRC,
    ///0x4c - SPI underrun data register
    pub spi_udrdr: SPI_UDRDR,
}
///SPI_CR1 (rw) register accessor: an alias for `Reg<SPI_CR1_SPEC>`
pub type SPI_CR1 = crate::Reg<spi_cr1::SPI_CR1_SPEC>;
///
pub mod spi_cr1;
///SPI_CR2 (rw) register accessor: an alias for `Reg<SPI_CR2_SPEC>`
pub type SPI_CR2 = crate::Reg<spi_cr2::SPI_CR2_SPEC>;
///
pub mod spi_cr2;
///SPI_CFG1 (rw) register accessor: an alias for `Reg<SPI_CFG1_SPEC>`
pub type SPI_CFG1 = crate::Reg<spi_cfg1::SPI_CFG1_SPEC>;
///SPI configuration register 1
pub mod spi_cfg1;
///SPI_CFG2 (rw) register accessor: an alias for `Reg<SPI_CFG2_SPEC>`
pub type SPI_CFG2 = crate::Reg<spi_cfg2::SPI_CFG2_SPEC>;
///SPI configuration register 2
pub mod spi_cfg2;
///SPI_IER (rw) register accessor: an alias for `Reg<SPI_IER_SPEC>`
pub type SPI_IER = crate::Reg<spi_ier::SPI_IER_SPEC>;
///
pub mod spi_ier;
///SPI_SR (r) register accessor: an alias for `Reg<SPI_SR_SPEC>`
pub type SPI_SR = crate::Reg<spi_sr::SPI_SR_SPEC>;
///
pub mod spi_sr;
///SPI_IFCR (w) register accessor: an alias for `Reg<SPI_IFCR_SPEC>`
pub type SPI_IFCR = crate::Reg<spi_ifcr::SPI_IFCR_SPEC>;
///
pub mod spi_ifcr;
///SPI_AUTOCR (rw) register accessor: an alias for `Reg<SPI_AUTOCR_SPEC>`
pub type SPI_AUTOCR = crate::Reg<spi_autocr::SPI_AUTOCR_SPEC>;
///
pub mod spi_autocr;
///SPI_TXDR (w) register accessor: an alias for `Reg<SPI_TXDR_SPEC>`
pub type SPI_TXDR = crate::Reg<spi_txdr::SPI_TXDR_SPEC>;
///
pub mod spi_txdr;
///SPI_RXDR (r) register accessor: an alias for `Reg<SPI_RXDR_SPEC>`
pub type SPI_RXDR = crate::Reg<spi_rxdr::SPI_RXDR_SPEC>;
///
pub mod spi_rxdr;
///SPI_CRCPOLY (rw) register accessor: an alias for `Reg<SPI_CRCPOLY_SPEC>`
pub type SPI_CRCPOLY = crate::Reg<spi_crcpoly::SPI_CRCPOLY_SPEC>;
///SPI polynomial register
pub mod spi_crcpoly;
///SPI_TXCRC (r) register accessor: an alias for `Reg<SPI_TXCRC_SPEC>`
pub type SPI_TXCRC = crate::Reg<spi_txcrc::SPI_TXCRC_SPEC>;
///
pub mod spi_txcrc;
///SPI_RXCRC (r) register accessor: an alias for `Reg<SPI_RXCRC_SPEC>`
pub type SPI_RXCRC = crate::Reg<spi_rxcrc::SPI_RXCRC_SPEC>;
///
pub mod spi_rxcrc;
///SPI_UDRDR (rw) register accessor: an alias for `Reg<SPI_UDRDR_SPEC>`
pub type SPI_UDRDR = crate::Reg<spi_udrdr::SPI_UDRDR_SPEC>;
///SPI underrun data register
pub mod spi_udrdr;
