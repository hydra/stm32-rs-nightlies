///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x20 - endpoint %s register
    pub epr: [EPR; 8],
    _reserved1: [u8; 0x20],
    ///0x40 - control register
    pub cntr: CNTR,
    ///0x44 - interrupt status register
    pub istr: ISTR,
    ///0x48 - frame number register
    pub fnr: FNR,
    ///0x4c - device address
    pub daddr: DADDR,
    ///0x50 - Buffer table address
    pub btable: BTABLE,
}
///EPR (rw) register accessor: an alias for `Reg<EPR_SPEC>`
pub type EPR = crate::Reg<epr::EPR_SPEC>;
///endpoint %s register
pub mod epr;
///CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
///control register
pub mod cntr;
///ISTR (rw) register accessor: an alias for `Reg<ISTR_SPEC>`
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
///interrupt status register
pub mod istr;
///FNR (r) register accessor: an alias for `Reg<FNR_SPEC>`
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
///frame number register
pub mod fnr;
///DADDR (rw) register accessor: an alias for `Reg<DADDR_SPEC>`
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
///device address
pub mod daddr;
///BTABLE (rw) register accessor: an alias for `Reg<BTABLE_SPEC>`
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
///Buffer table address
pub mod btable;
