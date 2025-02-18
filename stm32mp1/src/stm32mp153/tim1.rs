///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM1 control register 1
    pub tim1_cr1: TIM1_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM1 control register 2
    pub tim1_cr2: TIM1_CR2,
    ///0x08 - TIM1 slave mode control register
    pub tim1_smcr: TIM1_SMCR,
    ///0x0c - TIM1 DMA/interrupt enable register
    pub tim1_dier: TIM1_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM1 status register
    pub tim1_sr: TIM1_SR,
    ///0x14 - TIM1 event generation register
    pub tim1_egr: TIM1_EGR,
    _reserved6: [u8; 0x02],
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim1_ccmr1alternate1: TIM1_CCMR1ALTERNATE1,
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim1_ccmr2alternate17: TIM1_CCMR2ALTERNATE17,
    ///0x20 - TIM1 capture/compare enable register
    pub tim1_ccer: TIM1_CCER,
    ///0x24 - TIM1 counter
    pub tim1_cnt: TIM1_CNT,
    ///0x28 - TIM1 prescaler
    pub tim1_psc: TIM1_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM1 auto-reload register
    pub tim1_arr: TIM1_ARR,
    _reserved12: [u8; 0x02],
    ///0x30 - TIM1 repetition counter register
    pub tim1_rcr: TIM1_RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM1 capture/compare register 1
    pub tim1_ccr1: TIM1_CCR1,
    _reserved14: [u8; 0x02],
    ///0x38 - TIM1 capture/compare register 2
    pub tim1_ccr2: TIM1_CCR2,
    _reserved15: [u8; 0x02],
    ///0x3c - TIM1 capture/compare register 3
    pub tim1_ccr3: TIM1_CCR3,
    _reserved16: [u8; 0x02],
    ///0x40 - TIM1 capture/compare register 4
    pub tim1_ccr4: TIM1_CCR4,
    _reserved17: [u8; 0x02],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub tim1_bdtr: TIM1_BDTR,
    ///0x48 - TIM1 DMA control register
    pub tim1_dcr: TIM1_DCR,
    _reserved19: [u8; 0x02],
    ///0x4c - TIM1 DMA address for full transfer
    pub tim1_dmar: TIM1_DMAR,
    _reserved20: [u8; 0x04],
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    pub tim1_ccmr3: TIM1_CCMR3,
    ///0x58 - TIM1 capture/compare register 5
    pub tim1_ccr5: TIM1_CCR5,
    ///0x5c - TIM1 capture/compare register 6
    pub tim1_ccr6: TIM1_CCR6,
    _reserved23: [u8; 0x02],
    ///0x60 - TIM1 alternate function option register 1
    pub tim1_af1: TIM1_AF1,
    ///0x64 - TIM1 Alternate function register 2
    pub tim1_af2: TIM1_AF2,
    ///0x68 - TIM1 timer input selection register
    pub tim1_tisel: TIM1_TISEL,
}
///TIM1_CR1 (rw) register accessor: an alias for `Reg<TIM1_CR1_SPEC>`
pub type TIM1_CR1 = crate::Reg<tim1_cr1::TIM1_CR1_SPEC>;
///TIM1 control register 1
pub mod tim1_cr1;
///TIM1_CR2 (rw) register accessor: an alias for `Reg<TIM1_CR2_SPEC>`
pub type TIM1_CR2 = crate::Reg<tim1_cr2::TIM1_CR2_SPEC>;
///TIM1 control register 2
pub mod tim1_cr2;
///TIM1_SMCR (rw) register accessor: an alias for `Reg<TIM1_SMCR_SPEC>`
pub type TIM1_SMCR = crate::Reg<tim1_smcr::TIM1_SMCR_SPEC>;
///TIM1 slave mode control register
pub mod tim1_smcr;
///TIM1_DIER (rw) register accessor: an alias for `Reg<TIM1_DIER_SPEC>`
pub type TIM1_DIER = crate::Reg<tim1_dier::TIM1_DIER_SPEC>;
///TIM1 DMA/interrupt enable register
pub mod tim1_dier;
///TIM1_SR (rw) register accessor: an alias for `Reg<TIM1_SR_SPEC>`
pub type TIM1_SR = crate::Reg<tim1_sr::TIM1_SR_SPEC>;
///TIM1 status register
pub mod tim1_sr;
///TIM1_EGR (w) register accessor: an alias for `Reg<TIM1_EGR_SPEC>`
pub type TIM1_EGR = crate::Reg<tim1_egr::TIM1_EGR_SPEC>;
///TIM1 event generation register
pub mod tim1_egr;
///TIM1_CCMR1ALTERNATE1 (rw) register accessor: an alias for `Reg<TIM1_CCMR1ALTERNATE1_SPEC>`
pub type TIM1_CCMR1ALTERNATE1 = crate::Reg<tim1_ccmr1alternate1::TIM1_CCMR1ALTERNATE1_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim1_ccmr1alternate1;
///TIM1_CCMR2ALTERNATE17 (rw) register accessor: an alias for `Reg<TIM1_CCMR2ALTERNATE17_SPEC>`
pub type TIM1_CCMR2ALTERNATE17 = crate::Reg<tim1_ccmr2alternate17::TIM1_CCMR2ALTERNATE17_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim1_ccmr2alternate17;
///TIM1_CCER (rw) register accessor: an alias for `Reg<TIM1_CCER_SPEC>`
pub type TIM1_CCER = crate::Reg<tim1_ccer::TIM1_CCER_SPEC>;
///TIM1 capture/compare enable register
pub mod tim1_ccer;
///TIM1_CNT (rw) register accessor: an alias for `Reg<TIM1_CNT_SPEC>`
pub type TIM1_CNT = crate::Reg<tim1_cnt::TIM1_CNT_SPEC>;
///TIM1 counter
pub mod tim1_cnt;
///TIM1_PSC (rw) register accessor: an alias for `Reg<TIM1_PSC_SPEC>`
pub type TIM1_PSC = crate::Reg<tim1_psc::TIM1_PSC_SPEC>;
///TIM1 prescaler
pub mod tim1_psc;
///TIM1_ARR (rw) register accessor: an alias for `Reg<TIM1_ARR_SPEC>`
pub type TIM1_ARR = crate::Reg<tim1_arr::TIM1_ARR_SPEC>;
///TIM1 auto-reload register
pub mod tim1_arr;
///TIM1_RCR (rw) register accessor: an alias for `Reg<TIM1_RCR_SPEC>`
pub type TIM1_RCR = crate::Reg<tim1_rcr::TIM1_RCR_SPEC>;
///TIM1 repetition counter register
pub mod tim1_rcr;
///TIM1_CCR1 (rw) register accessor: an alias for `Reg<TIM1_CCR1_SPEC>`
pub type TIM1_CCR1 = crate::Reg<tim1_ccr1::TIM1_CCR1_SPEC>;
///TIM1 capture/compare register 1
pub mod tim1_ccr1;
///TIM1_CCR2 (rw) register accessor: an alias for `Reg<TIM1_CCR2_SPEC>`
pub type TIM1_CCR2 = crate::Reg<tim1_ccr2::TIM1_CCR2_SPEC>;
///TIM1 capture/compare register 2
pub mod tim1_ccr2;
///TIM1_CCR3 (rw) register accessor: an alias for `Reg<TIM1_CCR3_SPEC>`
pub type TIM1_CCR3 = crate::Reg<tim1_ccr3::TIM1_CCR3_SPEC>;
///TIM1 capture/compare register 3
pub mod tim1_ccr3;
///TIM1_CCR4 (rw) register accessor: an alias for `Reg<TIM1_CCR4_SPEC>`
pub type TIM1_CCR4 = crate::Reg<tim1_ccr4::TIM1_CCR4_SPEC>;
///TIM1 capture/compare register 4
pub mod tim1_ccr4;
///TIM1_BDTR (rw) register accessor: an alias for `Reg<TIM1_BDTR_SPEC>`
pub type TIM1_BDTR = crate::Reg<tim1_bdtr::TIM1_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod tim1_bdtr;
///TIM1_DCR (rw) register accessor: an alias for `Reg<TIM1_DCR_SPEC>`
pub type TIM1_DCR = crate::Reg<tim1_dcr::TIM1_DCR_SPEC>;
///TIM1 DMA control register
pub mod tim1_dcr;
///TIM1_DMAR (rw) register accessor: an alias for `Reg<TIM1_DMAR_SPEC>`
pub type TIM1_DMAR = crate::Reg<tim1_dmar::TIM1_DMAR_SPEC>;
///TIM1 DMA address for full transfer
pub mod tim1_dmar;
///TIM1_CCMR3 (rw) register accessor: an alias for `Reg<TIM1_CCMR3_SPEC>`
pub type TIM1_CCMR3 = crate::Reg<tim1_ccmr3::TIM1_CCMR3_SPEC>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim1_ccmr3;
///TIM1_CCR5 (rw) register accessor: an alias for `Reg<TIM1_CCR5_SPEC>`
pub type TIM1_CCR5 = crate::Reg<tim1_ccr5::TIM1_CCR5_SPEC>;
///TIM1 capture/compare register 5
pub mod tim1_ccr5;
///TIM1_CCR6 (rw) register accessor: an alias for `Reg<TIM1_CCR6_SPEC>`
pub type TIM1_CCR6 = crate::Reg<tim1_ccr6::TIM1_CCR6_SPEC>;
///TIM1 capture/compare register 6
pub mod tim1_ccr6;
///TIM1_AF1 (rw) register accessor: an alias for `Reg<TIM1_AF1_SPEC>`
pub type TIM1_AF1 = crate::Reg<tim1_af1::TIM1_AF1_SPEC>;
///TIM1 alternate function option register 1
pub mod tim1_af1;
///TIM1_AF2 (rw) register accessor: an alias for `Reg<TIM1_AF2_SPEC>`
pub type TIM1_AF2 = crate::Reg<tim1_af2::TIM1_AF2_SPEC>;
///TIM1 Alternate function register 2
pub mod tim1_af2;
///TIM1_TISEL (rw) register accessor: an alias for `Reg<TIM1_TISEL_SPEC>`
pub type TIM1_TISEL = crate::Reg<tim1_tisel::TIM1_TISEL_SPEC>;
///TIM1 timer input selection register
pub mod tim1_tisel;
