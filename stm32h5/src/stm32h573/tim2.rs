///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM2 control register 1
    pub tim2_cr1: TIM2_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM2 control register 2
    pub tim2_cr2: TIM2_CR2,
    ///0x08 - TIM2 slave mode control register
    pub tim2_smcr: TIM2_SMCR,
    ///0x0c - TIM2 DMA/Interrupt enable register
    pub tim2_dier: TIM2_DIER,
    ///0x10 - TIM2 status register
    pub tim2_sr: TIM2_SR,
    ///0x14 - TIM2 event generation register
    pub tim2_egr: TIM2_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim2_ccmr1: [u8; 0x04],
    _reserved_7_tim2_ccmr2: [u8; 0x04],
    ///0x20 - TIM2 capture/compare enable register
    pub tim2_ccer: TIM2_CCER,
    _reserved9: [u8; 0x02],
    ///0x24 - TIM2 counter
    pub tim2_cnt: TIM2_CNT,
    ///0x28 - TIM2 prescaler
    pub tim2_psc: TIM2_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM2 auto-reload register
    pub tim2_arr: TIM2_ARR,
    _reserved12: [u8; 0x04],
    ///0x34 - TIM2 capture/compare register 1
    pub tim2_ccr1: TIM2_CCR1,
    ///0x38 - TIM2 capture/compare register 2
    pub tim2_ccr2: TIM2_CCR2,
    ///0x3c - TIM2 capture/compare register 3
    pub tim2_ccr3: TIM2_CCR3,
    ///0x40 - TIM2 capture/compare register 4
    pub tim2_ccr4: TIM2_CCR4,
    _reserved16: [u8; 0x14],
    ///0x58 - TIM2 timer encoder control register
    pub tim2_ecr: TIM2_ECR,
    ///0x5c - TIM2 timer input selection register
    pub tim2_tisel: TIM2_TISEL,
    ///0x60 - TIM2 alternate function register 1
    pub tim2_af1: TIM2_AF1,
    ///0x64 - TIM2 alternate function register 2
    pub tim2_af2: TIM2_AF2,
    _reserved20: [u8; 0x0374],
    ///0x3dc - TIM2 DMA control register
    pub tim2_dcr: TIM2_DCR,
    ///0x3e0 - TIM2 DMA address for full transfer
    pub tim2_dmar: TIM2_DMAR,
}
impl RegisterBlock {
    ///0x18 - TIM2 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim2_ccmr1_output(&self) -> &TIM2_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - TIM2 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim2_ccmr1_input(&self) -> &TIM2_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x1c - TIM2 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim2_ccmr2_output(&self) -> &TIM2_CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - TIM2 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim2_ccmr2_input(&self) -> &TIM2_CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///TIM2_CR1 (rw) register accessor: an alias for `Reg<TIM2_CR1_SPEC>`
pub type TIM2_CR1 = crate::Reg<tim2_cr1::TIM2_CR1_SPEC>;
///TIM2 control register 1
pub mod tim2_cr1;
///TIM2_CR2 (rw) register accessor: an alias for `Reg<TIM2_CR2_SPEC>`
pub type TIM2_CR2 = crate::Reg<tim2_cr2::TIM2_CR2_SPEC>;
///TIM2 control register 2
pub mod tim2_cr2;
///TIM2_SMCR (rw) register accessor: an alias for `Reg<TIM2_SMCR_SPEC>`
pub type TIM2_SMCR = crate::Reg<tim2_smcr::TIM2_SMCR_SPEC>;
///TIM2 slave mode control register
pub mod tim2_smcr;
///TIM2_DIER (rw) register accessor: an alias for `Reg<TIM2_DIER_SPEC>`
pub type TIM2_DIER = crate::Reg<tim2_dier::TIM2_DIER_SPEC>;
///TIM2 DMA/Interrupt enable register
pub mod tim2_dier;
///TIM2_SR (rw) register accessor: an alias for `Reg<TIM2_SR_SPEC>`
pub type TIM2_SR = crate::Reg<tim2_sr::TIM2_SR_SPEC>;
///TIM2 status register
pub mod tim2_sr;
///TIM2_EGR (w) register accessor: an alias for `Reg<TIM2_EGR_SPEC>`
pub type TIM2_EGR = crate::Reg<tim2_egr::TIM2_EGR_SPEC>;
///TIM2 event generation register
pub mod tim2_egr;
///TIM2_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM2_CCMR1_INPUT_SPEC>`
pub type TIM2_CCMR1_INPUT = crate::Reg<tim2_ccmr1_input::TIM2_CCMR1_INPUT_SPEC>;
///TIM2 capture/compare mode register 1 \[alternate\]
pub mod tim2_ccmr1_input;
///TIM2_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM2_CCMR1_OUTPUT_SPEC>`
pub type TIM2_CCMR1_OUTPUT = crate::Reg<tim2_ccmr1_output::TIM2_CCMR1_OUTPUT_SPEC>;
///TIM2 capture/compare mode register 1 \[alternate\]
pub mod tim2_ccmr1_output;
///TIM2_CCMR2_Input (rw) register accessor: an alias for `Reg<TIM2_CCMR2_INPUT_SPEC>`
pub type TIM2_CCMR2_INPUT = crate::Reg<tim2_ccmr2_input::TIM2_CCMR2_INPUT_SPEC>;
///TIM2 capture/compare mode register 2 \[alternate\]
pub mod tim2_ccmr2_input;
///TIM2_CCMR2_Output (rw) register accessor: an alias for `Reg<TIM2_CCMR2_OUTPUT_SPEC>`
pub type TIM2_CCMR2_OUTPUT = crate::Reg<tim2_ccmr2_output::TIM2_CCMR2_OUTPUT_SPEC>;
///TIM2 capture/compare mode register 2 \[alternate\]
pub mod tim2_ccmr2_output;
///TIM2_CCER (rw) register accessor: an alias for `Reg<TIM2_CCER_SPEC>`
pub type TIM2_CCER = crate::Reg<tim2_ccer::TIM2_CCER_SPEC>;
///TIM2 capture/compare enable register
pub mod tim2_ccer;
///TIM2_CNT (rw) register accessor: an alias for `Reg<TIM2_CNT_SPEC>`
pub type TIM2_CNT = crate::Reg<tim2_cnt::TIM2_CNT_SPEC>;
///TIM2 counter
pub mod tim2_cnt;
///TIM2_PSC (rw) register accessor: an alias for `Reg<TIM2_PSC_SPEC>`
pub type TIM2_PSC = crate::Reg<tim2_psc::TIM2_PSC_SPEC>;
///TIM2 prescaler
pub mod tim2_psc;
///TIM2_ARR (rw) register accessor: an alias for `Reg<TIM2_ARR_SPEC>`
pub type TIM2_ARR = crate::Reg<tim2_arr::TIM2_ARR_SPEC>;
///TIM2 auto-reload register
pub mod tim2_arr;
///TIM2_CCR1 (rw) register accessor: an alias for `Reg<TIM2_CCR1_SPEC>`
pub type TIM2_CCR1 = crate::Reg<tim2_ccr1::TIM2_CCR1_SPEC>;
///TIM2 capture/compare register 1
pub mod tim2_ccr1;
///TIM2_CCR2 (rw) register accessor: an alias for `Reg<TIM2_CCR2_SPEC>`
pub type TIM2_CCR2 = crate::Reg<tim2_ccr2::TIM2_CCR2_SPEC>;
///TIM2 capture/compare register 2
pub mod tim2_ccr2;
///TIM2_CCR3 (rw) register accessor: an alias for `Reg<TIM2_CCR3_SPEC>`
pub type TIM2_CCR3 = crate::Reg<tim2_ccr3::TIM2_CCR3_SPEC>;
///TIM2 capture/compare register 3
pub mod tim2_ccr3;
///TIM2_CCR4 (rw) register accessor: an alias for `Reg<TIM2_CCR4_SPEC>`
pub type TIM2_CCR4 = crate::Reg<tim2_ccr4::TIM2_CCR4_SPEC>;
///TIM2 capture/compare register 4
pub mod tim2_ccr4;
///TIM2_ECR (rw) register accessor: an alias for `Reg<TIM2_ECR_SPEC>`
pub type TIM2_ECR = crate::Reg<tim2_ecr::TIM2_ECR_SPEC>;
///TIM2 timer encoder control register
pub mod tim2_ecr;
///TIM2_TISEL (rw) register accessor: an alias for `Reg<TIM2_TISEL_SPEC>`
pub type TIM2_TISEL = crate::Reg<tim2_tisel::TIM2_TISEL_SPEC>;
///TIM2 timer input selection register
pub mod tim2_tisel;
///TIM2_AF1 (rw) register accessor: an alias for `Reg<TIM2_AF1_SPEC>`
pub type TIM2_AF1 = crate::Reg<tim2_af1::TIM2_AF1_SPEC>;
///TIM2 alternate function register 1
pub mod tim2_af1;
///TIM2_AF2 (rw) register accessor: an alias for `Reg<TIM2_AF2_SPEC>`
pub type TIM2_AF2 = crate::Reg<tim2_af2::TIM2_AF2_SPEC>;
///TIM2 alternate function register 2
pub mod tim2_af2;
///TIM2_DCR (rw) register accessor: an alias for `Reg<TIM2_DCR_SPEC>`
pub type TIM2_DCR = crate::Reg<tim2_dcr::TIM2_DCR_SPEC>;
///TIM2 DMA control register
pub mod tim2_dcr;
///TIM2_DMAR (rw) register accessor: an alias for `Reg<TIM2_DMAR_SPEC>`
pub type TIM2_DMAR = crate::Reg<tim2_dmar::TIM2_DMAR_SPEC>;
///TIM2 DMA address for full transfer
pub mod tim2_dmar;
