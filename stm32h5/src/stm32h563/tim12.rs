///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM2 control register 1
    pub tim2_cr1: TIM2_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM2 control register 2
    pub tim2_cr2: TIM2_CR2,
    ///0x08 - TIM2 slave mode control register
    pub tim2_smcr: TIM2_SMCR,
    ///0x0c - TIM2 DMA/Interrupt enable register
    pub tim2_dier: TIM2_DIER,
    ///0x10 - TIM2 status register
    pub tim2_sr: TIM2_SR,
    ///0x14 - TIM2 event generation register
    pub tim2_egr: TIM2_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim2_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    ///0x20 - TIM2 capture/compare enable register
    pub tim2_ccer: TIM2_CCER,
    _reserved8: [u8; 0x02],
    ///0x24 - TIM2 counter
    pub tim2_cnt: TIM2_CNT,
    ///0x28 - TIM2 prescaler
    pub tim2_psc: TIM2_PSC,
    _reserved10: [u8; 0x02],
    ///0x2c - TIM2 auto-reload register
    pub tim2_arr: TIM2_ARR,
    _reserved11: [u8; 0x04],
    ///0x34 - TIM2 capture/compare register 1
    pub tim2_ccr1: TIM2_CCR1,
    ///0x38 - TIM2 capture/compare register 2
    pub tim2_ccr2: TIM2_CCR2,
    _reserved13: [u8; 0x20],
    ///0x5c - TIM2 timer input selection register
    pub tim2_tisel: TIM2_TISEL,
}
impl RegisterBlock {
    ///0x18 - TIM2 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim2_ccmr1_output(&self) -> &TIM2_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - TIM2 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim2_ccmr1_input(&self) -> &TIM2_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
///TIM2_CR1 (rw) register accessor: an alias for `Reg<TIM2_CR1_SPEC>`
pub type TIM2_CR1 = crate::Reg<tim2_cr1::TIM2_CR1_SPEC>;
///TIM2 control register 1
pub mod tim2_cr1;
///TIM2_CR2 (rw) register accessor: an alias for `Reg<TIM2_CR2_SPEC>`
pub type TIM2_CR2 = crate::Reg<tim2_cr2::TIM2_CR2_SPEC>;
///TIM2 control register 2
pub mod tim2_cr2;
///TIM2_SMCR (rw) register accessor: an alias for `Reg<TIM2_SMCR_SPEC>`
pub type TIM2_SMCR = crate::Reg<tim2_smcr::TIM2_SMCR_SPEC>;
///TIM2 slave mode control register
pub mod tim2_smcr;
///TIM2_DIER (rw) register accessor: an alias for `Reg<TIM2_DIER_SPEC>`
pub type TIM2_DIER = crate::Reg<tim2_dier::TIM2_DIER_SPEC>;
///TIM2 DMA/Interrupt enable register
pub mod tim2_dier;
///TIM2_SR (rw) register accessor: an alias for `Reg<TIM2_SR_SPEC>`
pub type TIM2_SR = crate::Reg<tim2_sr::TIM2_SR_SPEC>;
///TIM2 status register
pub mod tim2_sr;
///TIM2_EGR (w) register accessor: an alias for `Reg<TIM2_EGR_SPEC>`
pub type TIM2_EGR = crate::Reg<tim2_egr::TIM2_EGR_SPEC>;
///TIM2 event generation register
pub mod tim2_egr;
///TIM2_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM2_CCMR1_INPUT_SPEC>`
pub type TIM2_CCMR1_INPUT = crate::Reg<tim2_ccmr1_input::TIM2_CCMR1_INPUT_SPEC>;
///TIM2 capture/compare mode register 1 \[alternate\]
pub mod tim2_ccmr1_input;
///TIM2_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM2_CCMR1_OUTPUT_SPEC>`
pub type TIM2_CCMR1_OUTPUT = crate::Reg<tim2_ccmr1_output::TIM2_CCMR1_OUTPUT_SPEC>;
///TIM2 capture/compare mode register 1 \[alternate\]
pub mod tim2_ccmr1_output;
///TIM2_CCER (rw) register accessor: an alias for `Reg<TIM2_CCER_SPEC>`
pub type TIM2_CCER = crate::Reg<tim2_ccer::TIM2_CCER_SPEC>;
///TIM2 capture/compare enable register
pub mod tim2_ccer;
///TIM2_CNT (rw) register accessor: an alias for `Reg<TIM2_CNT_SPEC>`
pub type TIM2_CNT = crate::Reg<tim2_cnt::TIM2_CNT_SPEC>;
///TIM2 counter
pub mod tim2_cnt;
///TIM2_PSC (rw) register accessor: an alias for `Reg<TIM2_PSC_SPEC>`
pub type TIM2_PSC = crate::Reg<tim2_psc::TIM2_PSC_SPEC>;
///TIM2 prescaler
pub mod tim2_psc;
///TIM2_ARR (rw) register accessor: an alias for `Reg<TIM2_ARR_SPEC>`
pub type TIM2_ARR = crate::Reg<tim2_arr::TIM2_ARR_SPEC>;
///TIM2 auto-reload register
pub mod tim2_arr;
///TIM2_CCR1 (rw) register accessor: an alias for `Reg<TIM2_CCR1_SPEC>`
pub type TIM2_CCR1 = crate::Reg<tim2_ccr1::TIM2_CCR1_SPEC>;
///TIM2 capture/compare register 1
pub mod tim2_ccr1;
///TIM2_CCR2 (rw) register accessor: an alias for `Reg<TIM2_CCR2_SPEC>`
pub type TIM2_CCR2 = crate::Reg<tim2_ccr2::TIM2_CCR2_SPEC>;
///TIM2 capture/compare register 2
pub mod tim2_ccr2;
///TIM2_TISEL (rw) register accessor: an alias for `Reg<TIM2_TISEL_SPEC>`
pub type TIM2_TISEL = crate::Reg<tim2_tisel::TIM2_TISEL_SPEC>;
///TIM2 timer input selection register
pub mod tim2_tisel;
