///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM4 control register 1
    pub tim4_cr1: TIM4_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM4 control register 2
    pub tim4_cr2: TIM4_CR2,
    ///0x08 - TIM4 slave mode control register
    pub tim4_smcr: TIM4_SMCR,
    ///0x0c - TIM4 DMA/Interrupt enable register
    pub tim4_dier: TIM4_DIER,
    ///0x10 - TIM4 status register
    pub tim4_sr: TIM4_SR,
    ///0x14 - TIM4 event generation register
    pub tim4_egr: TIM4_EGR,
    _reserved6: [u8; 0x02],
    _reserved_6_tim4_ccmr1: [u8; 0x04],
    _reserved_7_tim4_ccmr2: [u8; 0x04],
    ///0x20 - TIM4 capture/compare enable register
    pub tim4_ccer: TIM4_CCER,
    _reserved9: [u8; 0x02],
    ///0x24 - TIM4 counter
    pub tim4_cnt: TIM4_CNT,
    ///0x28 - TIM4 prescaler
    pub tim4_psc: TIM4_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM4 auto-reload register
    pub tim4_arr: TIM4_ARR,
    _reserved12: [u8; 0x04],
    ///0x34 - TIM4 capture/compare register 1
    pub tim4_ccr1: TIM4_CCR1,
    ///0x38 - TIM4 capture/compare register 2
    pub tim4_ccr2: TIM4_CCR2,
    ///0x3c - TIM4 capture/compare register 3
    pub tim4_ccr3: TIM4_CCR3,
    ///0x40 - TIM4 capture/compare register 4
    pub tim4_ccr4: TIM4_CCR4,
    _reserved16: [u8; 0x14],
    ///0x58 - TIM4 timer encoder control register
    pub tim4_ecr: TIM4_ECR,
    ///0x5c - TIM4 timer input selection register
    pub tim4_tisel: TIM4_TISEL,
    ///0x60 - TIM4 alternate function register 1
    pub tim4_af1: TIM4_AF1,
    ///0x64 - TIM4 alternate function register 2
    pub tim4_af2: TIM4_AF2,
    _reserved20: [u8; 0x0374],
    ///0x3dc - TIM4 DMA control register
    pub tim4_dcr: TIM4_DCR,
    ///0x3e0 - TIM4 DMA address for full transfer
    pub tim4_dmar: TIM4_DMAR,
}
impl RegisterBlock {
    ///0x18 - TIM4 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim4_ccmr1_output(&self) -> &TIM4_CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x18 - TIM4 capture/compare mode register 1 \[alternate\]
    #[inline(always)]
    pub const fn tim4_ccmr1_input(&self) -> &TIM4_CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x1c - TIM4 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim4_ccmr2_output(&self) -> &TIM4_CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - TIM4 capture/compare mode register 2 \[alternate\]
    #[inline(always)]
    pub const fn tim4_ccmr2_input(&self) -> &TIM4_CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///TIM4_CR1 (rw) register accessor: an alias for `Reg<TIM4_CR1_SPEC>`
pub type TIM4_CR1 = crate::Reg<tim4_cr1::TIM4_CR1_SPEC>;
///TIM4 control register 1
pub mod tim4_cr1;
///TIM4_CR2 (rw) register accessor: an alias for `Reg<TIM4_CR2_SPEC>`
pub type TIM4_CR2 = crate::Reg<tim4_cr2::TIM4_CR2_SPEC>;
///TIM4 control register 2
pub mod tim4_cr2;
///TIM4_SMCR (rw) register accessor: an alias for `Reg<TIM4_SMCR_SPEC>`
pub type TIM4_SMCR = crate::Reg<tim4_smcr::TIM4_SMCR_SPEC>;
///TIM4 slave mode control register
pub mod tim4_smcr;
///TIM4_DIER (rw) register accessor: an alias for `Reg<TIM4_DIER_SPEC>`
pub type TIM4_DIER = crate::Reg<tim4_dier::TIM4_DIER_SPEC>;
///TIM4 DMA/Interrupt enable register
pub mod tim4_dier;
///TIM4_SR (rw) register accessor: an alias for `Reg<TIM4_SR_SPEC>`
pub type TIM4_SR = crate::Reg<tim4_sr::TIM4_SR_SPEC>;
///TIM4 status register
pub mod tim4_sr;
///TIM4_EGR (w) register accessor: an alias for `Reg<TIM4_EGR_SPEC>`
pub type TIM4_EGR = crate::Reg<tim4_egr::TIM4_EGR_SPEC>;
///TIM4 event generation register
pub mod tim4_egr;
///TIM4_CCMR1_Input (rw) register accessor: an alias for `Reg<TIM4_CCMR1_INPUT_SPEC>`
pub type TIM4_CCMR1_INPUT = crate::Reg<tim4_ccmr1_input::TIM4_CCMR1_INPUT_SPEC>;
///TIM4 capture/compare mode register 1 \[alternate\]
pub mod tim4_ccmr1_input;
///TIM4_CCMR1_Output (rw) register accessor: an alias for `Reg<TIM4_CCMR1_OUTPUT_SPEC>`
pub type TIM4_CCMR1_OUTPUT = crate::Reg<tim4_ccmr1_output::TIM4_CCMR1_OUTPUT_SPEC>;
///TIM4 capture/compare mode register 1 \[alternate\]
pub mod tim4_ccmr1_output;
///TIM4_CCMR2_Input (rw) register accessor: an alias for `Reg<TIM4_CCMR2_INPUT_SPEC>`
pub type TIM4_CCMR2_INPUT = crate::Reg<tim4_ccmr2_input::TIM4_CCMR2_INPUT_SPEC>;
///TIM4 capture/compare mode register 2 \[alternate\]
pub mod tim4_ccmr2_input;
///TIM4_CCMR2_Output (rw) register accessor: an alias for `Reg<TIM4_CCMR2_OUTPUT_SPEC>`
pub type TIM4_CCMR2_OUTPUT = crate::Reg<tim4_ccmr2_output::TIM4_CCMR2_OUTPUT_SPEC>;
///TIM4 capture/compare mode register 2 \[alternate\]
pub mod tim4_ccmr2_output;
///TIM4_CCER (rw) register accessor: an alias for `Reg<TIM4_CCER_SPEC>`
pub type TIM4_CCER = crate::Reg<tim4_ccer::TIM4_CCER_SPEC>;
///TIM4 capture/compare enable register
pub mod tim4_ccer;
///TIM4_CNT (rw) register accessor: an alias for `Reg<TIM4_CNT_SPEC>`
pub type TIM4_CNT = crate::Reg<tim4_cnt::TIM4_CNT_SPEC>;
///TIM4 counter
pub mod tim4_cnt;
///TIM4_PSC (rw) register accessor: an alias for `Reg<TIM4_PSC_SPEC>`
pub type TIM4_PSC = crate::Reg<tim4_psc::TIM4_PSC_SPEC>;
///TIM4 prescaler
pub mod tim4_psc;
///TIM4_ARR (rw) register accessor: an alias for `Reg<TIM4_ARR_SPEC>`
pub type TIM4_ARR = crate::Reg<tim4_arr::TIM4_ARR_SPEC>;
///TIM4 auto-reload register
pub mod tim4_arr;
///TIM4_CCR1 (rw) register accessor: an alias for `Reg<TIM4_CCR1_SPEC>`
pub type TIM4_CCR1 = crate::Reg<tim4_ccr1::TIM4_CCR1_SPEC>;
///TIM4 capture/compare register 1
pub mod tim4_ccr1;
///TIM4_CCR2 (rw) register accessor: an alias for `Reg<TIM4_CCR2_SPEC>`
pub type TIM4_CCR2 = crate::Reg<tim4_ccr2::TIM4_CCR2_SPEC>;
///TIM4 capture/compare register 2
pub mod tim4_ccr2;
///TIM4_CCR3 (rw) register accessor: an alias for `Reg<TIM4_CCR3_SPEC>`
pub type TIM4_CCR3 = crate::Reg<tim4_ccr3::TIM4_CCR3_SPEC>;
///TIM4 capture/compare register 3
pub mod tim4_ccr3;
///TIM4_CCR4 (rw) register accessor: an alias for `Reg<TIM4_CCR4_SPEC>`
pub type TIM4_CCR4 = crate::Reg<tim4_ccr4::TIM4_CCR4_SPEC>;
///TIM4 capture/compare register 4
pub mod tim4_ccr4;
///TIM4_ECR (rw) register accessor: an alias for `Reg<TIM4_ECR_SPEC>`
pub type TIM4_ECR = crate::Reg<tim4_ecr::TIM4_ECR_SPEC>;
///TIM4 timer encoder control register
pub mod tim4_ecr;
///TIM4_TISEL (rw) register accessor: an alias for `Reg<TIM4_TISEL_SPEC>`
pub type TIM4_TISEL = crate::Reg<tim4_tisel::TIM4_TISEL_SPEC>;
///TIM4 timer input selection register
pub mod tim4_tisel;
///TIM4_AF1 (rw) register accessor: an alias for `Reg<TIM4_AF1_SPEC>`
pub type TIM4_AF1 = crate::Reg<tim4_af1::TIM4_AF1_SPEC>;
///TIM4 alternate function register 1
pub mod tim4_af1;
///TIM4_AF2 (rw) register accessor: an alias for `Reg<TIM4_AF2_SPEC>`
pub type TIM4_AF2 = crate::Reg<tim4_af2::TIM4_AF2_SPEC>;
///TIM4 alternate function register 2
pub mod tim4_af2;
///TIM4_DCR (rw) register accessor: an alias for `Reg<TIM4_DCR_SPEC>`
pub type TIM4_DCR = crate::Reg<tim4_dcr::TIM4_DCR_SPEC>;
///TIM4 DMA control register
pub mod tim4_dcr;
///TIM4_DMAR (rw) register accessor: an alias for `Reg<TIM4_DMAR_SPEC>`
pub type TIM4_DMAR = crate::Reg<tim4_dmar::TIM4_DMAR_SPEC>;
///TIM4 DMA address for full transfer
pub mod tim4_dmar;
