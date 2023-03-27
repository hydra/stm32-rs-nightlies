///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CRC data register
    pub dr: DR,
    ///0x04 - CRC independent data register
    pub idr: IDR,
    ///0x08 - CRC control register
    pub cr: CR,
    _reserved3: [u8; 0x04],
    ///0x10 - CRC initial value
    pub init: INIT,
    ///0x14 - CRC polynomial
    pub pol: POL,
}
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///CRC data register
pub mod dr;
///IDR (rw) register accessor: an alias for `Reg<IDR_SPEC>`
pub type IDR = crate::Reg<idr::IDR_SPEC>;
///CRC independent data register
pub mod idr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///CRC control register
pub mod cr;
///INIT (rw) register accessor: an alias for `Reg<INIT_SPEC>`
pub type INIT = crate::Reg<init::INIT_SPEC>;
///CRC initial value
pub mod init;
///POL (rw) register accessor: an alias for `Reg<POL_SPEC>`
pub type POL = crate::Reg<pol::POL_SPEC>;
///CRC polynomial
pub mod pol;
