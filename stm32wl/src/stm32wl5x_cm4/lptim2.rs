///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt and status register
    pub isr: ISR,
    ///0x04 - interrupt clear register
    pub icr: ICR,
    ///0x08 - interrupt enable register
    pub ier: IER,
    ///0x0c - configuration register
    pub cfgr: CFGR,
    ///0x10 - control register
    pub cr: CR,
    ///0x14 - compare register
    pub cmp: CMP,
    ///0x18 - autoreload register
    pub arr: ARR,
    ///0x1c - counter register
    pub cnt: CNT,
    ///0x20 - option register
    pub or: OR,
    _reserved9: [u8; 0x04],
    ///0x28 - repetition register
    pub rcr: RCR,
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt and status register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt clear register
pub mod icr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///configuration register
pub mod cfgr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///CMP (rw) register accessor: an alias for `Reg<CMP_SPEC>`
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
///compare register
pub mod cmp;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///autoreload register
pub mod arr;
///CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///counter register
pub mod cnt;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///option register
pub mod or;
///RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
///repetition register
pub mod rcr;
