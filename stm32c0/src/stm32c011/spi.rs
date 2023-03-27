///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SPI control register 1
    pub cr1: CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - SPI control register 2
    pub cr2: CR2,
    _reserved2: [u8; 0x02],
    ///0x08 - SPI status register
    pub sr: SR,
    _reserved3: [u8; 0x02],
    ///0x0c - SPI data register
    pub dr: DR,
    _reserved4: [u8; 0x02],
    ///0x10 - SPI CRC polynomial register
    pub crcpr: CRCPR,
    _reserved5: [u8; 0x02],
    ///0x14 - SPI Rx CRC register
    pub rxcrcr: RXCRCR,
    _reserved6: [u8; 0x02],
    ///0x18 - SPI Tx CRC register
    pub txcrcr: TXCRCR,
    _reserved7: [u8; 0x02],
    ///0x1c - SPI_I2S configuration register
    pub i2scfgr: I2SCFGR,
    _reserved8: [u8; 0x02],
    ///0x20 - SPI_I2S prescaler register
    pub i2spr: I2SPR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///SPI control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///SPI control register 2
pub mod cr2;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///SPI status register
pub mod sr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///SPI data register
pub mod dr;
///CRCPR (rw) register accessor: an alias for `Reg<CRCPR_SPEC>`
pub type CRCPR = crate::Reg<crcpr::CRCPR_SPEC>;
///SPI CRC polynomial register
pub mod crcpr;
///RXCRCR (r) register accessor: an alias for `Reg<RXCRCR_SPEC>`
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCR_SPEC>;
///SPI Rx CRC register
pub mod rxcrcr;
///TXCRCR (r) register accessor: an alias for `Reg<TXCRCR_SPEC>`
pub type TXCRCR = crate::Reg<txcrcr::TXCRCR_SPEC>;
///SPI Tx CRC register
pub mod txcrcr;
///I2SCFGR (rw) register accessor: an alias for `Reg<I2SCFGR_SPEC>`
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
///SPI_I2S configuration register
pub mod i2scfgr;
///I2SPR (rw) register accessor: an alias for `Reg<I2SPR_SPEC>`
pub type I2SPR = crate::Reg<i2spr::I2SPR_SPEC>;
///SPI_I2S prescaler register
pub mod i2spr;
