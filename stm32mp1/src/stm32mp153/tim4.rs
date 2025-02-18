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
    ///0x0c - TIM4 DMA/interrupt enable register
    pub tim4_dier: TIM4_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM4 status register
    pub tim4_sr: TIM4_SR,
    ///0x14 - TIM4 event generation register
    pub tim4_egr: TIM4_EGR,
    _reserved6: [u8; 0x02],
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim4_ccmr1alternate4: TIM4_CCMR1ALTERNATE4,
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim4_ccmr2alternate20: TIM4_CCMR2ALTERNATE20,
    ///0x20 - TIM4 capture/compare enable register
    pub tim4_ccer: TIM4_CCER,
    ///0x24 - TIM4 counter
    pub tim4_cnt: TIM4_CNT,
    ///0x28 - TIM4 prescaler
    pub tim4_psc: TIM4_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM4 auto-reload register
    pub tim4_arr: TIM4_ARR,
    _reserved12: [u8; 0x02],
    ///0x30 - TIM4 repetition counter register
    pub tim4_rcr: TIM4_RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM4 capture/compare register 1
    pub tim4_ccr1: TIM4_CCR1,
    _reserved14: [u8; 0x02],
    ///0x38 - TIM4 capture/compare register 2
    pub tim4_ccr2: TIM4_CCR2,
    _reserved15: [u8; 0x02],
    ///0x3c - TIM4 capture/compare register 3
    pub tim4_ccr3: TIM4_CCR3,
    _reserved16: [u8; 0x02],
    ///0x40 - TIM4 capture/compare register 4
    pub tim4_ccr4: TIM4_CCR4,
    _reserved17: [u8; 0x02],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub tim4_bdtr: TIM4_BDTR,
    ///0x48 - TIM4 DMA control register
    pub tim4_dcr: TIM4_DCR,
    _reserved19: [u8; 0x02],
    ///0x4c - TIM4 DMA address for full transfer
    pub tim4_dmar: TIM4_DMAR,
    _reserved20: [u8; 0x04],
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    pub tim4_ccmr3: TIM4_CCMR3,
    ///0x58 - TIM4 capture/compare register 5
    pub tim4_ccr5: TIM4_CCR5,
    ///0x5c - TIM4 capture/compare register 6
    pub tim4_ccr6: TIM4_CCR6,
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
///TIM4 DMA/interrupt enable register
pub mod tim4_dier;
///TIM4_SR (rw) register accessor: an alias for `Reg<TIM4_SR_SPEC>`
pub type TIM4_SR = crate::Reg<tim4_sr::TIM4_SR_SPEC>;
///TIM4 status register
pub mod tim4_sr;
///TIM4_EGR (w) register accessor: an alias for `Reg<TIM4_EGR_SPEC>`
pub type TIM4_EGR = crate::Reg<tim4_egr::TIM4_EGR_SPEC>;
///TIM4 event generation register
pub mod tim4_egr;
///TIM4_CCMR1ALTERNATE4 (rw) register accessor: an alias for `Reg<TIM4_CCMR1ALTERNATE4_SPEC>`
pub type TIM4_CCMR1ALTERNATE4 = crate::Reg<tim4_ccmr1alternate4::TIM4_CCMR1ALTERNATE4_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim4_ccmr1alternate4;
///TIM4_CCMR2ALTERNATE20 (rw) register accessor: an alias for `Reg<TIM4_CCMR2ALTERNATE20_SPEC>`
pub type TIM4_CCMR2ALTERNATE20 = crate::Reg<tim4_ccmr2alternate20::TIM4_CCMR2ALTERNATE20_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim4_ccmr2alternate20;
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
///TIM4_RCR (rw) register accessor: an alias for `Reg<TIM4_RCR_SPEC>`
pub type TIM4_RCR = crate::Reg<tim4_rcr::TIM4_RCR_SPEC>;
///TIM4 repetition counter register
pub mod tim4_rcr;
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
///TIM4_BDTR (rw) register accessor: an alias for `Reg<TIM4_BDTR_SPEC>`
pub type TIM4_BDTR = crate::Reg<tim4_bdtr::TIM4_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod tim4_bdtr;
///TIM4_DCR (rw) register accessor: an alias for `Reg<TIM4_DCR_SPEC>`
pub type TIM4_DCR = crate::Reg<tim4_dcr::TIM4_DCR_SPEC>;
///TIM4 DMA control register
pub mod tim4_dcr;
///TIM4_DMAR (rw) register accessor: an alias for `Reg<TIM4_DMAR_SPEC>`
pub type TIM4_DMAR = crate::Reg<tim4_dmar::TIM4_DMAR_SPEC>;
///TIM4 DMA address for full transfer
pub mod tim4_dmar;
///TIM4_CCMR3 (rw) register accessor: an alias for `Reg<TIM4_CCMR3_SPEC>`
pub type TIM4_CCMR3 = crate::Reg<tim4_ccmr3::TIM4_CCMR3_SPEC>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim4_ccmr3;
///TIM4_CCR5 (rw) register accessor: an alias for `Reg<TIM4_CCR5_SPEC>`
pub type TIM4_CCR5 = crate::Reg<tim4_ccr5::TIM4_CCR5_SPEC>;
///TIM4 capture/compare register 5
pub mod tim4_ccr5;
///TIM4_CCR6 (rw) register accessor: an alias for `Reg<TIM4_CCR6_SPEC>`
pub type TIM4_CCR6 = crate::Reg<tim4_ccr6::TIM4_CCR6_SPEC>;
///TIM4 capture/compare register 6
pub mod tim4_ccr6;
