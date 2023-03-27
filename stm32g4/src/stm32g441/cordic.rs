///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CORDIC Control Status register
    pub csr: CSR,
    ///0x04 - CORDIC argument register
    pub wdata: WDATA,
    ///0x08 - CORDIC result register
    pub rdata: RDATA,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///CORDIC Control Status register
pub mod csr;
///WDATA (rw) register accessor: an alias for `Reg<WDATA_SPEC>`
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
///CORDIC argument register
pub mod wdata;
///RDATA (r) register accessor: an alias for `Reg<RDATA_SPEC>`
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
///CORDIC result register
pub mod rdata;
