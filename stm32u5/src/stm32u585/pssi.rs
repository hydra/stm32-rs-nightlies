///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - PSSI control register
    pub cr: CR,
    ///0x04 - PSSI status register
    pub sr: SR,
    ///0x08 - PSSI raw interrupt status register
    pub ris: RIS,
    ///0x0c - PSSI interrupt enable register
    pub ier: IER,
    ///0x10 - PSSI masked interrupt status register
    pub mis: MIS,
    ///0x14 - PSSI interrupt clear register
    pub icr: ICR,
    _reserved6: [u8; 0x10],
    ///0x28 - PSSI data register
    pub dr: DR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///PSSI control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///PSSI status register
pub mod sr;
///RIS (r) register accessor: an alias for `Reg<RIS_SPEC>`
pub type RIS = crate::Reg<ris::RIS_SPEC>;
///PSSI raw interrupt status register
pub mod ris;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///PSSI interrupt enable register
pub mod ier;
///MIS (r) register accessor: an alias for `Reg<MIS_SPEC>`
pub type MIS = crate::Reg<mis::MIS_SPEC>;
///PSSI masked interrupt status register
pub mod mis;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///PSSI interrupt clear register
pub mod icr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///PSSI data register
pub mod dr;
