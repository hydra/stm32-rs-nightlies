///Register block
#[repr(C)]
pub struct RX {
    ///0x00 - receive FIFO mailbox identifier register
    pub rir: RIR,
    ///0x04 - mailbox data high register
    pub rdtr: RDTR,
    ///0x08 - mailbox data high register
    pub rdlr: RDLR,
    ///0x0c - receive FIFO mailbox data high register
    pub rdhr: RDHR,
}
///RIR (r) register accessor: an alias for `Reg<RIR_SPEC>`
pub type RIR = crate::Reg<rir::RIR_SPEC>;
///receive FIFO mailbox identifier register
pub mod rir;
///RDTR (r) register accessor: an alias for `Reg<RDTR_SPEC>`
pub type RDTR = crate::Reg<rdtr::RDTR_SPEC>;
///mailbox data high register
pub mod rdtr;
///RDLR (r) register accessor: an alias for `Reg<RDLR_SPEC>`
pub type RDLR = crate::Reg<rdlr::RDLR_SPEC>;
///mailbox data high register
pub mod rdlr;
///RDHR (r) register accessor: an alias for `Reg<RDHR_SPEC>`
pub type RDHR = crate::Reg<rdhr::RDHR_SPEC>;
///receive FIFO mailbox data high register
pub mod rdhr;
