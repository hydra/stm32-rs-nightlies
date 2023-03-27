///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Cache Level ID register
    pub clidr: CLIDR,
    ///0x04 - Cache Type register
    pub ctr: CTR,
    ///0x08 - Cache Size ID register
    pub ccsidr: CCSIDR,
}
///CLIDR (r) register accessor: an alias for `Reg<CLIDR_SPEC>`
pub type CLIDR = crate::Reg<clidr::CLIDR_SPEC>;
///Cache Level ID register
pub mod clidr;
///CTR (r) register accessor: an alias for `Reg<CTR_SPEC>`
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
///Cache Type register
pub mod ctr;
///CCSIDR (r) register accessor: an alias for `Reg<CCSIDR_SPEC>`
pub type CCSIDR = crate::Reg<ccsidr::CCSIDR_SPEC>;
///Cache Size ID register
pub mod ccsidr;
