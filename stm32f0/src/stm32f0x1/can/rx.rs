///Register block
#[repr(C)]
pub struct RX {
    ///0x00 - CAN_RI0R
    pub rir: RIR,
    ///0x04 - CAN_RDT0R
    pub rdtr: RDTR,
    ///0x08 - CAN_RDL0R
    pub rdlr: RDLR,
    ///0x0c - CAN_RDH0R
    pub rdhr: RDHR,
}
///RIR (r) register accessor: an alias for `Reg<RIR_SPEC>`
pub type RIR = crate::Reg<rir::RIR_SPEC>;
///CAN_RI0R
pub mod rir;
///RDTR (r) register accessor: an alias for `Reg<RDTR_SPEC>`
pub type RDTR = crate::Reg<rdtr::RDTR_SPEC>;
///CAN_RDT0R
pub mod rdtr;
///RDLR (r) register accessor: an alias for `Reg<RDLR_SPEC>`
pub type RDLR = crate::Reg<rdlr::RDLR_SPEC>;
///CAN_RDL0R
pub mod rdlr;
///RDHR (r) register accessor: an alias for `Reg<RDHR_SPEC>`
pub type RDHR = crate::Reg<rdhr::RDHR_SPEC>;
///CAN_RDH0R
pub mod rdhr;
