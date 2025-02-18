///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM6 control register 1
    pub tim6_cr1: TIM6_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM6 control register 2
    pub tim6_cr2: TIM6_CR2,
    ///0x08 - TIM6 slave mode control register
    pub tim6_smcr: TIM6_SMCR,
    ///0x0c - TIM6 DMA/interrupt enable register
    pub tim6_dier: TIM6_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM6 status register
    pub tim6_sr: TIM6_SR,
    ///0x14 - TIM6 event generation register
    pub tim6_egr: TIM6_EGR,
    _reserved6: [u8; 0x02],
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim6_ccmr1alternate6: TIM6_CCMR1ALTERNATE6,
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim6_ccmr2alternate22: TIM6_CCMR2ALTERNATE22,
    ///0x20 - TIM6 capture/compare enable register
    pub tim6_ccer: TIM6_CCER,
    ///0x24 - TIM6 counter
    pub tim6_cnt: TIM6_CNT,
    ///0x28 - TIM6 prescaler
    pub tim6_psc: TIM6_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM6 auto-reload register
    pub tim6_arr: TIM6_ARR,
    _reserved12: [u8; 0x02],
    ///0x30 - TIM6 repetition counter register
    pub tim6_rcr: TIM6_RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM6 capture/compare register 1
    pub tim6_ccr1: TIM6_CCR1,
    _reserved14: [u8; 0x02],
    ///0x38 - TIM6 capture/compare register 2
    pub tim6_ccr2: TIM6_CCR2,
    _reserved15: [u8; 0x02],
    ///0x3c - TIM6 capture/compare register 3
    pub tim6_ccr3: TIM6_CCR3,
    _reserved16: [u8; 0x02],
    ///0x40 - TIM6 capture/compare register 4
    pub tim6_ccr4: TIM6_CCR4,
    _reserved17: [u8; 0x02],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub tim6_bdtr: TIM6_BDTR,
    ///0x48 - TIM6 DMA control register
    pub tim6_dcr: TIM6_DCR,
    _reserved19: [u8; 0x02],
    ///0x4c - TIM6 DMA address for full transfer
    pub tim6_dmar: TIM6_DMAR,
    _reserved20: [u8; 0x04],
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    pub tim6_ccmr3: TIM6_CCMR3,
    ///0x58 - TIM6 capture/compare register 5
    pub tim6_ccr5: TIM6_CCR5,
    ///0x5c - TIM6 capture/compare register 6
    pub tim6_ccr6: TIM6_CCR6,
}
///TIM6_CR1 (rw) register accessor: an alias for `Reg<TIM6_CR1_SPEC>`
pub type TIM6_CR1 = crate::Reg<tim6_cr1::TIM6_CR1_SPEC>;
///TIM6 control register 1
pub mod tim6_cr1;
///TIM6_CR2 (rw) register accessor: an alias for `Reg<TIM6_CR2_SPEC>`
pub type TIM6_CR2 = crate::Reg<tim6_cr2::TIM6_CR2_SPEC>;
///TIM6 control register 2
pub mod tim6_cr2;
///TIM6_SMCR (rw) register accessor: an alias for `Reg<TIM6_SMCR_SPEC>`
pub type TIM6_SMCR = crate::Reg<tim6_smcr::TIM6_SMCR_SPEC>;
///TIM6 slave mode control register
pub mod tim6_smcr;
///TIM6_DIER (rw) register accessor: an alias for `Reg<TIM6_DIER_SPEC>`
pub type TIM6_DIER = crate::Reg<tim6_dier::TIM6_DIER_SPEC>;
///TIM6 DMA/interrupt enable register
pub mod tim6_dier;
///TIM6_SR (rw) register accessor: an alias for `Reg<TIM6_SR_SPEC>`
pub type TIM6_SR = crate::Reg<tim6_sr::TIM6_SR_SPEC>;
///TIM6 status register
pub mod tim6_sr;
///TIM6_EGR (w) register accessor: an alias for `Reg<TIM6_EGR_SPEC>`
pub type TIM6_EGR = crate::Reg<tim6_egr::TIM6_EGR_SPEC>;
///TIM6 event generation register
pub mod tim6_egr;
///TIM6_CCMR1ALTERNATE6 (rw) register accessor: an alias for `Reg<TIM6_CCMR1ALTERNATE6_SPEC>`
pub type TIM6_CCMR1ALTERNATE6 = crate::Reg<tim6_ccmr1alternate6::TIM6_CCMR1ALTERNATE6_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim6_ccmr1alternate6;
///TIM6_CCMR2ALTERNATE22 (rw) register accessor: an alias for `Reg<TIM6_CCMR2ALTERNATE22_SPEC>`
pub type TIM6_CCMR2ALTERNATE22 = crate::Reg<tim6_ccmr2alternate22::TIM6_CCMR2ALTERNATE22_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim6_ccmr2alternate22;
///TIM6_CCER (rw) register accessor: an alias for `Reg<TIM6_CCER_SPEC>`
pub type TIM6_CCER = crate::Reg<tim6_ccer::TIM6_CCER_SPEC>;
///TIM6 capture/compare enable register
pub mod tim6_ccer;
///TIM6_CNT (rw) register accessor: an alias for `Reg<TIM6_CNT_SPEC>`
pub type TIM6_CNT = crate::Reg<tim6_cnt::TIM6_CNT_SPEC>;
///TIM6 counter
pub mod tim6_cnt;
///TIM6_PSC (rw) register accessor: an alias for `Reg<TIM6_PSC_SPEC>`
pub type TIM6_PSC = crate::Reg<tim6_psc::TIM6_PSC_SPEC>;
///TIM6 prescaler
pub mod tim6_psc;
///TIM6_ARR (rw) register accessor: an alias for `Reg<TIM6_ARR_SPEC>`
pub type TIM6_ARR = crate::Reg<tim6_arr::TIM6_ARR_SPEC>;
///TIM6 auto-reload register
pub mod tim6_arr;
///TIM6_RCR (rw) register accessor: an alias for `Reg<TIM6_RCR_SPEC>`
pub type TIM6_RCR = crate::Reg<tim6_rcr::TIM6_RCR_SPEC>;
///TIM6 repetition counter register
pub mod tim6_rcr;
///TIM6_CCR1 (rw) register accessor: an alias for `Reg<TIM6_CCR1_SPEC>`
pub type TIM6_CCR1 = crate::Reg<tim6_ccr1::TIM6_CCR1_SPEC>;
///TIM6 capture/compare register 1
pub mod tim6_ccr1;
///TIM6_CCR2 (rw) register accessor: an alias for `Reg<TIM6_CCR2_SPEC>`
pub type TIM6_CCR2 = crate::Reg<tim6_ccr2::TIM6_CCR2_SPEC>;
///TIM6 capture/compare register 2
pub mod tim6_ccr2;
///TIM6_CCR3 (rw) register accessor: an alias for `Reg<TIM6_CCR3_SPEC>`
pub type TIM6_CCR3 = crate::Reg<tim6_ccr3::TIM6_CCR3_SPEC>;
///TIM6 capture/compare register 3
pub mod tim6_ccr3;
///TIM6_CCR4 (rw) register accessor: an alias for `Reg<TIM6_CCR4_SPEC>`
pub type TIM6_CCR4 = crate::Reg<tim6_ccr4::TIM6_CCR4_SPEC>;
///TIM6 capture/compare register 4
pub mod tim6_ccr4;
///TIM6_BDTR (rw) register accessor: an alias for `Reg<TIM6_BDTR_SPEC>`
pub type TIM6_BDTR = crate::Reg<tim6_bdtr::TIM6_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod tim6_bdtr;
///TIM6_DCR (rw) register accessor: an alias for `Reg<TIM6_DCR_SPEC>`
pub type TIM6_DCR = crate::Reg<tim6_dcr::TIM6_DCR_SPEC>;
///TIM6 DMA control register
pub mod tim6_dcr;
///TIM6_DMAR (rw) register accessor: an alias for `Reg<TIM6_DMAR_SPEC>`
pub type TIM6_DMAR = crate::Reg<tim6_dmar::TIM6_DMAR_SPEC>;
///TIM6 DMA address for full transfer
pub mod tim6_dmar;
///TIM6_CCMR3 (rw) register accessor: an alias for `Reg<TIM6_CCMR3_SPEC>`
pub type TIM6_CCMR3 = crate::Reg<tim6_ccmr3::TIM6_CCMR3_SPEC>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim6_ccmr3;
///TIM6_CCR5 (rw) register accessor: an alias for `Reg<TIM6_CCR5_SPEC>`
pub type TIM6_CCR5 = crate::Reg<tim6_ccr5::TIM6_CCR5_SPEC>;
///TIM6 capture/compare register 5
pub mod tim6_ccr5;
///TIM6_CCR6 (rw) register accessor: an alias for `Reg<TIM6_CCR6_SPEC>`
pub type TIM6_CCR6 = crate::Reg<tim6_ccr6::TIM6_CCR6_SPEC>;
///TIM6 capture/compare register 6
pub mod tim6_ccr6;
