///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM15 control register 1
    pub tim15_cr1: TIM15_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM15 control register 2
    pub tim15_cr2: TIM15_CR2,
    _reserved2: [u8; 0x02],
    ///0x08 - slave mode control register
    pub timx_smcr: TIMX_SMCR,
    ///0x0c - TIM15 DMA/interrupt enable register
    pub tim15_dier: TIM15_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM15 status register
    pub tim15_sr: TIM15_SR,
    _reserved5: [u8; 0x02],
    ///0x14 - event generation register
    pub timx_egr: TIMX_EGR,
    _reserved_6_timx_ccmr1: [u8; 0x04],
    _reserved7: [u8; 0x04],
    ///0x20 - TIM15 capture/compare enable register
    pub tim15_ccer: TIM15_CCER,
    _reserved8: [u8; 0x02],
    ///0x24 - TIM15 counter
    pub tim15_cnt: TIM15_CNT,
    ///0x28 - TIM15 prescaler
    pub tim15_psc: TIM15_PSC,
    _reserved10: [u8; 0x02],
    ///0x2c - TIM15 auto-reload register
    pub tim15_arr: TIM15_ARR,
    _reserved11: [u8; 0x02],
    ///0x30 - TIM15 repetition counter register
    pub tim15_rcr: TIM15_RCR,
    _reserved12: [u8; 0x02],
    ///0x34 - TIM15 capture/compare register 1
    pub tim15_ccr1: TIM15_CCR1,
    _reserved13: [u8; 0x02],
    ///0x38 - TIM15 capture/compare register 2
    pub tim15_ccr2: TIM15_CCR2,
    _reserved14: [u8; 0x0a],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub timx_bdtr: TIMX_BDTR,
    ///0x48 - TIM15 DMA control register
    pub tim15_dcr: TIM15_DCR,
    _reserved16: [u8; 0x02],
    ///0x4c - TIM15 DMA address for full transfer
    pub tim15_dmar: TIM15_DMAR,
    _reserved17: [u8; 0x12],
    ///0x60 - TIM15 alternate register 1
    pub tim15_af1: TIM15_AF1,
    _reserved18: [u8; 0x04],
    ///0x68 - TIM15 input selection register
    pub tim15_tisel: TIM15_TISEL,
}
impl RegisterBlock {
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub const fn timx_ccmr1_input(&self) -> &TIMX_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - capture/compare mode register 1 (output mode)
    #[inline(always)]
    pub const fn timx_ccmr1_output(&self) -> &TIMX_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
}
///TIM15_CR1 (rw) register accessor: an alias for `Reg<TIM15_CR1_SPEC>`
pub type TIM15_CR1 = crate::Reg<tim15_cr1::TIM15_CR1_SPEC>;
///TIM15 control register 1
pub mod tim15_cr1;
///TIM15_CR2 (rw) register accessor: an alias for `Reg<TIM15_CR2_SPEC>`
pub type TIM15_CR2 = crate::Reg<tim15_cr2::TIM15_CR2_SPEC>;
///TIM15 control register 2
pub mod tim15_cr2;
///TIMx_SMCR (rw) register accessor: an alias for `Reg<TIMX_SMCR_SPEC>`
pub type TIMX_SMCR = crate::Reg<timx_smcr::TIMX_SMCR_SPEC>;
///slave mode control register
pub mod timx_smcr;
///TIM15_DIER (rw) register accessor: an alias for `Reg<TIM15_DIER_SPEC>`
pub type TIM15_DIER = crate::Reg<tim15_dier::TIM15_DIER_SPEC>;
///TIM15 DMA/interrupt enable register
pub mod tim15_dier;
///TIM15_SR (rw) register accessor: an alias for `Reg<TIM15_SR_SPEC>`
pub type TIM15_SR = crate::Reg<tim15_sr::TIM15_SR_SPEC>;
///TIM15 status register
pub mod tim15_sr;
///TIMx_EGR (w) register accessor: an alias for `Reg<TIMX_EGR_SPEC>`
pub type TIMX_EGR = crate::Reg<timx_egr::TIMX_EGR_SPEC>;
///event generation register
pub mod timx_egr;
///TIMx_CCMR1_Output (rw) register accessor: an alias for `Reg<TIMX_CCMR1_OUTPUT_SPEC>`
pub type TIMX_CCMR1_OUTPUT = crate::Reg<timx_ccmr1_output::TIMX_CCMR1_OUTPUT_SPEC>;
///capture/compare mode register 1 (output mode)
pub mod timx_ccmr1_output;
///TIMx_CCMR1_Input (rw) register accessor: an alias for `Reg<TIMX_CCMR1_INPUT_SPEC>`
pub type TIMX_CCMR1_INPUT = crate::Reg<timx_ccmr1_input::TIMX_CCMR1_INPUT_SPEC>;
///capture/compare mode register 1 (input mode)
pub mod timx_ccmr1_input;
///TIM15_CCER (rw) register accessor: an alias for `Reg<TIM15_CCER_SPEC>`
pub type TIM15_CCER = crate::Reg<tim15_ccer::TIM15_CCER_SPEC>;
///TIM15 capture/compare enable register
pub mod tim15_ccer;
///TIM15_CNT (rw) register accessor: an alias for `Reg<TIM15_CNT_SPEC>`
pub type TIM15_CNT = crate::Reg<tim15_cnt::TIM15_CNT_SPEC>;
///TIM15 counter
pub mod tim15_cnt;
///TIM15_PSC (rw) register accessor: an alias for `Reg<TIM15_PSC_SPEC>`
pub type TIM15_PSC = crate::Reg<tim15_psc::TIM15_PSC_SPEC>;
///TIM15 prescaler
pub mod tim15_psc;
///TIM15_ARR (rw) register accessor: an alias for `Reg<TIM15_ARR_SPEC>`
pub type TIM15_ARR = crate::Reg<tim15_arr::TIM15_ARR_SPEC>;
///TIM15 auto-reload register
pub mod tim15_arr;
///TIM15_RCR (rw) register accessor: an alias for `Reg<TIM15_RCR_SPEC>`
pub type TIM15_RCR = crate::Reg<tim15_rcr::TIM15_RCR_SPEC>;
///TIM15 repetition counter register
pub mod tim15_rcr;
///TIM15_CCR1 (rw) register accessor: an alias for `Reg<TIM15_CCR1_SPEC>`
pub type TIM15_CCR1 = crate::Reg<tim15_ccr1::TIM15_CCR1_SPEC>;
///TIM15 capture/compare register 1
pub mod tim15_ccr1;
///TIM15_CCR2 (rw) register accessor: an alias for `Reg<TIM15_CCR2_SPEC>`
pub type TIM15_CCR2 = crate::Reg<tim15_ccr2::TIM15_CCR2_SPEC>;
///TIM15 capture/compare register 2
pub mod tim15_ccr2;
///TIMx_BDTR (rw) register accessor: an alias for `Reg<TIMX_BDTR_SPEC>`
pub type TIMX_BDTR = crate::Reg<timx_bdtr::TIMX_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod timx_bdtr;
///TIM15_DCR (rw) register accessor: an alias for `Reg<TIM15_DCR_SPEC>`
pub type TIM15_DCR = crate::Reg<tim15_dcr::TIM15_DCR_SPEC>;
///TIM15 DMA control register
pub mod tim15_dcr;
///TIM15_DMAR (rw) register accessor: an alias for `Reg<TIM15_DMAR_SPEC>`
pub type TIM15_DMAR = crate::Reg<tim15_dmar::TIM15_DMAR_SPEC>;
///TIM15 DMA address for full transfer
pub mod tim15_dmar;
///TIM15_AF1 (rw) register accessor: an alias for `Reg<TIM15_AF1_SPEC>`
pub type TIM15_AF1 = crate::Reg<tim15_af1::TIM15_AF1_SPEC>;
///TIM15 alternate register 1
pub mod tim15_af1;
///TIM15_TISEL (rw) register accessor: an alias for `Reg<TIM15_TISEL_SPEC>`
pub type TIM15_TISEL = crate::Reg<tim15_tisel::TIM15_TISEL_SPEC>;
///TIM15 input selection register
pub mod tim15_tisel;
