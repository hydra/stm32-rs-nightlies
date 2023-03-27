///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM6 control register 1
    pub cr1: CR1,
    ///0x04 - TIM6 control register 2
    pub cr2: CR2,
    _reserved2: [u8; 0x04],
    ///0x0c - TIM6 DMA/Interrupt enable register
    pub dier: DIER,
    ///0x10 - TIM6 status register
    pub sr: SR,
    ///0x14 - TIM6 event generation register
    pub egr: EGR,
    _reserved5: [u8; 0x0c],
    ///0x24 - TIM6 counter
    pub cnt: CNT,
    ///0x28 - TIM6 prescaler
    pub psc: PSC,
    ///0x2c - TIM6 auto-reload register
    pub arr: ARR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///TIM6 control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///TIM6 control register 2
pub mod cr2;
///DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`
pub type DIER = crate::Reg<dier::DIER_SPEC>;
///TIM6 DMA/Interrupt enable register
pub mod dier;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///TIM6 status register
pub mod sr;
///EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`
pub type EGR = crate::Reg<egr::EGR_SPEC>;
///TIM6 event generation register
pub mod egr;
///CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///TIM6 counter
pub mod cnt;
///PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///TIM6 prescaler
pub mod psc;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///TIM6 auto-reload register
pub mod arr;
