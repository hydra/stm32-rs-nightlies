///Register block
#[repr(C)]
pub struct TX {
    ///0x00 - TX mailbox identifier register
    pub tir: TIR,
    ///0x04 - mailbox data length control and time stamp register
    pub tdtr: TDTR,
    ///0x08 - mailbox data low register
    pub tdlr: TDLR,
    ///0x0c - mailbox data high register
    pub tdhr: TDHR,
}
///TIR (rw) register accessor: an alias for `Reg<TIR_SPEC>`
pub type TIR = crate::Reg<tir::TIR_SPEC>;
///TX mailbox identifier register
pub mod tir;
///TDTR (rw) register accessor: an alias for `Reg<TDTR_SPEC>`
pub type TDTR = crate::Reg<tdtr::TDTR_SPEC>;
///mailbox data length control and time stamp register
pub mod tdtr;
///TDLR (rw) register accessor: an alias for `Reg<TDLR_SPEC>`
pub type TDLR = crate::Reg<tdlr::TDLR_SPEC>;
///mailbox data low register
pub mod tdlr;
///TDHR (rw) register accessor: an alias for `Reg<TDHR_SPEC>`
pub type TDHR = crate::Reg<tdhr::TDHR_SPEC>;
///mailbox data high register
pub mod tdhr;
