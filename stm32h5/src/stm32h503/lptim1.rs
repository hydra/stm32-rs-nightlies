///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_icr: [u8; 0x04],
    _reserved_2_dier: [u8; 0x04],
    ///0x0c - LPTIM configuration register
    pub cfgr: CFGR,
    ///0x10 - LPTIM control register
    pub cr: CR,
    ///0x14 - LPTIM compare register 1
    pub ccr1: CCR1,
    ///0x18 - LPTIM autoreload register
    pub arr: ARR,
    ///0x1c - LPTIM counter register
    pub cnt: CNT,
    _reserved8: [u8; 0x08],
    ///0x28 - LPTIM repetition register
    pub rcr: RCR,
    ///0x2c - LPTIM capture/compare mode register 1
    pub ccmr1: CCMR1,
    _reserved10: [u8; 0x04],
    ///0x34 - LPTIM compare register 2
    pub ccr2: CCR2,
}
impl RegisterBlock {
    ///0x00 - LPTIM1 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_intput(&self) -> &ISR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x00 - LPTIM1 interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_output(&self) -> &ISR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x04 - LPTIM interrupt clear register
    #[inline(always)]
    pub const fn icr_intput(&self) -> &ICR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    ///0x04 - LPTIM1 interrupt clear register \[alternate\]
    #[inline(always)]
    pub const fn icr_output(&self) -> &ICR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    ///0x08 - LPTIM interrupt enable register
    #[inline(always)]
    pub const fn dier_intput(&self) -> &DIER_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    ///0x08 - LPTIM1 interrupt enable register \[alternate\]
    #[inline(always)]
    pub const fn dier_output(&self) -> &DIER_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
///ISR_output (r) register accessor: an alias for `Reg<ISR_OUTPUT_SPEC>`
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUT_SPEC>;
///LPTIM1 interrupt and status register \[alternate\]
pub mod isr_output;
///ISR_intput (r) register accessor: an alias for `Reg<ISR_INTPUT_SPEC>`
pub type ISR_INTPUT = crate::Reg<isr_intput::ISR_INTPUT_SPEC>;
///LPTIM1 interrupt and status register \[alternate\]
pub mod isr_intput;
///ICR_output (w) register accessor: an alias for `Reg<ICR_OUTPUT_SPEC>`
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUT_SPEC>;
///LPTIM1 interrupt clear register \[alternate\]
pub mod icr_output;
///ICR_intput (w) register accessor: an alias for `Reg<ICR_INTPUT_SPEC>`
pub type ICR_INTPUT = crate::Reg<icr_intput::ICR_INTPUT_SPEC>;
///LPTIM interrupt clear register
pub mod icr_intput;
///DIER_output (rw) register accessor: an alias for `Reg<DIER_OUTPUT_SPEC>`
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUT_SPEC>;
///LPTIM1 interrupt enable register \[alternate\]
pub mod dier_output;
///DIER_intput (rw) register accessor: an alias for `Reg<DIER_INTPUT_SPEC>`
pub type DIER_INTPUT = crate::Reg<dier_intput::DIER_INTPUT_SPEC>;
///LPTIM interrupt enable register
pub mod dier_intput;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///LPTIM configuration register
pub mod cfgr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///LPTIM control register
pub mod cr;
///CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///LPTIM compare register 1
pub mod ccr1;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///LPTIM autoreload register
pub mod arr;
///CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///LPTIM counter register
pub mod cnt;
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
///LPTIM compare register 2
pub mod ccr2;
