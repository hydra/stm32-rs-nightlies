///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM12 control register 1
    pub tim12_cr1: TIM12_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM12 control register 2
    pub tim12_cr2: TIM12_CR2,
    ///0x08 - TIM12 slave mode control register
    pub tim12_smcr: TIM12_SMCR,
    ///0x0c - TIM12 interrupt enable register
    pub tim12_dier: TIM12_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM12 status register
    pub tim12_sr: TIM12_SR,
    ///0x14 - TIM12 event generation register
    pub tim12_egr: TIM12_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim12_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    ///0x20 - TIM12 capture/compare enable register
    pub tim12_ccer: TIM12_CCER,
    ///0x24 - TIM12 counter
    pub tim12_cnt: TIM12_CNT,
    ///0x28 - TIM12 prescaler
    pub tim12_psc: TIM12_PSC,
    _reserved10: [u8; 0x02],
    ///0x2c - TIM12 auto-reload register
    pub tim12_arr: TIM12_ARR,
    _reserved11: [u8; 0x06],
    ///0x34 - TIM12 capture/compare register 1
    pub tim12_ccr1: TIM12_CCR1,
    _reserved12: [u8; 0x02],
    ///0x38 - TIM12 capture/compare register 2
    pub tim12_ccr2: TIM12_CCR2,
    _reserved13: [u8; 0x2e],
    ///0x68 - TIM12 timer input selection register
    pub tim12_tisel: TIM12_TISEL,
}
impl RegisterBlock {
    ///0x18 - TIM12 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim12_ccmr1_output(&self) -> &TIM12_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - TIM12 capture/compare mode register 1
    #[inline(always)]
    pub const fn tim12_ccmr1_input(&self) -> &TIM12_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
///TIM12_CR1 (rw) register accessor: an alias for `Reg<TIM12_CR1_SPEC>`
pub type TIM12_CR1 = crate::Reg<tim12_cr1::TIM12_CR1_SPEC>;
///TIM12 control register 1
pub mod tim12_cr1;
///TIM12_CR2 (rw) register accessor: an alias for `Reg<TIM12_CR2_SPEC>`
pub type TIM12_CR2 = crate::Reg<tim12_cr2::TIM12_CR2_SPEC>;
///TIM12 control register 2
pub mod tim12_cr2;
///TIM12_SMCR (rw) register accessor: an alias for `Reg<TIM12_SMCR_SPEC>`
pub type TIM12_SMCR = crate::Reg<tim12_smcr::TIM12_SMCR_SPEC>;
///TIM12 slave mode control register
pub mod tim12_smcr;
///TIM12_DIER (rw) register accessor: an alias for `Reg<TIM12_DIER_SPEC>`
pub type TIM12_DIER = crate::Reg<tim12_dier::TIM12_DIER_SPEC>;
///TIM12 interrupt enable register
pub mod tim12_dier;
///TIM12_SR (rw) register accessor: an alias for `Reg<TIM12_SR_SPEC>`
pub type TIM12_SR = crate::Reg<tim12_sr::TIM12_SR_SPEC>;
///TIM12 status register
pub mod tim12_sr;
///TIM12_EGR (w) register accessor: an alias for `Reg<TIM12_EGR_SPEC>`
pub type TIM12_EGR = crate::Reg<tim12_egr::TIM12_EGR_SPEC>;
///TIM12 event generation register
pub mod tim12_egr;
///TIM12_CCMR1_input (rw) register accessor: an alias for `Reg<TIM12_CCMR1_INPUT_SPEC>`
pub type TIM12_CCMR1_INPUT = crate::Reg<tim12_ccmr1_input::TIM12_CCMR1_INPUT_SPEC>;
///TIM12 capture/compare mode register 1
pub mod tim12_ccmr1_input;
///TIM12_CCMR1_output (rw) register accessor: an alias for `Reg<TIM12_CCMR1_OUTPUT_SPEC>`
pub type TIM12_CCMR1_OUTPUT = crate::Reg<tim12_ccmr1_output::TIM12_CCMR1_OUTPUT_SPEC>;
///TIM12 capture/compare mode register 1
pub mod tim12_ccmr1_output;
///TIM12_CCER (rw) register accessor: an alias for `Reg<TIM12_CCER_SPEC>`
pub type TIM12_CCER = crate::Reg<tim12_ccer::TIM12_CCER_SPEC>;
///TIM12 capture/compare enable register
pub mod tim12_ccer;
///TIM12_CNT (rw) register accessor: an alias for `Reg<TIM12_CNT_SPEC>`
pub type TIM12_CNT = crate::Reg<tim12_cnt::TIM12_CNT_SPEC>;
///TIM12 counter
pub mod tim12_cnt;
///TIM12_PSC (rw) register accessor: an alias for `Reg<TIM12_PSC_SPEC>`
pub type TIM12_PSC = crate::Reg<tim12_psc::TIM12_PSC_SPEC>;
///TIM12 prescaler
pub mod tim12_psc;
///TIM12_ARR (rw) register accessor: an alias for `Reg<TIM12_ARR_SPEC>`
pub type TIM12_ARR = crate::Reg<tim12_arr::TIM12_ARR_SPEC>;
///TIM12 auto-reload register
pub mod tim12_arr;
///TIM12_CCR1 (rw) register accessor: an alias for `Reg<TIM12_CCR1_SPEC>`
pub type TIM12_CCR1 = crate::Reg<tim12_ccr1::TIM12_CCR1_SPEC>;
///TIM12 capture/compare register 1
pub mod tim12_ccr1;
///TIM12_CCR2 (rw) register accessor: an alias for `Reg<TIM12_CCR2_SPEC>`
pub type TIM12_CCR2 = crate::Reg<tim12_ccr2::TIM12_CCR2_SPEC>;
///TIM12 capture/compare register 2
pub mod tim12_ccr2;
///TIM12_TISEL (rw) register accessor: an alias for `Reg<TIM12_TISEL_SPEC>`
pub type TIM12_TISEL = crate::Reg<tim12_tisel::TIM12_TISEL_SPEC>;
///TIM12 timer input selection register
pub mod tim12_tisel;
