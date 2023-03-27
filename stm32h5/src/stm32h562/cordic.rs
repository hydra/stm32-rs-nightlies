///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CORDIC control/status register
    pub cordic_csr: CORDIC_CSR,
    ///0x04 - CORDIC argument register
    pub cordic_wdata: CORDIC_WDATA,
    ///0x08 - CORDIC result register
    pub cordic_rdata: CORDIC_RDATA,
}
///CORDIC_CSR (rw) register accessor: an alias for `Reg<CORDIC_CSR_SPEC>`
pub type CORDIC_CSR = crate::Reg<cordic_csr::CORDIC_CSR_SPEC>;
///CORDIC control/status register
pub mod cordic_csr;
///CORDIC_WDATA (w) register accessor: an alias for `Reg<CORDIC_WDATA_SPEC>`
pub type CORDIC_WDATA = crate::Reg<cordic_wdata::CORDIC_WDATA_SPEC>;
///CORDIC argument register
pub mod cordic_wdata;
///CORDIC_RDATA (r) register accessor: an alias for `Reg<CORDIC_RDATA_SPEC>`
pub type CORDIC_RDATA = crate::Reg<cordic_rdata::CORDIC_RDATA_SPEC>;
///CORDIC result register
pub mod cordic_rdata;
