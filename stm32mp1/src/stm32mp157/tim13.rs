///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM13 control register 1
    pub tim13_cr1: TIM13_CR1,
    _reserved1: [u8; 0x0a],
    ///0x0c - TIM13 Interrupt enable register
    pub tim13_dier: TIM13_DIER,
    _reserved2: [u8; 0x02],
    ///0x10 - TIM13 status register
    pub tim13_sr: TIM13_SR,
    _reserved3: [u8; 0x02],
    ///0x14 - TIM13 event generation register
    pub tim13_egr: TIM13_EGR,
    _reserved4: [u8; 0x02],
    ///0x18 - The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode
    pub tim13_ccmr1: TIM13_CCMR1,
    _reserved5: [u8; 0x04],
    ///0x20 - TIM13 capture/compare enable register
    pub tim13_ccer: TIM13_CCER,
    _reserved6: [u8; 0x02],
    ///0x24 - TIM13 counter
    pub tim13_cnt: TIM13_CNT,
    ///0x28 - TIM13 prescaler
    pub tim13_psc: TIM13_PSC,
    _reserved8: [u8; 0x02],
    ///0x2c - TIM13 auto-reload register
    pub tim13_arr: TIM13_ARR,
    _reserved9: [u8; 0x06],
    ///0x34 - TIM13 capture/compare register 1
    pub tim13_ccr1: TIM13_CCR1,
    _reserved10: [u8; 0x32],
    ///0x68 - TIM13 timer input selection register
    pub tim13_tisel: TIM13_TISEL,
}
///TIM13_CR1 (rw) register accessor: an alias for `Reg<TIM13_CR1_SPEC>`
pub type TIM13_CR1 = crate::Reg<tim13_cr1::TIM13_CR1_SPEC>;
///TIM13 control register 1
pub mod tim13_cr1;
///TIM13_DIER (rw) register accessor: an alias for `Reg<TIM13_DIER_SPEC>`
pub type TIM13_DIER = crate::Reg<tim13_dier::TIM13_DIER_SPEC>;
///TIM13 Interrupt enable register
pub mod tim13_dier;
///TIM13_SR (rw) register accessor: an alias for `Reg<TIM13_SR_SPEC>`
pub type TIM13_SR = crate::Reg<tim13_sr::TIM13_SR_SPEC>;
///TIM13 status register
pub mod tim13_sr;
///TIM13_EGR (w) register accessor: an alias for `Reg<TIM13_EGR_SPEC>`
pub type TIM13_EGR = crate::Reg<tim13_egr::TIM13_EGR_SPEC>;
///TIM13 event generation register
pub mod tim13_egr;
///TIM13_CCMR1 (rw) register accessor: an alias for `Reg<TIM13_CCMR1_SPEC>`
pub type TIM13_CCMR1 = crate::Reg<tim13_ccmr1::TIM13_CCMR1_SPEC>;
///The channels can be used in input (capture mode) or in output (compare mode). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function in input and in output mode. For a given bit, OCxx describes its function when the channel is configured in output, ICxx describes its function when the channel is configured in input. So one must take care that the same bit can have a different meaning for the input stage and for the output stage. Output compare mode
pub mod tim13_ccmr1;
///TIM13_CCER (rw) register accessor: an alias for `Reg<TIM13_CCER_SPEC>`
pub type TIM13_CCER = crate::Reg<tim13_ccer::TIM13_CCER_SPEC>;
///TIM13 capture/compare enable register
pub mod tim13_ccer;
///TIM13_CNT (rw) register accessor: an alias for `Reg<TIM13_CNT_SPEC>`
pub type TIM13_CNT = crate::Reg<tim13_cnt::TIM13_CNT_SPEC>;
///TIM13 counter
pub mod tim13_cnt;
///TIM13_PSC (rw) register accessor: an alias for `Reg<TIM13_PSC_SPEC>`
pub type TIM13_PSC = crate::Reg<tim13_psc::TIM13_PSC_SPEC>;
///TIM13 prescaler
pub mod tim13_psc;
///TIM13_ARR (rw) register accessor: an alias for `Reg<TIM13_ARR_SPEC>`
pub type TIM13_ARR = crate::Reg<tim13_arr::TIM13_ARR_SPEC>;
///TIM13 auto-reload register
pub mod tim13_arr;
///TIM13_CCR1 (rw) register accessor: an alias for `Reg<TIM13_CCR1_SPEC>`
pub type TIM13_CCR1 = crate::Reg<tim13_ccr1::TIM13_CCR1_SPEC>;
///TIM13 capture/compare register 1
pub mod tim13_ccr1;
///TIM13_TISEL (rw) register accessor: an alias for `Reg<TIM13_TISEL_SPEC>`
pub type TIM13_TISEL = crate::Reg<tim13_tisel::TIM13_TISEL_SPEC>;
///TIM13 timer input selection register
pub mod tim13_tisel;
