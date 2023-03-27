///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control and status register
    pub csr: CSR,
    ///0x04 - Argument register
    pub wdata: WDATA,
    ///0x08 - Result register
    pub rdata: RDATA,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Control and status register
pub mod csr;
///WDATA (rw) register accessor: an alias for `Reg<WDATA_SPEC>`
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
///Argument register
pub mod wdata;
///RDATA (rw) register accessor: an alias for `Reg<RDATA_SPEC>`
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
///Result register
pub mod rdata;
