///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM6 control register 1
    pub tim6_cr1: TIM6_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM6 control register 2
    pub tim6_cr2: TIM6_CR2,
    _reserved2: [u8; 0x06],
    ///0x0c - TIM6 DMA/Interrupt enable register
    pub tim6_dier: TIM6_DIER,
    _reserved3: [u8; 0x02],
    ///0x10 - TIM6 status register
    pub tim6_sr: TIM6_SR,
    _reserved4: [u8; 0x02],
    ///0x14 - TIM6 event generation register
    pub tim6_egr: TIM6_EGR,
    _reserved5: [u8; 0x0e],
    ///0x24 - TIM6 counter
    pub tim6_cnt: TIM6_CNT,
    ///0x28 - TIM6 prescaler
    pub tim6_psc: TIM6_PSC,
    _reserved7: [u8; 0x02],
    ///0x2c - TIM6 auto-reload register
    pub tim6_arr: TIM6_ARR,
}
///TIM6_CR1 (rw) register accessor: an alias for `Reg<TIM6_CR1_SPEC>`
pub type TIM6_CR1 = crate::Reg<tim6_cr1::TIM6_CR1_SPEC>;
///TIM6 control register 1
pub mod tim6_cr1;
///TIM6_CR2 (rw) register accessor: an alias for `Reg<TIM6_CR2_SPEC>`
pub type TIM6_CR2 = crate::Reg<tim6_cr2::TIM6_CR2_SPEC>;
///TIM6 control register 2
pub mod tim6_cr2;
///TIM6_DIER (rw) register accessor: an alias for `Reg<TIM6_DIER_SPEC>`
pub type TIM6_DIER = crate::Reg<tim6_dier::TIM6_DIER_SPEC>;
///TIM6 DMA/Interrupt enable register
pub mod tim6_dier;
///TIM6_SR (rw) register accessor: an alias for `Reg<TIM6_SR_SPEC>`
pub type TIM6_SR = crate::Reg<tim6_sr::TIM6_SR_SPEC>;
///TIM6 status register
pub mod tim6_sr;
///TIM6_EGR (w) register accessor: an alias for `Reg<TIM6_EGR_SPEC>`
pub type TIM6_EGR = crate::Reg<tim6_egr::TIM6_EGR_SPEC>;
///TIM6 event generation register
pub mod tim6_egr;
///TIM6_CNT (rw) register accessor: an alias for `Reg<TIM6_CNT_SPEC>`
pub type TIM6_CNT = crate::Reg<tim6_cnt::TIM6_CNT_SPEC>;
///TIM6 counter
pub mod tim6_cnt;
///TIM6_PSC (rw) register accessor: an alias for `Reg<TIM6_PSC_SPEC>`
pub type TIM6_PSC = crate::Reg<tim6_psc::TIM6_PSC_SPEC>;
///TIM6 prescaler
pub mod tim6_psc;
///TIM6_ARR (rw) register accessor: an alias for `Reg<TIM6_ARR_SPEC>`
pub type TIM6_ARR = crate::Reg<tim6_arr::TIM6_ARR_SPEC>;
///TIM6 auto-reload register
pub mod tim6_arr;
