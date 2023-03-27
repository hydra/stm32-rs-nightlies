///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - master control register
    pub mcr: MCR,
    ///0x04 - master status register
    pub msr: MSR,
    ///0x08 - transmit status register
    pub tsr: TSR,
    ///0x0c..0x14 - receive FIFO %s register
    pub rfr: [RFR; 2],
    ///0x14 - interrupt enable register
    pub ier: IER,
    ///0x18 - interrupt enable register
    pub esr: ESR,
    ///0x1c - bit timing register
    pub btr: BTR,
    _reserved7: [u8; 0x0160],
    ///0x180..0x1b0 - CAN Transmit cluster
    pub tx: [TX; 3],
    ///0x1b0..0x1d0 - CAN Receive cluster
    pub rx: [RX; 2],
    _reserved9: [u8; 0x70],
    ///0x240..0x320 - CAN Filter Bank cluster
    pub fb: [FB; 28],
}
///MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
///master control register
pub mod mcr;
///MSR (rw) register accessor: an alias for `Reg<MSR_SPEC>`
pub type MSR = crate::Reg<msr::MSR_SPEC>;
///master status register
pub mod msr;
///TSR (rw) register accessor: an alias for `Reg<TSR_SPEC>`
pub type TSR = crate::Reg<tsr::TSR_SPEC>;
///transmit status register
pub mod tsr;
///RFR (rw) register accessor: an alias for `Reg<RFR_SPEC>`
pub type RFR = crate::Reg<rfr::RFR_SPEC>;
///receive FIFO %s register
pub mod rfr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///ESR (rw) register accessor: an alias for `Reg<ESR_SPEC>`
pub type ESR = crate::Reg<esr::ESR_SPEC>;
///interrupt enable register
pub mod esr;
///BTR (rw) register accessor: an alias for `Reg<BTR_SPEC>`
pub type BTR = crate::Reg<btr::BTR_SPEC>;
///bit timing register
pub mod btr;
///CAN Transmit cluster
pub use self::tx::TX;
///Cluster
///CAN Transmit cluster
pub mod tx;
///CAN Receive cluster
pub use self::rx::RX;
///Cluster
///CAN Receive cluster
pub mod rx;
///CAN Filter Bank cluster
pub use self::fb::FB;
///Cluster
///CAN Filter Bank cluster
pub mod fb;
