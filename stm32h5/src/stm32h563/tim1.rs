///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM1 control register 1
    pub cr1: CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM1 control register 2
    pub cr2: CR2,
    ///0x08 - TIM1 slave mode control register
    pub smcr: SMCR,
    ///0x0c - TIM1 DMA/interrupt enable register
    pub dier: DIER,
    ///0x10 - TIM1 status register
    pub sr: SR,
    ///0x14 - TIM1 event generation register
    pub egr: EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ///0x20 - TIM1 capture/compare enable register
    pub ccer: CCER,
    ///0x24 - TIM1 counter
    pub cnt: CNT,
    ///0x28 - TIM1 prescaler
    pub psc: PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM1 auto-reload register
    pub arr: ARR,
    ///0x30 - TIM1 repetition counter register
    pub rcr: RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM1 capture/compare register 1
    pub ccr1: CCR1,
    ///0x38 - TIM1 capture/compare register 2
    pub ccr2: CCR2,
    ///0x3c - TIM1 capture/compare register 3
    pub ccr3: CCR3,
    ///0x40 - TIM1 capture/compare register 4
    pub ccr4: CCR4,
    ///0x44 - TIM1 break and dead-time register
    pub bdtr: BDTR,
    ///0x48 - TIM1 capture/compare register 5
    pub ccr5: CCR5,
    ///0x4c - TIM1 capture/compare register 6
    pub ccr6: CCR6,
    ///0x50 - TIM1 capture/compare mode register 3
    pub ccmr3: CCMR3,
    ///0x54 - TIM1 timer deadtime register 2
    pub dtr2: DTR2,
    ///0x58 - TIM1 timer encoder control register
    pub ecr: ECR,
    ///0x5c - TIM1 timer input selection register
    pub tisel: TISEL,
    ///0x60 - TIM1 alternate function option register 1
    pub af1: AF1,
    ///0x64 - TIM1 alternate function register 2
    pub af2: AF2,
    _reserved26: [u8; 0x0374],
    ///0x3dc - TIM1 DMA control register
    pub dcr: DCR,
    ///0x3e0 - TIM1 DMA address for full transfer
    pub dmar: DMAR,
}
impl RegisterBlock {
    ///0x18 - TIM1 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - TIM1 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x1c - TIM1 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - TIM1 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///TIM1 control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///TIM1 control register 2
pub mod cr2;
///SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
///TIM1 slave mode control register
pub mod smcr;
///DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`
pub type DIER = crate::Reg<dier::DIER_SPEC>;
///TIM1 DMA/interrupt enable register
pub mod dier;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///TIM1 status register
pub mod sr;
///EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`
pub type EGR = crate::Reg<egr::EGR_SPEC>;
///TIM1 event generation register
pub mod egr;
///CCMR1_Input (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
///TIM1 capture/compare mode register 1 \[alternate\]
pub mod ccmr1_input;
///CCMR1_Output (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
///TIM1 capture/compare mode register 1 \[alternate\]
pub mod ccmr1_output;
///CCMR2_Input (rw) register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
///TIM1 capture/compare mode register 2 \[alternate\]
pub mod ccmr2_input;
///CCMR2_Output (rw) register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
///TIM1 capture/compare mode register 2 \[alternate\]
pub mod ccmr2_output;
///CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
///TIM1 capture/compare enable register
pub mod ccer;
///CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///TIM1 counter
pub mod cnt;
///PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///TIM1 prescaler
pub mod psc;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///TIM1 auto-reload register
pub mod arr;
///RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
///TIM1 repetition counter register
pub mod rcr;
///CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///TIM1 capture/compare register 1
pub mod ccr1;
///CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
///TIM1 capture/compare register 2
pub mod ccr2;
///CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
///TIM1 capture/compare register 3
pub mod ccr3;
///CCR4 (rw) register accessor: an alias for `Reg<CCR4_SPEC>`
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
///TIM1 capture/compare register 4
pub mod ccr4;
///BDTR (rw) register accessor: an alias for `Reg<BDTR_SPEC>`
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
///TIM1 break and dead-time register
pub mod bdtr;
///CCR5 (rw) register accessor: an alias for `Reg<CCR5_SPEC>`
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
///TIM1 capture/compare register 5
pub mod ccr5;
///CCR6 (rw) register accessor: an alias for `Reg<CCR6_SPEC>`
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
///TIM1 capture/compare register 6
pub mod ccr6;
///CCMR3 (rw) register accessor: an alias for `Reg<CCMR3_SPEC>`
pub type CCMR3 = crate::Reg<ccmr3::CCMR3_SPEC>;
///TIM1 capture/compare mode register 3
pub mod ccmr3;
///DTR2 (rw) register accessor: an alias for `Reg<DTR2_SPEC>`
pub type DTR2 = crate::Reg<dtr2::DTR2_SPEC>;
///TIM1 timer deadtime register 2
pub mod dtr2;
///ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
///TIM1 timer encoder control register
pub mod ecr;
///TISEL (rw) register accessor: an alias for `Reg<TISEL_SPEC>`
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
///TIM1 timer input selection register
pub mod tisel;
///AF1 (rw) register accessor: an alias for `Reg<AF1_SPEC>`
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
///TIM1 alternate function option register 1
pub mod af1;
///AF2 (rw) register accessor: an alias for `Reg<AF2_SPEC>`
pub type AF2 = crate::Reg<af2::AF2_SPEC>;
///TIM1 alternate function register 2
pub mod af2;
///DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///TIM1 DMA control register
pub mod dcr;
///DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
///TIM1 DMA address for full transfer
pub mod dmar;
