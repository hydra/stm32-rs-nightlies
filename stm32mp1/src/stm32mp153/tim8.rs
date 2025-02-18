///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TIM8 control register 1
    pub tim8_cr1: TIM8_CR1,
    _reserved1: [u8; 0x02],
    ///0x04 - TIM8 control register 2
    pub tim8_cr2: TIM8_CR2,
    ///0x08 - TIM8 slave mode control register
    pub tim8_smcr: TIM8_SMCR,
    ///0x0c - TIM8 DMA/interrupt enable register
    pub tim8_dier: TIM8_DIER,
    _reserved4: [u8; 0x02],
    ///0x10 - TIM8 status register
    pub tim8_sr: TIM8_SR,
    ///0x14 - TIM8 event generation register
    pub tim8_egr: TIM8_EGR,
    _reserved6: [u8; 0x02],
    ///0x18 - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim8_ccmr1alternate8: TIM8_CCMR1ALTERNATE8,
    ///0x1c - The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
    pub tim8_ccmr2alternate24: TIM8_CCMR2ALTERNATE24,
    ///0x20 - TIM8 capture/compare enable register
    pub tim8_ccer: TIM8_CCER,
    ///0x24 - TIM8 counter
    pub tim8_cnt: TIM8_CNT,
    ///0x28 - TIM8 prescaler
    pub tim8_psc: TIM8_PSC,
    _reserved11: [u8; 0x02],
    ///0x2c - TIM8 auto-reload register
    pub tim8_arr: TIM8_ARR,
    _reserved12: [u8; 0x02],
    ///0x30 - TIM8 repetition counter register
    pub tim8_rcr: TIM8_RCR,
    _reserved13: [u8; 0x02],
    ///0x34 - TIM8 capture/compare register 1
    pub tim8_ccr1: TIM8_CCR1,
    _reserved14: [u8; 0x02],
    ///0x38 - TIM8 capture/compare register 2
    pub tim8_ccr2: TIM8_CCR2,
    _reserved15: [u8; 0x02],
    ///0x3c - TIM8 capture/compare register 3
    pub tim8_ccr3: TIM8_CCR3,
    _reserved16: [u8; 0x02],
    ///0x40 - TIM8 capture/compare register 4
    pub tim8_ccr4: TIM8_CCR4,
    _reserved17: [u8; 0x02],
    ///0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
    ///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
    pub tim8_bdtr: TIM8_BDTR,
    ///0x48 - TIM8 DMA control register
    pub tim8_dcr: TIM8_DCR,
    _reserved19: [u8; 0x02],
    ///0x4c - TIM8 DMA address for full transfer
    pub tim8_dmar: TIM8_DMAR,
    _reserved20: [u8; 0x04],
    ///0x54 - The channels 5 and 6 can only be configured in output. Output compare mode:
    pub tim8_ccmr3: TIM8_CCMR3,
    ///0x58 - TIM8 capture/compare register 5
    pub tim8_ccr5: TIM8_CCR5,
    ///0x5c - TIM8 capture/compare register 6
    pub tim8_ccr6: TIM8_CCR6,
    _reserved23: [u8; 0x02],
    ///0x60 - TIM8 Alternate function option register 1
    pub tim8_af1: TIM8_AF1,
    ///0x64 - TIM8 Alternate function option register 2
    pub tim8_af2: TIM8_AF2,
    ///0x68 - TIM8 timer input selection register
    pub tim8_tisel: TIM8_TISEL,
}
///TIM8_CR1 (rw) register accessor: an alias for `Reg<TIM8_CR1_SPEC>`
pub type TIM8_CR1 = crate::Reg<tim8_cr1::TIM8_CR1_SPEC>;
///TIM8 control register 1
pub mod tim8_cr1;
///TIM8_CR2 (rw) register accessor: an alias for `Reg<TIM8_CR2_SPEC>`
pub type TIM8_CR2 = crate::Reg<tim8_cr2::TIM8_CR2_SPEC>;
///TIM8 control register 2
pub mod tim8_cr2;
///TIM8_SMCR (rw) register accessor: an alias for `Reg<TIM8_SMCR_SPEC>`
pub type TIM8_SMCR = crate::Reg<tim8_smcr::TIM8_SMCR_SPEC>;
///TIM8 slave mode control register
pub mod tim8_smcr;
///TIM8_DIER (rw) register accessor: an alias for `Reg<TIM8_DIER_SPEC>`
pub type TIM8_DIER = crate::Reg<tim8_dier::TIM8_DIER_SPEC>;
///TIM8 DMA/interrupt enable register
pub mod tim8_dier;
///TIM8_SR (rw) register accessor: an alias for `Reg<TIM8_SR_SPEC>`
pub type TIM8_SR = crate::Reg<tim8_sr::TIM8_SR_SPEC>;
///TIM8 status register
pub mod tim8_sr;
///TIM8_EGR (w) register accessor: an alias for `Reg<TIM8_EGR_SPEC>`
pub type TIM8_EGR = crate::Reg<tim8_egr::TIM8_EGR_SPEC>;
///TIM8 event generation register
pub mod tim8_egr;
///TIM8_CCMR1ALTERNATE8 (rw) register accessor: an alias for `Reg<TIM8_CCMR1ALTERNATE8_SPEC>`
pub type TIM8_CCMR1ALTERNATE8 = crate::Reg<tim8_ccmr1alternate8::TIM8_CCMR1ALTERNATE8_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim8_ccmr1alternate8;
///TIM8_CCMR2ALTERNATE24 (rw) register accessor: an alias for `Reg<TIM8_CCMR2ALTERNATE24_SPEC>`
pub type TIM8_CCMR2ALTERNATE24 = crate::Reg<tim8_ccmr2alternate24::TIM8_CCMR2ALTERNATE24_SPEC>;
///The same register can be used for input capture mode (this section) or for output compare mode (next section). The direction of a channel is defined by configuring the corresponding CCxS bits. All the other bits of this register have a different function for input capture and for output compare modes. It is possible to combine both modes independently (e.g. channel 1 in input capture mode and channel 2 in output compare mode). Input capture mode:
pub mod tim8_ccmr2alternate24;
///TIM8_CCER (rw) register accessor: an alias for `Reg<TIM8_CCER_SPEC>`
pub type TIM8_CCER = crate::Reg<tim8_ccer::TIM8_CCER_SPEC>;
///TIM8 capture/compare enable register
pub mod tim8_ccer;
///TIM8_CNT (rw) register accessor: an alias for `Reg<TIM8_CNT_SPEC>`
pub type TIM8_CNT = crate::Reg<tim8_cnt::TIM8_CNT_SPEC>;
///TIM8 counter
pub mod tim8_cnt;
///TIM8_PSC (rw) register accessor: an alias for `Reg<TIM8_PSC_SPEC>`
pub type TIM8_PSC = crate::Reg<tim8_psc::TIM8_PSC_SPEC>;
///TIM8 prescaler
pub mod tim8_psc;
///TIM8_ARR (rw) register accessor: an alias for `Reg<TIM8_ARR_SPEC>`
pub type TIM8_ARR = crate::Reg<tim8_arr::TIM8_ARR_SPEC>;
///TIM8 auto-reload register
pub mod tim8_arr;
///TIM8_RCR (rw) register accessor: an alias for `Reg<TIM8_RCR_SPEC>`
pub type TIM8_RCR = crate::Reg<tim8_rcr::TIM8_RCR_SPEC>;
///TIM8 repetition counter register
pub mod tim8_rcr;
///TIM8_CCR1 (rw) register accessor: an alias for `Reg<TIM8_CCR1_SPEC>`
pub type TIM8_CCR1 = crate::Reg<tim8_ccr1::TIM8_CCR1_SPEC>;
///TIM8 capture/compare register 1
pub mod tim8_ccr1;
///TIM8_CCR2 (rw) register accessor: an alias for `Reg<TIM8_CCR2_SPEC>`
pub type TIM8_CCR2 = crate::Reg<tim8_ccr2::TIM8_CCR2_SPEC>;
///TIM8 capture/compare register 2
pub mod tim8_ccr2;
///TIM8_CCR3 (rw) register accessor: an alias for `Reg<TIM8_CCR3_SPEC>`
pub type TIM8_CCR3 = crate::Reg<tim8_ccr3::TIM8_CCR3_SPEC>;
///TIM8 capture/compare register 3
pub mod tim8_ccr3;
///TIM8_CCR4 (rw) register accessor: an alias for `Reg<TIM8_CCR4_SPEC>`
pub type TIM8_CCR4 = crate::Reg<tim8_ccr4::TIM8_CCR4_SPEC>;
///TIM8 capture/compare register 4
pub mod tim8_ccr4;
///TIM8_BDTR (rw) register accessor: an alias for `Reg<TIM8_BDTR_SPEC>`
pub type TIM8_BDTR = crate::Reg<tim8_bdtr::TIM8_BDTR_SPEC>;
///As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\[3:0\], BKF\[3:0\], AOE, BKP, BKE, OSSI, OSSR and DTG\[7:0\]
///can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.
pub mod tim8_bdtr;
///TIM8_DCR (rw) register accessor: an alias for `Reg<TIM8_DCR_SPEC>`
pub type TIM8_DCR = crate::Reg<tim8_dcr::TIM8_DCR_SPEC>;
///TIM8 DMA control register
pub mod tim8_dcr;
///TIM8_DMAR (rw) register accessor: an alias for `Reg<TIM8_DMAR_SPEC>`
pub type TIM8_DMAR = crate::Reg<tim8_dmar::TIM8_DMAR_SPEC>;
///TIM8 DMA address for full transfer
pub mod tim8_dmar;
///TIM8_CCMR3 (rw) register accessor: an alias for `Reg<TIM8_CCMR3_SPEC>`
pub type TIM8_CCMR3 = crate::Reg<tim8_ccmr3::TIM8_CCMR3_SPEC>;
///The channels 5 and 6 can only be configured in output. Output compare mode:
pub mod tim8_ccmr3;
///TIM8_CCR5 (rw) register accessor: an alias for `Reg<TIM8_CCR5_SPEC>`
pub type TIM8_CCR5 = crate::Reg<tim8_ccr5::TIM8_CCR5_SPEC>;
///TIM8 capture/compare register 5
pub mod tim8_ccr5;
///TIM8_CCR6 (rw) register accessor: an alias for `Reg<TIM8_CCR6_SPEC>`
pub type TIM8_CCR6 = crate::Reg<tim8_ccr6::TIM8_CCR6_SPEC>;
///TIM8 capture/compare register 6
pub mod tim8_ccr6;
///TIM8_AF1 (rw) register accessor: an alias for `Reg<TIM8_AF1_SPEC>`
pub type TIM8_AF1 = crate::Reg<tim8_af1::TIM8_AF1_SPEC>;
///TIM8 Alternate function option register 1
pub mod tim8_af1;
///TIM8_AF2 (rw) register accessor: an alias for `Reg<TIM8_AF2_SPEC>`
pub type TIM8_AF2 = crate::Reg<tim8_af2::TIM8_AF2_SPEC>;
///TIM8 Alternate function option register 2
pub mod tim8_af2;
///TIM8_TISEL (rw) register accessor: an alias for `Reg<TIM8_TISEL_SPEC>`
pub type TIM8_TISEL = crate::Reg<tim8_tisel::TIM8_TISEL_SPEC>;
///TIM8 timer input selection register
pub mod tim8_tisel;
