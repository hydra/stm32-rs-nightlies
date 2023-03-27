///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt and status register
    pub isr: ISR,
    ///0x04 - interrupt enable register
    pub ier: IER,
    ///0x08 - control register
    pub cr: CR,
    ///0x0c - configuration register 1
    pub cfgr1: CFGR1,
    ///0x10 - configuration register 2
    pub cfgr2: CFGR2,
    ///0x14 - sampling time register
    pub smpr: SMPR,
    _reserved6: [u8; 0x08],
    ///0x20 - watchdog threshold register
    pub tr: TR,
    _reserved7: [u8; 0x04],
    ///0x28 - channel selection register
    pub chselr: CHSELR,
    _reserved8: [u8; 0x14],
    ///0x40 - data register
    pub dr: DR,
    _reserved9: [u8; 0x02c4],
    ///0x308 - common configuration register
    pub ccr: CCR,
}
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt and status register
pub mod isr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///configuration register 1
pub mod cfgr1;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///configuration register 2
pub mod cfgr2;
///SMPR (rw) register accessor: an alias for `Reg<SMPR_SPEC>`
pub type SMPR = crate::Reg<smpr::SMPR_SPEC>;
///sampling time register
pub mod smpr;
///TR (rw) register accessor: an alias for `Reg<TR_SPEC>`
pub type TR = crate::Reg<tr::TR_SPEC>;
///watchdog threshold register
pub mod tr;
///CHSELR (rw) register accessor: an alias for `Reg<CHSELR_SPEC>`
pub type CHSELR = crate::Reg<chselr::CHSELR_SPEC>;
///channel selection register
pub mod chselr;
///DR (r) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///data register
pub mod dr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///common configuration register
pub mod ccr;
