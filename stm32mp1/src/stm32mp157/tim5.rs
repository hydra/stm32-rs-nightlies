///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM5 control register 1
    pub tim5_cr1: TIM5_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM5 control register 2
    pub tim5_cr2: TIM5_CR2,
    ///0x08 - TIM5 slave mode control register
    pub tim5_smcr: TIM5_SMCR,
    ///0x0c - TIM5 DMA/interrupt enable register
    pub tim5_dier: TIM5_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM5 status register
    pub tim5_sr: TIM5_SR,
    ///0x14 - TIM5 event generation register
    pub tim5_egr: TIM5_EGR,
    _reserved6: [u8; 0x02],
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim5_ccmr1alternate5: TIM5_CCMR1ALTERNATE5,
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim5_ccmr2alternate21: TIM5_CCMR2ALTERNATE21,
    ///0x20 - TIM5 capture/compare enable register
    pub tim5_ccer: TIM5_CCER,
    ///0x24 - TIM5 counter
    pub tim5_cnt: TIM5_CNT,
    ///0x28 - TIM5 prescaler
    pub tim5_psc: TIM5_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM5 auto-reload register
    pub tim5_arr: TIM5_ARR,
    _reserved12: [u8; 0x02],
    ///0x30 - TIM5 repetition counter register
    pub tim5_rcr: TIM5_RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM5 capture/compare register 1
    pub tim5_ccr1: TIM5_CCR1,
    _reserved14: [u8; 0x02],
    ///0x38 - TIM5 capture/compare register 2
    pub tim5_ccr2: TIM5_CCR2,
    _reserved15: [u8; 0x02],
    ///0x3c - TIM5 capture/compare register 3
    pub tim5_ccr3: TIM5_CCR3,
    _reserved16: [u8; 0x02],
    ///0x40 - TIM5 capture/compare register 4
    pub tim5_ccr4: TIM5_CCR4,
    _reserved17: [u8; 0x02],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub tim5_bdtr: TIM5_BDTR,
    ///0x48 - TIM5 DMA control register
    pub tim5_dcr: TIM5_DCR,
    _reserved19: [u8; 0x02],
    ///0x4c - TIM5 DMA address for full transfer
    pub tim5_dmar: TIM5_DMAR,
    _reserved20: [u8; 0x04],
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    pub tim5_ccmr3: TIM5_CCMR3,
    ///0x58 - TIM5 capture/compare register 5
    pub tim5_ccr5: TIM5_CCR5,
    ///0x5c - TIM5 capture/compare register 6
    pub tim5_ccr6: TIM5_CCR6,
}
///TIM5_CR1 (rw) register accessor: an alias for `Reg<TIM5_CR1_SPEC>`
pub type TIM5_CR1 = crate::Reg<tim5_cr1::TIM5_CR1_SPEC>;
///TIM5 control register 1
pub mod tim5_cr1;
///TIM5_CR2 (rw) register accessor: an alias for `Reg<TIM5_CR2_SPEC>`
pub type TIM5_CR2 = crate::Reg<tim5_cr2::TIM5_CR2_SPEC>;
///TIM5 control register 2
pub mod tim5_cr2;
///TIM5_SMCR (rw) register accessor: an alias for `Reg<TIM5_SMCR_SPEC>`
pub type TIM5_SMCR = crate::Reg<tim5_smcr::TIM5_SMCR_SPEC>;
///TIM5 slave mode control register
pub mod tim5_smcr;
///TIM5_DIER (rw) register accessor: an alias for `Reg<TIM5_DIER_SPEC>`
pub type TIM5_DIER = crate::Reg<tim5_dier::TIM5_DIER_SPEC>;
///TIM5 DMA/interrupt enable register
pub mod tim5_dier;
///TIM5_SR (rw) register accessor: an alias for `Reg<TIM5_SR_SPEC>`
pub type TIM5_SR = crate::Reg<tim5_sr::TIM5_SR_SPEC>;
///TIM5 status register
pub mod tim5_sr;
///TIM5_EGR (w) register accessor: an alias for `Reg<TIM5_EGR_SPEC>`
pub type TIM5_EGR = crate::Reg<tim5_egr::TIM5_EGR_SPEC>;
///TIM5 event generation register
pub mod tim5_egr;
///TIM5_CCMR1ALTERNATE5 (rw) register accessor: an alias for `Reg<TIM5_CCMR1ALTERNATE5_SPEC>`
pub type TIM5_CCMR1ALTERNATE5 = crate::Reg<tim5_ccmr1alternate5::TIM5_CCMR1ALTERNATE5_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim5_ccmr1alternate5;
///TIM5_CCMR2ALTERNATE21 (rw) register accessor: an alias for `Reg<TIM5_CCMR2ALTERNATE21_SPEC>`
pub type TIM5_CCMR2ALTERNATE21 = crate::Reg<tim5_ccmr2alternate21::TIM5_CCMR2ALTERNATE21_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim5_ccmr2alternate21;
///TIM5_CCER (rw) register accessor: an alias for `Reg<TIM5_CCER_SPEC>`
pub type TIM5_CCER = crate::Reg<tim5_ccer::TIM5_CCER_SPEC>;
///TIM5 capture/compare enable register
pub mod tim5_ccer;
///TIM5_CNT (rw) register accessor: an alias for `Reg<TIM5_CNT_SPEC>`
pub type TIM5_CNT = crate::Reg<tim5_cnt::TIM5_CNT_SPEC>;
///TIM5 counter
pub mod tim5_cnt;
///TIM5_PSC (rw) register accessor: an alias for `Reg<TIM5_PSC_SPEC>`
pub type TIM5_PSC = crate::Reg<tim5_psc::TIM5_PSC_SPEC>;
///TIM5 prescaler
pub mod tim5_psc;
///TIM5_ARR (rw) register accessor: an alias for `Reg<TIM5_ARR_SPEC>`
pub type TIM5_ARR = crate::Reg<tim5_arr::TIM5_ARR_SPEC>;
///TIM5 auto-reload register
pub mod tim5_arr;
///TIM5_RCR (rw) register accessor: an alias for `Reg<TIM5_RCR_SPEC>`
pub type TIM5_RCR = crate::Reg<tim5_rcr::TIM5_RCR_SPEC>;
///TIM5 repetition counter register
pub mod tim5_rcr;
///TIM5_CCR1 (rw) register accessor: an alias for `Reg<TIM5_CCR1_SPEC>`
pub type TIM5_CCR1 = crate::Reg<tim5_ccr1::TIM5_CCR1_SPEC>;
///TIM5 capture/compare register 1
pub mod tim5_ccr1;
///TIM5_CCR2 (rw) register accessor: an alias for `Reg<TIM5_CCR2_SPEC>`
pub type TIM5_CCR2 = crate::Reg<tim5_ccr2::TIM5_CCR2_SPEC>;
///TIM5 capture/compare register 2
pub mod tim5_ccr2;
///TIM5_CCR3 (rw) register accessor: an alias for `Reg<TIM5_CCR3_SPEC>`
pub type TIM5_CCR3 = crate::Reg<tim5_ccr3::TIM5_CCR3_SPEC>;
///TIM5 capture/compare register 3
pub mod tim5_ccr3;
///TIM5_CCR4 (rw) register accessor: an alias for `Reg<TIM5_CCR4_SPEC>`
pub type TIM5_CCR4 = crate::Reg<tim5_ccr4::TIM5_CCR4_SPEC>;
///TIM5 capture/compare register 4
pub mod tim5_ccr4;
///TIM5_BDTR (rw) register accessor: an alias for `Reg<TIM5_BDTR_SPEC>`
pub type TIM5_BDTR = crate::Reg<tim5_bdtr::TIM5_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod tim5_bdtr;
///TIM5_DCR (rw) register accessor: an alias for `Reg<TIM5_DCR_SPEC>`
pub type TIM5_DCR = crate::Reg<tim5_dcr::TIM5_DCR_SPEC>;
///TIM5 DMA control register
pub mod tim5_dcr;
///TIM5_DMAR (rw) register accessor: an alias for `Reg<TIM5_DMAR_SPEC>`
pub type TIM5_DMAR = crate::Reg<tim5_dmar::TIM5_DMAR_SPEC>;
///TIM5 DMA address for full transfer
pub mod tim5_dmar;
///TIM5_CCMR3 (rw) register accessor: an alias for `Reg<TIM5_CCMR3_SPEC>`
pub type TIM5_CCMR3 = crate::Reg<tim5_ccmr3::TIM5_CCMR3_SPEC>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim5_ccmr3;
///TIM5_CCR5 (rw) register accessor: an alias for `Reg<TIM5_CCR5_SPEC>`
pub type TIM5_CCR5 = crate::Reg<tim5_ccr5::TIM5_CCR5_SPEC>;
///TIM5 capture/compare register 5
pub mod tim5_ccr5;
///TIM5_CCR6 (rw) register accessor: an alias for `Reg<TIM5_CCR6_SPEC>`
pub type TIM5_CCR6 = crate::Reg<tim5_ccr6::TIM5_CCR6_SPEC>;
///TIM5 capture/compare register 6
pub mod tim5_ccr6;
