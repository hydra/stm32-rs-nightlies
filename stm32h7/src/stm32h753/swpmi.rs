///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SWPMI Configuration/Control register
    pub cr: CR,
    ///0x04 - SWPMI Bitrate register
    pub brr: BRR,
    _reserved2: [u8; 0x04],
    ///0x0c - SWPMI Interrupt and Status register
    pub isr: ISR,
    ///0x10 - SWPMI Interrupt Flag Clear register
    pub icr: ICR,
    ///0x14 - SWPMI Interrupt Enable register
    pub ier: IER,
    ///0x18 - SWPMI Receive Frame Length register
    pub rfl: RFL,
    ///0x1c - SWPMI Transmit data register
    pub tdr: TDR,
    ///0x20 - SWPMI Receive data register
    pub rdr: RDR,
    ///0x24 - SWPMI Option register
    pub or: OR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///SWPMI Configuration/Control register
pub mod cr;
///BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///SWPMI Bitrate register
pub mod brr;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///SWPMI Interrupt and Status register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///SWPMI Interrupt Flag Clear register
pub mod icr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///SWPMI Interrupt Enable register
pub mod ier;
///RFL (r) register accessor: an alias for `Reg<RFL_SPEC>`
pub type RFL = crate::Reg<rfl::RFL_SPEC>;
///SWPMI Receive Frame Length register
pub mod rfl;
///TDR (w) register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///SWPMI Transmit data register
pub mod tdr;
///RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///SWPMI Receive data register
pub mod rdr;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///SWPMI Option register
pub mod or;
