///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM16/TIM17 control register 1
    pub timx_cr1: TIMX_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM16/TIM17 control register 2
    pub timx_cr2: TIMX_CR2,
    _reserved2: [u8; 0x06],
    ///0x0c - TIM16/TIM17 DMA/interrupt enable register
    pub timx_dier: TIMX_DIER,
    _reserved3: [u8; 0x02],
    ///0x10 - TIM16/TIM17 status register
    pub timx_sr: TIMX_SR,
    _reserved4: [u8; 0x02],
    ///0x14 - event generation register
    pub timx_egr: TIMX_EGR,
    _reserved5: [u8; 0x08],
    ///0x20 - TIM16/TIM17 capture/compare enable register
    pub timx_ccer: TIMX_CCER,
    _reserved6: [u8; 0x02],
    ///0x24 - TIM16/TIM17 counter
    pub timx_cnt: TIMX_CNT,
    ///0x28 - TIM16/TIM17 prescaler
    pub timx_psc: TIMX_PSC,
    _reserved8: [u8; 0x02],
    ///0x2c - TIM16/TIM17 auto-reload register
    pub timx_arr: TIMX_ARR,
    _reserved9: [u8; 0x02],
    ///0x30 - TIM16/TIM17 repetition counter register
    pub timx_rcr: TIMX_RCR,
    _reserved10: [u8; 0x02],
    ///0x34 - TIM16/TIM17 capture/compare register 1
    pub timx_ccr1: TIMX_CCR1,
    _reserved11: [u8; 0x0e],
    ///0x44 - As the BKBID, BKDSRM, BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub timx_bdtr: TIMX_BDTR,
    ///0x48 - TIM16/TIM17 DMA control register
    pub timx_dcr: TIMX_DCR,
    _reserved13: [u8; 0x02],
    ///0x4c - TIM16/TIM17 DMA address for full transfer
    pub timx_dmar: TIMX_DMAR,
    _reserved14: [u8; 0x12],
    ///0x60 - TIM17 alternate function register 1
    pub timx_af1: TIMX_AF1,
    _reserved15: [u8; 0x04],
    ///0x68 - TIM17 input selection register
    pub timx_tisel: TIMX_TISEL,
}
///TIMx_CR1 (rw) register accessor: an alias for `Reg<TIMX_CR1_SPEC>`
pub type TIMX_CR1 = crate::Reg<timx_cr1::TIMX_CR1_SPEC>;
///TIM16/TIM17 control register 1
pub mod timx_cr1;
///TIMx_CR2 (rw) register accessor: an alias for `Reg<TIMX_CR2_SPEC>`
pub type TIMX_CR2 = crate::Reg<timx_cr2::TIMX_CR2_SPEC>;
///TIM16/TIM17 control register 2
pub mod timx_cr2;
///TIMx_DIER (rw) register accessor: an alias for `Reg<TIMX_DIER_SPEC>`
pub type TIMX_DIER = crate::Reg<timx_dier::TIMX_DIER_SPEC>;
///TIM16/TIM17 DMA/interrupt enable register
pub mod timx_dier;
///TIMx_SR (rw) register accessor: an alias for `Reg<TIMX_SR_SPEC>`
pub type TIMX_SR = crate::Reg<timx_sr::TIMX_SR_SPEC>;
///TIM16/TIM17 status register
pub mod timx_sr;
///TIMx_EGR (w) register accessor: an alias for `Reg<TIMX_EGR_SPEC>`
pub type TIMX_EGR = crate::Reg<timx_egr::TIMX_EGR_SPEC>;
///event generation register
pub mod timx_egr;
///TIMx_CCER (rw) register accessor: an alias for `Reg<TIMX_CCER_SPEC>`
pub type TIMX_CCER = crate::Reg<timx_ccer::TIMX_CCER_SPEC>;
///TIM16/TIM17 capture/compare enable register
pub mod timx_ccer;
///TIMx_CNT (rw) register accessor: an alias for `Reg<TIMX_CNT_SPEC>`
pub type TIMX_CNT = crate::Reg<timx_cnt::TIMX_CNT_SPEC>;
///TIM16/TIM17 counter
pub mod timx_cnt;
///TIMx_PSC (rw) register accessor: an alias for `Reg<TIMX_PSC_SPEC>`
pub type TIMX_PSC = crate::Reg<timx_psc::TIMX_PSC_SPEC>;
///TIM16/TIM17 prescaler
pub mod timx_psc;
///TIMx_ARR (rw) register accessor: an alias for `Reg<TIMX_ARR_SPEC>`
pub type TIMX_ARR = crate::Reg<timx_arr::TIMX_ARR_SPEC>;
///TIM16/TIM17 auto-reload register
pub mod timx_arr;
///TIMx_RCR (rw) register accessor: an alias for `Reg<TIMX_RCR_SPEC>`
pub type TIMX_RCR = crate::Reg<timx_rcr::TIMX_RCR_SPEC>;
///TIM16/TIM17 repetition counter register
pub mod timx_rcr;
///TIMx_CCR1 (rw) register accessor: an alias for `Reg<TIMX_CCR1_SPEC>`
pub type TIMX_CCR1 = crate::Reg<timx_ccr1::TIMX_CCR1_SPEC>;
///TIM16/TIM17 capture/compare register 1
pub mod timx_ccr1;
///TIMx_BDTR (rw) register accessor: an alias for `Reg<TIMX_BDTR_SPEC>`
pub type TIMX_BDTR = crate::Reg<timx_bdtr::TIMX_BDTR_SPEC>;
///As the BKBID, BKDSRM, BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod timx_bdtr;
///TIMx_DCR (rw) register accessor: an alias for `Reg<TIMX_DCR_SPEC>`
pub type TIMX_DCR = crate::Reg<timx_dcr::TIMX_DCR_SPEC>;
///TIM16/TIM17 DMA control register
pub mod timx_dcr;
///TIMx_DMAR (rw) register accessor: an alias for `Reg<TIMX_DMAR_SPEC>`
pub type TIMX_DMAR = crate::Reg<timx_dmar::TIMX_DMAR_SPEC>;
///TIM16/TIM17 DMA address for full transfer
pub mod timx_dmar;
///TIMx_AF1 (rw) register accessor: an alias for `Reg<TIMX_AF1_SPEC>`
pub type TIMX_AF1 = crate::Reg<timx_af1::TIMX_AF1_SPEC>;
///TIM17 alternate function register 1
pub mod timx_af1;
///TIMx_TISEL (rw) register accessor: an alias for `Reg<TIMX_TISEL_SPEC>`
pub type TIMX_TISEL = crate::Reg<timx_tisel::TIMX_TISEL_SPEC>;
///TIM17 input selection register
pub mod timx_tisel;
