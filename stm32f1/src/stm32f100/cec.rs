///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - configuration register
    pub cfgr: CFGR,
    ///0x04 - CEC own address register
    pub oar: OAR,
    ///0x08 - Rx Data Register
    pub pres: PRES,
    ///0x0c - CEC error status register
    pub esr: ESR,
    ///0x10 - CEC control and status register
    pub csr: CSR,
    ///0x14 - CEC Tx data register
    pub txd: TXD,
    ///0x18 - CEC Rx data register
    pub rxd: RXD,
}
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///configuration register
pub mod cfgr;
///OAR (rw) register accessor: an alias for `Reg<OAR_SPEC>`
pub type OAR = crate::Reg<oar::OAR_SPEC>;
///CEC own address register
pub mod oar;
///PRES (rw) register accessor: an alias for `Reg<PRES_SPEC>`
pub type PRES = crate::Reg<pres::PRES_SPEC>;
///Rx Data Register
pub mod pres;
///ESR (r) register accessor: an alias for `Reg<ESR_SPEC>`
pub type ESR = crate::Reg<esr::ESR_SPEC>;
///CEC error status register
pub mod esr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///CEC control and status register
pub mod csr;
///TXD (rw) register accessor: an alias for `Reg<TXD_SPEC>`
pub type TXD = crate::Reg<txd::TXD_SPEC>;
///CEC Tx data register
pub mod txd;
///RXD (r) register accessor: an alias for `Reg<RXD_SPEC>`
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
///CEC Rx data register
pub mod rxd;
