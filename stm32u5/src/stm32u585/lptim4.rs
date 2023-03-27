///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt and Status Register
    pub isr: ISR,
    ///0x04 - Interrupt Clear Register
    pub icr: ICR,
    ///0x08 - LPTIM interrupt Enable Register
    pub dier: DIER,
    ///0x0c - Configuration Register
    pub cfgr: CFGR,
    ///0x10 - Control Register
    pub cr: CR,
    ///0x14 - Compare Register
    pub ccr1: CCR1,
    ///0x18 - Autoreload Register
    pub arr: ARR,
    ///0x1c - Counter Register
    pub cnt: CNT,
    _reserved8: [u8; 0x04],
    ///0x24 - LPTIM configuration register 2
    pub cfgr2: CFGR2,
    ///0x28 - LPTIM repetition register
    pub rcr: RCR,
    ///0x2c - LPTIM capture/compare mode register 1
    pub ccmr1: CCMR1,
    _reserved11: [u8; 0x04],
    ///0x34 - LPTIM Compare Register 2
    pub ccr2: CCR2,
    _reserved12: [u8; 0x03b4],
    ///0x3ec - LPTIM peripheral hardware configuration register 2
    pub hwcfgr2: HWCFGR2,
    ///0x3f0 - LPTIM peripheral hardware configuration register 1
    pub hwcfgr1: HWCFGR1,
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt and Status Register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt Clear Register
pub mod icr;
///DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`
pub type DIER = crate::Reg<dier::DIER_SPEC>;
///LPTIM interrupt Enable Register
pub mod dier;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///Configuration Register
pub mod cfgr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control Register
pub mod cr;
///CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///Compare Register
pub mod ccr1;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///Autoreload Register
pub mod arr;
///CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///Counter Register
pub mod cnt;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///LPTIM configuration register 2
pub mod cfgr2;
///RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
///LPTIM repetition register
pub mod rcr;
///CCMR1 (rw) register accessor: an alias for `Reg<CCMR1_SPEC>`
pub type CCMR1 = crate::Reg<ccmr1::CCMR1_SPEC>;
///LPTIM capture/compare mode register 1
pub mod ccmr1;
///CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
///LPTIM Compare Register 2
pub mod ccr2;
///HWCFGR2 (r) register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///LPTIM peripheral hardware configuration register 2
pub mod hwcfgr2;
///HWCFGR1 (r) register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///LPTIM peripheral hardware configuration register 1
pub mod hwcfgr1;
