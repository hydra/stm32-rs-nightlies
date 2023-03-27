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
    ///0x24 - counter
    pub cnt: CNT,
    ///0x28 - prescaler
    pub psc: PSC,
    ///0x2c - auto-reload register
    pub arr: ARR,
    ///0x30 - repetition counter register
    pub rcr: RCR,
    ///0x34..0x44 - capture/compare register
    pub ccr: [CCR; 4],
    ///0x44 - break and dead-time register
    pub bdtr: BDTR,
    ///0x48 - capture/compare register
    pub ccr5: CCR5,
    ///0x4c - capture/compare register
    pub ccr6: CCR6,
    ///0x50 - capture/compare mode register 2 (output mode)
    pub ccmr3_output: CCMR3_OUTPUT,
    ///0x54 - timer Deadtime Register 2
    pub dtr2: DTR2,
    ///0x58 - DMA control register
    pub ecr: ECR,
    ///0x5c - TIM timer input selection register
    pub tisel: TISEL,
    ///0x60 - TIM alternate function option register 1
    pub af1: AF1,
    ///0x64 - TIM alternate function option register 2
    pub af2: AF2,
    _reserved23: [u8; 0x0374],
    ///0x3dc - control register
    pub dcr: DCR,
    ///0x3e0 - DMA address for full transfer
    pub dmar: DMAR,
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
    ///0x34 - capture/compare register
    #[inline(always)]
    pub fn ccr1(&self) -> &CCR {
        &self.ccr[0]
    }
    ///0x38 - capture/compare register
    #[inline(always)]
    pub fn ccr2(&self) -> &CCR {
        &self.ccr[1]
    }
    ///0x3c - capture/compare register
    #[inline(always)]
    pub fn ccr3(&self) -> &CCR {
        &self.ccr[2]
    }
    ///0x40 - capture/compare register
    #[inline(always)]
    pub fn ccr4(&self) -> &CCR {
        &self.ccr[3]
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
///CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///counter
pub mod cnt;
///PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///prescaler
pub mod psc;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///auto-reload register
pub mod arr;
///RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
///repetition counter register
pub mod rcr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///capture/compare register
pub mod ccr;
///BDTR (rw) register accessor: an alias for `Reg<BDTR_SPEC>`
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
///break and dead-time register
pub mod bdtr;
///CCR5 (rw) register accessor: an alias for `Reg<CCR5_SPEC>`
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
///capture/compare register
pub mod ccr5;
///CCR6 (rw) register accessor: an alias for `Reg<CCR6_SPEC>`
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
///capture/compare register
pub mod ccr6;
///CCMR3_Output (rw) register accessor: an alias for `Reg<CCMR3_OUTPUT_SPEC>`
pub type CCMR3_OUTPUT = crate::Reg<ccmr3_output::CCMR3_OUTPUT_SPEC>;
///capture/compare mode register 2 (output mode)
pub mod ccmr3_output;
///DTR2 (rw) register accessor: an alias for `Reg<DTR2_SPEC>`
pub type DTR2 = crate::Reg<dtr2::DTR2_SPEC>;
///timer Deadtime Register 2
pub mod dtr2;
///ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
///DMA control register
pub mod ecr;
///TISEL (rw) register accessor: an alias for `Reg<TISEL_SPEC>`
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
///TIM timer input selection register
pub mod tisel;
///AF1 (rw) register accessor: an alias for `Reg<AF1_SPEC>`
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
///TIM alternate function option register 1
pub mod af1;
///AF2 (rw) register accessor: an alias for `Reg<AF2_SPEC>`
pub type AF2 = crate::Reg<af2::AF2_SPEC>;
///TIM alternate function option register 2
pub mod af2;
///DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///control register
pub mod dcr;
///DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
///DMA address for full transfer
pub mod dmar;
