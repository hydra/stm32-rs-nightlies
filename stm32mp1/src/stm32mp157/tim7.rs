///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM7 control register 1
    pub tim7_cr1: TIM7_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM7 control register 2
    pub tim7_cr2: TIM7_CR2,
    ///0x08 - TIM7 slave mode control register
    pub tim7_smcr: TIM7_SMCR,
    ///0x0c - TIM7 DMA/interrupt enable register
    pub tim7_dier: TIM7_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM7 status register
    pub tim7_sr: TIM7_SR,
    ///0x14 - TIM7 event generation register
    pub tim7_egr: TIM7_EGR,
    _reserved6: [u8; 0x02],
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim7_ccmr1alternate7: TIM7_CCMR1ALTERNATE7,
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim7_ccmr2alternate23: TIM7_CCMR2ALTERNATE23,
    ///0x20 - TIM7 capture/compare enable register
    pub tim7_ccer: TIM7_CCER,
    ///0x24 - TIM7 counter
    pub tim7_cnt: TIM7_CNT,
    ///0x28 - TIM7 prescaler
    pub tim7_psc: TIM7_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM7 auto-reload register
    pub tim7_arr: TIM7_ARR,
    _reserved12: [u8; 0x02],
    ///0x30 - TIM7 repetition counter register
    pub tim7_rcr: TIM7_RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM7 capture/compare register 1
    pub tim7_ccr1: TIM7_CCR1,
    _reserved14: [u8; 0x02],
    ///0x38 - TIM7 capture/compare register 2
    pub tim7_ccr2: TIM7_CCR2,
    _reserved15: [u8; 0x02],
    ///0x3c - TIM7 capture/compare register 3
    pub tim7_ccr3: TIM7_CCR3,
    _reserved16: [u8; 0x02],
    ///0x40 - TIM7 capture/compare register 4
    pub tim7_ccr4: TIM7_CCR4,
    _reserved17: [u8; 0x02],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub tim7_bdtr: TIM7_BDTR,
    ///0x48 - TIM7 DMA control register
    pub tim7_dcr: TIM7_DCR,
    _reserved19: [u8; 0x02],
    ///0x4c - TIM7 DMA address for full transfer
    pub tim7_dmar: TIM7_DMAR,
    _reserved20: [u8; 0x04],
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    pub tim7_ccmr3: TIM7_CCMR3,
    ///0x58 - TIM7 capture/compare register 5
    pub tim7_ccr5: TIM7_CCR5,
    ///0x5c - TIM7 capture/compare register 6
    pub tim7_ccr6: TIM7_CCR6,
}
///TIM7_CR1 (rw) register accessor: an alias for `Reg<TIM7_CR1_SPEC>`
pub type TIM7_CR1 = crate::Reg<tim7_cr1::TIM7_CR1_SPEC>;
///TIM7 control register 1
pub mod tim7_cr1;
///TIM7_CR2 (rw) register accessor: an alias for `Reg<TIM7_CR2_SPEC>`
pub type TIM7_CR2 = crate::Reg<tim7_cr2::TIM7_CR2_SPEC>;
///TIM7 control register 2
pub mod tim7_cr2;
///TIM7_SMCR (rw) register accessor: an alias for `Reg<TIM7_SMCR_SPEC>`
pub type TIM7_SMCR = crate::Reg<tim7_smcr::TIM7_SMCR_SPEC>;
///TIM7 slave mode control register
pub mod tim7_smcr;
///TIM7_DIER (rw) register accessor: an alias for `Reg<TIM7_DIER_SPEC>`
pub type TIM7_DIER = crate::Reg<tim7_dier::TIM7_DIER_SPEC>;
///TIM7 DMA/interrupt enable register
pub mod tim7_dier;
///TIM7_SR (rw) register accessor: an alias for `Reg<TIM7_SR_SPEC>`
pub type TIM7_SR = crate::Reg<tim7_sr::TIM7_SR_SPEC>;
///TIM7 status register
pub mod tim7_sr;
///TIM7_EGR (w) register accessor: an alias for `Reg<TIM7_EGR_SPEC>`
pub type TIM7_EGR = crate::Reg<tim7_egr::TIM7_EGR_SPEC>;
///TIM7 event generation register
pub mod tim7_egr;
///TIM7_CCMR1ALTERNATE7 (rw) register accessor: an alias for `Reg<TIM7_CCMR1ALTERNATE7_SPEC>`
pub type TIM7_CCMR1ALTERNATE7 = crate::Reg<tim7_ccmr1alternate7::TIM7_CCMR1ALTERNATE7_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim7_ccmr1alternate7;
///TIM7_CCMR2ALTERNATE23 (rw) register accessor: an alias for `Reg<TIM7_CCMR2ALTERNATE23_SPEC>`
pub type TIM7_CCMR2ALTERNATE23 = crate::Reg<tim7_ccmr2alternate23::TIM7_CCMR2ALTERNATE23_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim7_ccmr2alternate23;
///TIM7_CCER (rw) register accessor: an alias for `Reg<TIM7_CCER_SPEC>`
pub type TIM7_CCER = crate::Reg<tim7_ccer::TIM7_CCER_SPEC>;
///TIM7 capture/compare enable register
pub mod tim7_ccer;
///TIM7_CNT (rw) register accessor: an alias for `Reg<TIM7_CNT_SPEC>`
pub type TIM7_CNT = crate::Reg<tim7_cnt::TIM7_CNT_SPEC>;
///TIM7 counter
pub mod tim7_cnt;
///TIM7_PSC (rw) register accessor: an alias for `Reg<TIM7_PSC_SPEC>`
pub type TIM7_PSC = crate::Reg<tim7_psc::TIM7_PSC_SPEC>;
///TIM7 prescaler
pub mod tim7_psc;
///TIM7_ARR (rw) register accessor: an alias for `Reg<TIM7_ARR_SPEC>`
pub type TIM7_ARR = crate::Reg<tim7_arr::TIM7_ARR_SPEC>;
///TIM7 auto-reload register
pub mod tim7_arr;
///TIM7_RCR (rw) register accessor: an alias for `Reg<TIM7_RCR_SPEC>`
pub type TIM7_RCR = crate::Reg<tim7_rcr::TIM7_RCR_SPEC>;
///TIM7 repetition counter register
pub mod tim7_rcr;
///TIM7_CCR1 (rw) register accessor: an alias for `Reg<TIM7_CCR1_SPEC>`
pub type TIM7_CCR1 = crate::Reg<tim7_ccr1::TIM7_CCR1_SPEC>;
///TIM7 capture/compare register 1
pub mod tim7_ccr1;
///TIM7_CCR2 (rw) register accessor: an alias for `Reg<TIM7_CCR2_SPEC>`
pub type TIM7_CCR2 = crate::Reg<tim7_ccr2::TIM7_CCR2_SPEC>;
///TIM7 capture/compare register 2
pub mod tim7_ccr2;
///TIM7_CCR3 (rw) register accessor: an alias for `Reg<TIM7_CCR3_SPEC>`
pub type TIM7_CCR3 = crate::Reg<tim7_ccr3::TIM7_CCR3_SPEC>;
///TIM7 capture/compare register 3
pub mod tim7_ccr3;
///TIM7_CCR4 (rw) register accessor: an alias for `Reg<TIM7_CCR4_SPEC>`
pub type TIM7_CCR4 = crate::Reg<tim7_ccr4::TIM7_CCR4_SPEC>;
///TIM7 capture/compare register 4
pub mod tim7_ccr4;
///TIM7_BDTR (rw) register accessor: an alias for `Reg<TIM7_BDTR_SPEC>`
pub type TIM7_BDTR = crate::Reg<tim7_bdtr::TIM7_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod tim7_bdtr;
///TIM7_DCR (rw) register accessor: an alias for `Reg<TIM7_DCR_SPEC>`
pub type TIM7_DCR = crate::Reg<tim7_dcr::TIM7_DCR_SPEC>;
///TIM7 DMA control register
pub mod tim7_dcr;
///TIM7_DMAR (rw) register accessor: an alias for `Reg<TIM7_DMAR_SPEC>`
pub type TIM7_DMAR = crate::Reg<tim7_dmar::TIM7_DMAR_SPEC>;
///TIM7 DMA address for full transfer
pub mod tim7_dmar;
///TIM7_CCMR3 (rw) register accessor: an alias for `Reg<TIM7_CCMR3_SPEC>`
pub type TIM7_CCMR3 = crate::Reg<tim7_ccmr3::TIM7_CCMR3_SPEC>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim7_ccmr3;
///TIM7_CCR5 (rw) register accessor: an alias for `Reg<TIM7_CCR5_SPEC>`
pub type TIM7_CCR5 = crate::Reg<tim7_ccr5::TIM7_CCR5_SPEC>;
///TIM7 capture/compare register 5
pub mod tim7_ccr5;
///TIM7_CCR6 (rw) register accessor: an alias for `Reg<TIM7_CCR6_SPEC>`
pub type TIM7_CCR6 = crate::Reg<tim7_ccr6::TIM7_CCR6_SPEC>;
///TIM7 capture/compare register 6
pub mod tim7_ccr6;
