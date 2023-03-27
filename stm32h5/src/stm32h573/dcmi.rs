///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DCMI control register
    pub cr: CR,
    ///0x04 - DCMI status register
    pub sr: SR,
    ///0x08 - DCMI raw interrupt status register
    pub ris: RIS,
    ///0x0c - DCMI interrupt enable register
    pub ier: IER,
    ///0x10 - DCMI masked interrupt status register
    pub mis: MIS,
    ///0x14 - DCMI interrupt clear register
    pub icr: ICR,
    ///0x18 - DCMI embedded synchronization code register
    pub escr: ESCR,
    ///0x1c - DCMI embedded synchronization unmask register
    pub esur: ESUR,
    ///0x20 - DCMI crop window start
    pub cwstrt: CWSTRT,
    ///0x24 - DCMI crop window size
    pub cwsize: CWSIZE,
    ///0x28 - DCMI data register
    pub dr: DR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DCMI control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///DCMI status register
pub mod sr;
///RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`
pub type RIS = crate::Reg<ris::RIS_SPEC>;
///DCMI raw interrupt status register
pub mod ris;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///DCMI interrupt enable register
pub mod ier;
///MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`
pub type MIS = crate::Reg<mis::MIS_SPEC>;
///DCMI masked interrupt status register
pub mod mis;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///DCMI interrupt clear register
pub mod icr;
///ESCR (rw) register accessor: an alias for `Reg<ESCR_SPEC>`
pub type ESCR = crate::Reg<escr::ESCR_SPEC>;
///DCMI embedded synchronization code register
pub mod escr;
///ESUR (rw) register accessor: an alias for `Reg<ESUR_SPEC>`
pub type ESUR = crate::Reg<esur::ESUR_SPEC>;
///DCMI embedded synchronization unmask register
pub mod esur;
///CWSTRT (rw) register accessor: an alias for `Reg<CWSTRT_SPEC>`
pub type CWSTRT = crate::Reg<cwstrt::CWSTRT_SPEC>;
///DCMI crop window start
pub mod cwstrt;
///CWSIZE (rw) register accessor: an alias for `Reg<CWSIZE_SPEC>`
pub type CWSIZE = crate::Reg<cwsize::CWSIZE_SPEC>;
///DCMI crop window size
pub mod cwsize;
///DR (r) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///DCMI data register
pub mod dr;
