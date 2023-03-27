///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CORDIC Control Status register
    pub csr: CSR,
    ///0x04 - FMAC Write Data register
    pub wdata: WDATA,
    ///0x08 - FMAC Read Data register
    pub rdata: RDATA,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///CORDIC Control Status register
pub mod csr;
///WDATA (w) register accessor: an alias for `Reg<WDATA_SPEC>`
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
///FMAC Write Data register
pub mod wdata;
///RDATA (r) register accessor: an alias for `Reg<RDATA_SPEC>`
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
///FMAC Read Data register
pub mod rdata;
