///Register block
#[repr(C)]
pub struct TX {
    ///0x00 - CAN_TI0R
    pub tir: TIR,
    ///0x04 - CAN_TDT0R
    pub tdtr: TDTR,
    ///0x08 - CAN_TDL0R
    pub tdlr: TDLR,
    ///0x0c - CAN_TDH0R
    pub tdhr: TDHR,
}
///TIR (rw) register accessor: an alias for `Reg<TIR_SPEC>`
pub type TIR = crate::Reg<tir::TIR_SPEC>;
///CAN_TI0R
pub mod tir;
///TDTR (rw) register accessor: an alias for `Reg<TDTR_SPEC>`
pub type TDTR = crate::Reg<tdtr::TDTR_SPEC>;
///CAN_TDT0R
pub mod tdtr;
///TDLR (rw) register accessor: an alias for `Reg<TDLR_SPEC>`
pub type TDLR = crate::Reg<tdlr::TDLR_SPEC>;
///CAN_TDL0R
pub mod tdlr;
///TDHR (rw) register accessor: an alias for `Reg<TDHR_SPEC>`
pub type TDHR = crate::Reg<tdhr::TDHR_SPEC>;
///CAN_TDH0R
pub mod tdhr;
