///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - power control register
    pub power: POWER,
    ///0x04 - clock control register
    pub clkcr: CLKCR,
    ///0x08 - argument register
    pub argr: ARGR,
    ///0x0c - command register
    pub cmdr: CMDR,
    ///0x10 - command response register
    pub respcmdr: RESPCMDR,
    ///0x14 - response 1 register
    pub resp1: RESP1,
    ///0x18 - response 2 register
    pub resp2: RESP2,
    ///0x1c - response 3 register
    pub resp3: RESP3,
    ///0x20 - response 4 register
    pub resp4: RESP4,
    ///0x24 - data timer register
    pub dtimer: DTIMER,
    ///0x28 - data length register
    pub dlenr: DLENR,
    ///0x2c - data control register
    pub dctrl: DCTRL,
    ///0x30 - data counter register
    pub dcntr: DCNTR,
    ///0x34 - status register
    pub star: STAR,
    ///0x38 - interrupt clear register
    pub icr: ICR,
    ///0x3c - mask register
    pub maskr: MASKR,
    ///0x40 - acknowledgment timer register
    pub acktimer: ACKTIMER,
    _reserved17: [u8; 0x0c],
    ///0x50 - DMA control register
    pub sdmmc_idmactrlr: SDMMC_IDMACTRLR,
    ///0x54 - buffer size register
    pub sdmmc_idmabsizer: SDMMC_IDMABSIZER,
    ///0x58 - buffer base address register
    pub sdmmc_idmabaser: SDMMC_IDMABASER,
    _reserved20: [u8; 0x08],
    ///0x64 - linked list address register
    pub sdmmc_idmalar: SDMMC_IDMALAR,
    ///0x68 - linked list memory base register
    pub sdmmc_idmabar: SDMMC_IDMABAR,
    _reserved22: [u8; 0x14],
    ///0x80 - data FIFO register 0
    pub fifor0: FIFOR0,
    ///0x84 - data FIFO register 1
    pub fifor1: FIFOR1,
    ///0x88 - data FIFO register 2
    pub fifor2: FIFOR2,
    ///0x8c - data FIFO register 3
    pub fifor3: FIFOR3,
    ///0x90 - data FIFO register 4
    pub fifor4: FIFOR4,
    ///0x94 - data FIFO register 5
    pub fifor5: FIFOR5,
    ///0x98 - data FIFO register 6
    pub fifor6: FIFOR6,
    ///0x9c - data FIFO register 7
    pub fifor7: FIFOR7,
    ///0xa0 - data FIFO register 8
    pub fifor8: FIFOR8,
    ///0xa4 - data FIFO register 9
    pub fifor9: FIFOR9,
    ///0xa8 - data FIFO register 10
    pub fifor10: FIFOR10,
    ///0xac - data FIFO register 11
    pub fifor11: FIFOR11,
    ///0xb0 - data FIFO register 12
    pub fifor12: FIFOR12,
    ///0xb4 - data FIFO register 13
    pub fifor13: FIFOR13,
    ///0xb8 - data FIFO register 14
    pub fifor14: FIFOR14,
    ///0xbc - data FIFO register 15
    pub fifor15: FIFOR15,
}
///POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`
pub type POWER = crate::Reg<power::POWER_SPEC>;
///power control register
pub mod power;
///CLKCR (rw) register accessor: an alias for `Reg<CLKCR_SPEC>`
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
///clock control register
pub mod clkcr;
///ARGR (rw) register accessor: an alias for `Reg<ARGR_SPEC>`
pub type ARGR = crate::Reg<argr::ARGR_SPEC>;
///argument register
pub mod argr;
///CMDR (rw) register accessor: an alias for `Reg<CMDR_SPEC>`
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
///command register
pub mod cmdr;
///RESPCMDR (r) register accessor: an alias for `Reg<RESPCMDR_SPEC>`
pub type RESPCMDR = crate::Reg<respcmdr::RESPCMDR_SPEC>;
///command response register
pub mod respcmdr;
///RESP1 (r) register accessor: an alias for `Reg<RESP1_SPEC>`
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
///response 1 register
pub mod resp1;
///RESP2 (r) register accessor: an alias for `Reg<RESP2_SPEC>`
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
///response 2 register
pub mod resp2;
///RESP3 (r) register accessor: an alias for `Reg<RESP3_SPEC>`
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
///response 3 register
pub mod resp3;
///RESP4 (r) register accessor: an alias for `Reg<RESP4_SPEC>`
pub type RESP4 = crate::Reg<resp4::RESP4_SPEC>;
///response 4 register
pub mod resp4;
///DTIMER (rw) register accessor: an alias for `Reg<DTIMER_SPEC>`
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
///data timer register
pub mod dtimer;
///DLENR (rw) register accessor: an alias for `Reg<DLENR_SPEC>`
pub type DLENR = crate::Reg<dlenr::DLENR_SPEC>;
///data length register
pub mod dlenr;
///DCTRL (rw) register accessor: an alias for `Reg<DCTRL_SPEC>`
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
///data control register
pub mod dctrl;
///DCNTR (r) register accessor: an alias for `Reg<DCNTR_SPEC>`
pub type DCNTR = crate::Reg<dcntr::DCNTR_SPEC>;
///data counter register
pub mod dcntr;
///STAR (r) register accessor: an alias for `Reg<STAR_SPEC>`
pub type STAR = crate::Reg<star::STAR_SPEC>;
///status register
pub mod star;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt clear register
pub mod icr;
///MASKR (rw) register accessor: an alias for `Reg<MASKR_SPEC>`
pub type MASKR = crate::Reg<maskr::MASKR_SPEC>;
///mask register
pub mod maskr;
///ACKTIMER (rw) register accessor: an alias for `Reg<ACKTIMER_SPEC>`
pub type ACKTIMER = crate::Reg<acktimer::ACKTIMER_SPEC>;
///acknowledgment timer register
pub mod acktimer;
///FIFOR0 (rw) register accessor: an alias for `Reg<FIFOR0_SPEC>`
pub type FIFOR0 = crate::Reg<fifor0::FIFOR0_SPEC>;
///data FIFO register 0
pub mod fifor0;
///FIFOR1 (rw) register accessor: an alias for `Reg<FIFOR1_SPEC>`
pub type FIFOR1 = crate::Reg<fifor1::FIFOR1_SPEC>;
///data FIFO register 1
pub mod fifor1;
///FIFOR2 (rw) register accessor: an alias for `Reg<FIFOR2_SPEC>`
pub type FIFOR2 = crate::Reg<fifor2::FIFOR2_SPEC>;
///data FIFO register 2
pub mod fifor2;
///FIFOR3 (rw) register accessor: an alias for `Reg<FIFOR3_SPEC>`
pub type FIFOR3 = crate::Reg<fifor3::FIFOR3_SPEC>;
///data FIFO register 3
pub mod fifor3;
///FIFOR4 (rw) register accessor: an alias for `Reg<FIFOR4_SPEC>`
pub type FIFOR4 = crate::Reg<fifor4::FIFOR4_SPEC>;
///data FIFO register 4
pub mod fifor4;
///FIFOR5 (rw) register accessor: an alias for `Reg<FIFOR5_SPEC>`
pub type FIFOR5 = crate::Reg<fifor5::FIFOR5_SPEC>;
///data FIFO register 5
pub mod fifor5;
///FIFOR6 (rw) register accessor: an alias for `Reg<FIFOR6_SPEC>`
pub type FIFOR6 = crate::Reg<fifor6::FIFOR6_SPEC>;
///data FIFO register 6
pub mod fifor6;
///FIFOR7 (rw) register accessor: an alias for `Reg<FIFOR7_SPEC>`
pub type FIFOR7 = crate::Reg<fifor7::FIFOR7_SPEC>;
///data FIFO register 7
pub mod fifor7;
///FIFOR8 (rw) register accessor: an alias for `Reg<FIFOR8_SPEC>`
pub type FIFOR8 = crate::Reg<fifor8::FIFOR8_SPEC>;
///data FIFO register 8
pub mod fifor8;
///FIFOR9 (rw) register accessor: an alias for `Reg<FIFOR9_SPEC>`
pub type FIFOR9 = crate::Reg<fifor9::FIFOR9_SPEC>;
///data FIFO register 9
pub mod fifor9;
///FIFOR10 (rw) register accessor: an alias for `Reg<FIFOR10_SPEC>`
pub type FIFOR10 = crate::Reg<fifor10::FIFOR10_SPEC>;
///data FIFO register 10
pub mod fifor10;
///FIFOR11 (rw) register accessor: an alias for `Reg<FIFOR11_SPEC>`
pub type FIFOR11 = crate::Reg<fifor11::FIFOR11_SPEC>;
///data FIFO register 11
pub mod fifor11;
///FIFOR12 (rw) register accessor: an alias for `Reg<FIFOR12_SPEC>`
pub type FIFOR12 = crate::Reg<fifor12::FIFOR12_SPEC>;
///data FIFO register 12
pub mod fifor12;
///FIFOR13 (rw) register accessor: an alias for `Reg<FIFOR13_SPEC>`
pub type FIFOR13 = crate::Reg<fifor13::FIFOR13_SPEC>;
///data FIFO register 13
pub mod fifor13;
///FIFOR14 (rw) register accessor: an alias for `Reg<FIFOR14_SPEC>`
pub type FIFOR14 = crate::Reg<fifor14::FIFOR14_SPEC>;
///data FIFO register 14
pub mod fifor14;
///FIFOR15 (rw) register accessor: an alias for `Reg<FIFOR15_SPEC>`
pub type FIFOR15 = crate::Reg<fifor15::FIFOR15_SPEC>;
///data FIFO register 15
pub mod fifor15;
///SDMMC_IDMACTRLR (rw) register accessor: an alias for `Reg<SDMMC_IDMACTRLR_SPEC>`
pub type SDMMC_IDMACTRLR = crate::Reg<sdmmc_idmactrlr::SDMMC_IDMACTRLR_SPEC>;
///DMA control register
pub mod sdmmc_idmactrlr;
///SDMMC_IDMABSIZER (rw) register accessor: an alias for `Reg<SDMMC_IDMABSIZER_SPEC>`
pub type SDMMC_IDMABSIZER = crate::Reg<sdmmc_idmabsizer::SDMMC_IDMABSIZER_SPEC>;
///buffer size register
pub mod sdmmc_idmabsizer;
///SDMMC_IDMABASER (rw) register accessor: an alias for `Reg<SDMMC_IDMABASER_SPEC>`
pub type SDMMC_IDMABASER = crate::Reg<sdmmc_idmabaser::SDMMC_IDMABASER_SPEC>;
///buffer base address register
pub mod sdmmc_idmabaser;
///SDMMC_IDMALAR (rw) register accessor: an alias for `Reg<SDMMC_IDMALAR_SPEC>`
pub type SDMMC_IDMALAR = crate::Reg<sdmmc_idmalar::SDMMC_IDMALAR_SPEC>;
///linked list address register
pub mod sdmmc_idmalar;
///SDMMC_IDMABAR (rw) register accessor: an alias for `Reg<SDMMC_IDMABAR_SPEC>`
pub type SDMMC_IDMABAR = crate::Reg<sdmmc_idmabar::SDMMC_IDMABAR_SPEC>;
///linked list memory base register
pub mod sdmmc_idmabar;
