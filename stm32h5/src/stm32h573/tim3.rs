///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM3 control register 1
    pub tim3_cr1: TIM3_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM3 control register 2
    pub tim3_cr2: TIM3_CR2,
    ///0x08 - TIM3 slave mode control register
    pub tim3_smcr: TIM3_SMCR,
    ///0x0c - TIM3 DMA/Interrupt enable register
    pub tim3_dier: TIM3_DIER,
    ///0x10 - TIM3 status register
    pub tim3_sr: TIM3_SR,
    ///0x14 - TIM3 event generation register
    pub tim3_egr: TIM3_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim3_ccmr1: [u8; 0x04],
    _reserved_7_tim3_ccmr2: [u8; 0x04],
    ///0x20 - TIM3 capture/compare enable register
    pub tim3_ccer: TIM3_CCER,
    _reserved9: [u8; 0x02],
    ///0x24 - TIM3 counter
    pub tim3_cnt: TIM3_CNT,
    ///0x28 - TIM3 prescaler
    pub tim3_psc: TIM3_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM3 auto-reload register
    pub tim3_arr: TIM3_ARR,
    _reserved12: [u8; 0x04],
    ///0x34 - TIM3 capture/compare register 1
    pub tim3_ccr1: TIM3_CCR1,
    ///0x38 - TIM3 capture/compare register 2
    pub tim3_ccr2: TIM3_CCR2,
    ///0x3c - TIM3 capture/compare register 3
    pub tim3_ccr3: TIM3_CCR3,
    ///0x40 - TIM3 capture/compare register 4
    pub tim3_ccr4: TIM3_CCR4,
    _reserved16: [u8; 0x14],
    ///0x58 - TIM3 timer encoder control register
    pub tim3_ecr: TIM3_ECR,
    ///0x5c - TIM3 timer input selection register
    pub tim3_tisel: TIM3_TISEL,
    ///0x60 - TIM3 alternate function register 1
    pub tim3_af1: TIM3_AF1,
    ///0x64 - TIM3 alternate function register 2
    pub tim3_af2: TIM3_AF2,
    _reserved20: [u8; 0x0374],
    ///0x3dc - TIM3 DMA control register
    pub tim3_dcr: TIM3_DCR,
    ///0x3e0 - TIM3 DMA address for full transfer
    pub tim3_dmar: TIM3_DMAR,
}
impl RegisterBlock {
    ///0x18 - TIM3 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim3_ccmr1_output(&self) -> &TIM3_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - TIM3 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim3_ccmr1_input(&self) -> &TIM3_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x1c - TIM3 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim3_ccmr2_output(&self) -> &TIM3_CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - TIM3 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim3_ccmr2_input(&self) -> &TIM3_CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///TIM3_CR1 (rw) register accessor: an alias for `Reg<TIM3_CR1_SPEC>`
pub type TIM3_CR1 = crate::Reg<tim3_cr1::TIM3_CR1_SPEC>;
///TIM3 control register 1
pub mod tim3_cr1;
///TIM3_CR2 (rw) register accessor: an alias for `Reg<TIM3_CR2_SPEC>`
pub type TIM3_CR2 = crate::Reg<tim3_cr2::TIM3_CR2_SPEC>;
///TIM3 control register 2
pub mod tim3_cr2;
///TIM3_SMCR (rw) register accessor: an alias for `Reg<TIM3_SMCR_SPEC>`
pub type TIM3_SMCR = crate::Reg<tim3_smcr::TIM3_SMCR_SPEC>;
///TIM3 slave mode control register
pub mod tim3_smcr;
///TIM3_DIER (rw) register accessor: an alias for `Reg<TIM3_DIER_SPEC>`
pub type TIM3_DIER = crate::Reg<tim3_dier::TIM3_DIER_SPEC>;
///TIM3 DMA/Interrupt enable register
pub mod tim3_dier;
///TIM3_SR (rw) register accessor: an alias for `Reg<TIM3_SR_SPEC>`
pub type TIM3_SR = crate::Reg<tim3_sr::TIM3_SR_SPEC>;
///TIM3 status register
pub mod tim3_sr;
///TIM3_EGR (w) register accessor: an alias for `Reg<TIM3_EGR_SPEC>`
pub type TIM3_EGR = crate::Reg<tim3_egr::TIM3_EGR_SPEC>;
///TIM3 event generation register
pub mod tim3_egr;
///TIM3_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM3_CCMR1_INPUT_SPEC>`
pub type TIM3_CCMR1_INPUT = crate::Reg<tim3_ccmr1_input::TIM3_CCMR1_INPUT_SPEC>;
///TIM3 capture/compare mode register 1 \[alternate\]
pub mod tim3_ccmr1_input;
///TIM3_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM3_CCMR1_OUTPUT_SPEC>`
pub type TIM3_CCMR1_OUTPUT = crate::Reg<tim3_ccmr1_output::TIM3_CCMR1_OUTPUT_SPEC>;
///TIM3 capture/compare mode register 1 \[alternate\]
pub mod tim3_ccmr1_output;
///TIM3_CCMR2_Input (rw) register accessor: an alias for `Reg<TIM3_CCMR2_INPUT_SPEC>`
pub type TIM3_CCMR2_INPUT = crate::Reg<tim3_ccmr2_input::TIM3_CCMR2_INPUT_SPEC>;
///TIM3 capture/compare mode register 2 \[alternate\]
pub mod tim3_ccmr2_input;
///TIM3_CCMR2_Output (rw) register accessor: an alias for `Reg<TIM3_CCMR2_OUTPUT_SPEC>`
pub type TIM3_CCMR2_OUTPUT = crate::Reg<tim3_ccmr2_output::TIM3_CCMR2_OUTPUT_SPEC>;
///TIM3 capture/compare mode register 2 \[alternate\]
pub mod tim3_ccmr2_output;
///TIM3_CCER (rw) register accessor: an alias for `Reg<TIM3_CCER_SPEC>`
pub type TIM3_CCER = crate::Reg<tim3_ccer::TIM3_CCER_SPEC>;
///TIM3 capture/compare enable register
pub mod tim3_ccer;
///TIM3_CNT (rw) register accessor: an alias for `Reg<TIM3_CNT_SPEC>`
pub type TIM3_CNT = crate::Reg<tim3_cnt::TIM3_CNT_SPEC>;
///TIM3 counter
pub mod tim3_cnt;
///TIM3_PSC (rw) register accessor: an alias for `Reg<TIM3_PSC_SPEC>`
pub type TIM3_PSC = crate::Reg<tim3_psc::TIM3_PSC_SPEC>;
///TIM3 prescaler
pub mod tim3_psc;
///TIM3_ARR (rw) register accessor: an alias for `Reg<TIM3_ARR_SPEC>`
pub type TIM3_ARR = crate::Reg<tim3_arr::TIM3_ARR_SPEC>;
///TIM3 auto-reload register
pub mod tim3_arr;
///TIM3_CCR1 (rw) register accessor: an alias for `Reg<TIM3_CCR1_SPEC>`
pub type TIM3_CCR1 = crate::Reg<tim3_ccr1::TIM3_CCR1_SPEC>;
///TIM3 capture/compare register 1
pub mod tim3_ccr1;
///TIM3_CCR2 (rw) register accessor: an alias for `Reg<TIM3_CCR2_SPEC>`
pub type TIM3_CCR2 = crate::Reg<tim3_ccr2::TIM3_CCR2_SPEC>;
///TIM3 capture/compare register 2
pub mod tim3_ccr2;
///TIM3_CCR3 (rw) register accessor: an alias for `Reg<TIM3_CCR3_SPEC>`
pub type TIM3_CCR3 = crate::Reg<tim3_ccr3::TIM3_CCR3_SPEC>;
///TIM3 capture/compare register 3
pub mod tim3_ccr3;
///TIM3_CCR4 (rw) register accessor: an alias for `Reg<TIM3_CCR4_SPEC>`
pub type TIM3_CCR4 = crate::Reg<tim3_ccr4::TIM3_CCR4_SPEC>;
///TIM3 capture/compare register 4
pub mod tim3_ccr4;
///TIM3_ECR (rw) register accessor: an alias for `Reg<TIM3_ECR_SPEC>`
pub type TIM3_ECR = crate::Reg<tim3_ecr::TIM3_ECR_SPEC>;
///TIM3 timer encoder control register
pub mod tim3_ecr;
///TIM3_TISEL (rw) register accessor: an alias for `Reg<TIM3_TISEL_SPEC>`
pub type TIM3_TISEL = crate::Reg<tim3_tisel::TIM3_TISEL_SPEC>;
///TIM3 timer input selection register
pub mod tim3_tisel;
///TIM3_AF1 (rw) register accessor: an alias for `Reg<TIM3_AF1_SPEC>`
pub type TIM3_AF1 = crate::Reg<tim3_af1::TIM3_AF1_SPEC>;
///TIM3 alternate function register 1
pub mod tim3_af1;
///TIM3_AF2 (rw) register accessor: an alias for `Reg<TIM3_AF2_SPEC>`
pub type TIM3_AF2 = crate::Reg<tim3_af2::TIM3_AF2_SPEC>;
///TIM3 alternate function register 2
pub mod tim3_af2;
///TIM3_DCR (rw) register accessor: an alias for `Reg<TIM3_DCR_SPEC>`
pub type TIM3_DCR = crate::Reg<tim3_dcr::TIM3_DCR_SPEC>;
///TIM3 DMA control register
pub mod tim3_dcr;
///TIM3_DMAR (rw) register accessor: an alias for `Reg<TIM3_DMAR_SPEC>`
pub type TIM3_DMAR = crate::Reg<tim3_dmar::TIM3_DMAR_SPEC>;
///TIM3 DMA address for full transfer
pub mod tim3_dmar;
