///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_isr: [u8; 0x04],
    _reserved_1_icr: [u8; 0x04],
    _reserved_2_dier: [u8; 0x04],
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
impl RegisterBlock {
    ///0x00 - Interrupt and Status Register (intput mode)
    #[inline(always)]
    pub const fn isr_intput(&self) -> &ISR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x00 - Interrupt and Status Register (output mode)
    #[inline(always)]
    pub const fn isr_output(&self) -> &ISR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x04 - Interrupt Clear Register (intput mode)
    #[inline(always)]
    pub const fn icr_intput(&self) -> &ICR_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    ///0x04 - Interrupt Clear Register (output mode)
    #[inline(always)]
    pub const fn icr_output(&self) -> &ICR_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    ///0x08 - LPTIM interrupt Enable Register (intput mode)
    #[inline(always)]
    pub const fn dier_intput(&self) -> &DIER_INTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    ///0x08 - LPTIM interrupt Enable Register (output mode)
    #[inline(always)]
    pub const fn dier_output(&self) -> &DIER_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
}
///ISR_output (r) register accessor: an alias for `Reg<ISR_OUTPUT_SPEC>`
pub type ISR_OUTPUT = crate::Reg<isr_output::ISR_OUTPUT_SPEC>;
///Interrupt and Status Register (output mode)
pub mod isr_output;
///ISR_intput (r) register accessor: an alias for `Reg<ISR_INTPUT_SPEC>`
pub type ISR_INTPUT = crate::Reg<isr_intput::ISR_INTPUT_SPEC>;
///Interrupt and Status Register (intput mode)
pub mod isr_intput;
///ICR_output (w) register accessor: an alias for `Reg<ICR_OUTPUT_SPEC>`
pub type ICR_OUTPUT = crate::Reg<icr_output::ICR_OUTPUT_SPEC>;
///Interrupt Clear Register (output mode)
pub mod icr_output;
///ICR_intput (w) register accessor: an alias for `Reg<ICR_INTPUT_SPEC>`
pub type ICR_INTPUT = crate::Reg<icr_intput::ICR_INTPUT_SPEC>;
///Interrupt Clear Register (intput mode)
pub mod icr_intput;
///DIER_output (rw) register accessor: an alias for `Reg<DIER_OUTPUT_SPEC>`
pub type DIER_OUTPUT = crate::Reg<dier_output::DIER_OUTPUT_SPEC>;
///LPTIM interrupt Enable Register (output mode)
pub mod dier_output;
///DIER_intput (rw) register accessor: an alias for `Reg<DIER_INTPUT_SPEC>`
pub type DIER_INTPUT = crate::Reg<dier_intput::DIER_INTPUT_SPEC>;
///LPTIM interrupt Enable Register (intput mode)
pub mod dier_intput;
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
