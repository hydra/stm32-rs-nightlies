///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM3 control register 1
    pub cr1: CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM3 control register 2
    pub cr2: CR2,
    _reserved2: [u8; 0x02],
    ///0x08 - TIM3 slave mode control register
    pub smcr: SMCR,
    ///0x0c - TIM3 DMA/Interrupt enable register
    pub dier: DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM3 status register
    pub sr: SR,
    _reserved5: [u8; 0x02],
    ///0x14 - TIM3 event generation register
    pub egr: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ///0x20 - TIM3 capture/compare enable register
    pub ccer: CCER,
    _reserved9: [u8; 0x02],
    ///0x24 - TIM3 counter \[alternate\]
    pub cnt: CNT,
    ///0x28 - TIM3 prescaler
    pub psc: PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM3 auto-reload register
    pub arr: ARR,
    _reserved12: [u8; 0x04],
    ///0x34 - TIM3 capture/compare register 1
    pub ccr1: CCR1,
    ///0x38 - TIM3 capture/compare register 2
    pub ccr2: CCR2,
    ///0x3c - TIM3 capture/compare register 3
    pub ccr3: CCR3,
    ///0x40 - TIM3 capture/compare register 4
    pub ccr4: CCR4,
    _reserved16: [u8; 0x04],
    ///0x48 - TIM3 DMA control register
    pub dcr: DCR,
    _reserved17: [u8; 0x02],
    ///0x4c - TIM3 DMA address for full transfer
    pub dmar: DMAR,
    _reserved18: [u8; 0x12],
    ///0x60 - TIM3 alternate function option register 1
    pub af1: AF1,
    _reserved19: [u8; 0x04],
    ///0x68 - TIM3 timer input selection register
    pub tisel: TISEL,
}
impl RegisterBlock {
    ///0x18 - TIM3 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - TIM3 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x1c - TIM3 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - TIM3 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///TIM3 control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///TIM3 control register 2
pub mod cr2;
///SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
///TIM3 slave mode control register
pub mod smcr;
///DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`
pub type DIER = crate::Reg<dier::DIER_SPEC>;
///TIM3 DMA/Interrupt enable register
pub mod dier;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///TIM3 status register
pub mod sr;
///EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`
pub type EGR = crate::Reg<egr::EGR_SPEC>;
///TIM3 event generation register
pub mod egr;
///CCMR1_input (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
///TIM3 capture/compare mode register 1 \[alternate\]
pub mod ccmr1_input;
///CCMR1_output (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
///TIM3 capture/compare mode register 1 \[alternate\]
pub mod ccmr1_output;
///CCMR2_input (rw) register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
///TIM3 capture/compare mode register 2 \[alternate\]
pub mod ccmr2_input;
///CCMR2_output (rw) register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
///TIM3 capture/compare mode register 2 \[alternate\]
pub mod ccmr2_output;
///CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
///TIM3 capture/compare enable register
pub mod ccer;
///CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///TIM3 counter \[alternate\]
pub mod cnt;
///PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///TIM3 prescaler
pub mod psc;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///TIM3 auto-reload register
pub mod arr;
///CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///TIM3 capture/compare register 1
pub mod ccr1;
///CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
///TIM3 capture/compare register 2
pub mod ccr2;
///CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
///TIM3 capture/compare register 3
pub mod ccr3;
///CCR4 (rw) register accessor: an alias for `Reg<CCR4_SPEC>`
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
///TIM3 capture/compare register 4
pub mod ccr4;
///DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///TIM3 DMA control register
pub mod dcr;
///DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
///TIM3 DMA address for full transfer
pub mod dmar;
///AF1 (rw) register accessor: an alias for `Reg<AF1_SPEC>`
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
///TIM3 alternate function option register 1
pub mod af1;
///TISEL (rw) register accessor: an alias for `Reg<TISEL_SPEC>`
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
///TIM3 timer input selection register
pub mod tisel;
