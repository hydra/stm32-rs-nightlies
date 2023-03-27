///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - configuration register
    pub cfgr: CFGR,
    ///0x08 - Tx data register
    pub txdr: TXDR,
    ///0x0c - Rx Data Register
    pub rxdr: RXDR,
    ///0x10 - Interrupt and Status Register
    pub isr: ISR,
    ///0x14 - interrupt enable register
    pub ier: IER,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///configuration register
pub mod cfgr;
///TXDR (w) register accessor: an alias for `Reg<TXDR_SPEC>`
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
///Tx data register
pub mod txdr;
///RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
///Rx Data Register
pub mod rxdr;
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt and Status Register
pub mod isr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
