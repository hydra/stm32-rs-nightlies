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
    ///0x0c - TIM2 DMA/interrupt enable register
    pub tim2_dier: TIM2_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM2 status register
    pub tim2_sr: TIM2_SR,
    ///0x14 - TIM2 event generation register
    pub tim2_egr: TIM2_EGR,
    _reserved6: [u8; 0x02],
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim2_ccmr1alternate2: TIM2_CCMR1ALTERNATE2,
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim2_ccmr2alternate18: TIM2_CCMR2ALTERNATE18,
    ///0x20 - TIM2 capture/compare enable register
    pub tim2_ccer: TIM2_CCER,
    ///0x24 - TIM2 counter
    pub tim2_cnt: TIM2_CNT,
    ///0x28 - TIM2 prescaler
    pub tim2_psc: TIM2_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM2 auto-reload register
    pub tim2_arr: TIM2_ARR,
    _reserved12: [u8; 0x02],
    ///0x30 - TIM2 repetition counter register
    pub tim2_rcr: TIM2_RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM2 capture/compare register 1
    pub tim2_ccr1: TIM2_CCR1,
    _reserved14: [u8; 0x02],
    ///0x38 - TIM2 capture/compare register 2
    pub tim2_ccr2: TIM2_CCR2,
    _reserved15: [u8; 0x02],
    ///0x3c - TIM2 capture/compare register 3
    pub tim2_ccr3: TIM2_CCR3,
    _reserved16: [u8; 0x02],
    ///0x40 - TIM2 capture/compare register 4
    pub tim2_ccr4: TIM2_CCR4,
    _reserved17: [u8; 0x02],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub tim2_bdtr: TIM2_BDTR,
    ///0x48 - TIM2 DMA control register
    pub tim2_dcr: TIM2_DCR,
    _reserved19: [u8; 0x02],
    ///0x4c - TIM2 DMA address for full transfer
    pub tim2_dmar: TIM2_DMAR,
    _reserved20: [u8; 0x04],
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    pub tim2_ccmr3: TIM2_CCMR3,
    ///0x58 - TIM2 capture/compare register 5
    pub tim2_ccr5: TIM2_CCR5,
    ///0x5c - TIM2 capture/compare register 6
    pub tim2_ccr6: TIM2_CCR6,
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
///TIM2 DMA/interrupt enable register
pub mod tim2_dier;
///TIM2_SR (rw) register accessor: an alias for `Reg<TIM2_SR_SPEC>`
pub type TIM2_SR = crate::Reg<tim2_sr::TIM2_SR_SPEC>;
///TIM2 status register
pub mod tim2_sr;
///TIM2_EGR (w) register accessor: an alias for `Reg<TIM2_EGR_SPEC>`
pub type TIM2_EGR = crate::Reg<tim2_egr::TIM2_EGR_SPEC>;
///TIM2 event generation register
pub mod tim2_egr;
///TIM2_CCMR1ALTERNATE2 (rw) register accessor: an alias for `Reg<TIM2_CCMR1ALTERNATE2_SPEC>`
pub type TIM2_CCMR1ALTERNATE2 = crate::Reg<tim2_ccmr1alternate2::TIM2_CCMR1ALTERNATE2_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim2_ccmr1alternate2;
///TIM2_CCMR2ALTERNATE18 (rw) register accessor: an alias for `Reg<TIM2_CCMR2ALTERNATE18_SPEC>`
pub type TIM2_CCMR2ALTERNATE18 = crate::Reg<tim2_ccmr2alternate18::TIM2_CCMR2ALTERNATE18_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim2_ccmr2alternate18;
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
///TIM2_RCR (rw) register accessor: an alias for `Reg<TIM2_RCR_SPEC>`
pub type TIM2_RCR = crate::Reg<tim2_rcr::TIM2_RCR_SPEC>;
///TIM2 repetition counter register
pub mod tim2_rcr;
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
///TIM2_BDTR (rw) register accessor: an alias for `Reg<TIM2_BDTR_SPEC>`
pub type TIM2_BDTR = crate::Reg<tim2_bdtr::TIM2_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod tim2_bdtr;
///TIM2_DCR (rw) register accessor: an alias for `Reg<TIM2_DCR_SPEC>`
pub type TIM2_DCR = crate::Reg<tim2_dcr::TIM2_DCR_SPEC>;
///TIM2 DMA control register
pub mod tim2_dcr;
///TIM2_DMAR (rw) register accessor: an alias for `Reg<TIM2_DMAR_SPEC>`
pub type TIM2_DMAR = crate::Reg<tim2_dmar::TIM2_DMAR_SPEC>;
///TIM2 DMA address for full transfer
pub mod tim2_dmar;
///TIM2_CCMR3 (rw) register accessor: an alias for `Reg<TIM2_CCMR3_SPEC>`
pub type TIM2_CCMR3 = crate::Reg<tim2_ccmr3::TIM2_CCMR3_SPEC>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim2_ccmr3;
///TIM2_CCR5 (rw) register accessor: an alias for `Reg<TIM2_CCR5_SPEC>`
pub type TIM2_CCR5 = crate::Reg<tim2_ccr5::TIM2_CCR5_SPEC>;
///TIM2 capture/compare register 5
pub mod tim2_ccr5;
///TIM2_CCR6 (rw) register accessor: an alias for `Reg<TIM2_CCR6_SPEC>`
pub type TIM2_CCR6 = crate::Reg<tim2_ccr6::TIM2_CCR6_SPEC>;
///TIM2 capture/compare register 6
pub mod tim2_ccr6;
