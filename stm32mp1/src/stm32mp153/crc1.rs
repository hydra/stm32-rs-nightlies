///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CRC data register
    pub crc_dr: CRC_DR,
    ///0x04 - CRC independent data register
    pub crc_idr: CRC_IDR,
    ///0x08 - CRC control register
    pub crc_cr: CRC_CR,
    _reserved3: [u8; 0x04],
    ///0x10 - CRC initial value
    pub crc_init: CRC_INIT,
    ///0x14 - CRC polynomial
    pub crc_pol: CRC_POL,
}
///CRC_DR (rw) register accessor: an alias for `Reg<CRC_DR_SPEC>`
pub type CRC_DR = crate::Reg<crc_dr::CRC_DR_SPEC>;
///CRC data register
pub mod crc_dr;
///CRC_IDR (rw) register accessor: an alias for `Reg<CRC_IDR_SPEC>`
pub type CRC_IDR = crate::Reg<crc_idr::CRC_IDR_SPEC>;
///CRC independent data register
pub mod crc_idr;
///CRC_CR (rw) register accessor: an alias for `Reg<CRC_CR_SPEC>`
pub type CRC_CR = crate::Reg<crc_cr::CRC_CR_SPEC>;
///CRC control register
pub mod crc_cr;
///CRC_INIT (rw) register accessor: an alias for `Reg<CRC_INIT_SPEC>`
pub type CRC_INIT = crate::Reg<crc_init::CRC_INIT_SPEC>;
///CRC initial value
pub mod crc_init;
///CRC_POL (rw) register accessor: an alias for `Reg<CRC_POL_SPEC>`
pub type CRC_POL = crate::Reg<crc_pol::CRC_POL_SPEC>;
///CRC polynomial
pub mod crc_pol;
