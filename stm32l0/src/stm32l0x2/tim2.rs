///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - slave mode control register
    pub smcr: SMCR,
    ///0x0c - DMA/Interrupt enable register
    pub dier: DIER,
    ///0x10 - status register
    pub sr: SR,
    ///0x14 - event generation register
    pub egr: EGR,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ///0x20 - capture/compare enable register
    pub ccer: CCER,
    ///0x24 - TIMx counter
    pub cnt: CNT,
    _reserved10: [u8; 0x02],
    ///0x28 - prescaler
    pub psc: PSC,
    ///0x2c - TIMx auto-reload register
    pub arr: ARR,
    _reserved12: [u8; 0x06],
    ///0x34 - capture/compare register
    pub ccr1: CCR,
    _reserved13: [u8; 0x02],
    ///0x38 - capture/compare register
    pub ccr2: CCR,
    _reserved14: [u8; 0x02],
    ///0x3c - capture/compare register
    pub ccr3: CCR,
    _reserved15: [u8; 0x02],
    ///0x40 - capture/compare register
    pub ccr4: CCR,
    _reserved16: [u8; 0x06],
    ///0x48 - DMA control register
    pub dcr: DCR,
    ///0x4c - DMA address for full transfer
    pub dmar: DMAR,
    ///0x50 - TIM2 option register
    pub or: OR,
}
impl RegisterBlock {
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - capture/compare mode register 1 (output mode)
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x1c - capture/compare mode register 2 (input mode)
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - capture/compare mode register 2 (output mode)
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
///slave mode control register
pub mod smcr;
///DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`
pub type DIER = crate::Reg<dier::DIER_SPEC>;
///DMA/Interrupt enable register
pub mod dier;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`
pub type EGR = crate::Reg<egr::EGR_SPEC>;
///event generation register
pub mod egr;
///CCMR1_Output (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
///capture/compare mode register 1 (output mode)
pub mod ccmr1_output;
///CCMR1_Input (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
///capture/compare mode register 1 (input mode)
pub mod ccmr1_input;
///CCMR2_Output (rw) register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
///capture/compare mode register 2 (output mode)
pub mod ccmr2_output;
///CCMR2_Input (rw) register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
///capture/compare mode register 2 (input mode)
pub mod ccmr2_input;
///CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
///capture/compare enable register
pub mod ccer;
///PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///prescaler
pub mod psc;
///DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///DMA control register
pub mod dcr;
///DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
///DMA address for full transfer
pub mod dmar;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///TIM2 option register
pub mod or;
///CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///TIMx counter
pub mod cnt;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///TIMx auto-reload register
pub mod arr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///capture/compare register
pub mod ccr;
