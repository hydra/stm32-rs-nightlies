///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Bits 1:0 = PWRCTRL: Power supply control bits
    pub power: POWER,
    ///0x04 - SDI clock control register (SDIO_CLKCR)
    pub clkcr: CLKCR,
    ///0x08 - Bits 31:0 = : Command argument
    pub arg: ARG,
    ///0x0c - SDIO command register (SDIO_CMD)
    pub cmd: CMD,
    ///0x10 - SDIO command register
    pub respcmd: RESPCMD,
    ///0x14 - Bits 31:0 = CARDSTATUS1
    pub respi1: RESPI1,
    ///0x18 - Bits 31:0 = CARDSTATUS2
    pub resp2: RESP2,
    ///0x1c - Bits 31:0 = CARDSTATUS3
    pub resp3: RESP3,
    ///0x20 - Bits 31:0 = CARDSTATUS4
    pub resp4: RESP4,
    ///0x24 - Bits 31:0 = DATATIME: Data timeout period
    pub dtimer: DTIMER,
    ///0x28 - Bits 24:0 = DATALENGTH: Data length value
    pub dlen: DLEN,
    ///0x2c - SDIO data control register (SDIO_DCTRL)
    pub dctrl: DCTRL,
    ///0x30 - Bits 24:0 = DATACOUNT: Data count value
    pub dcount: DCOUNT,
    ///0x34 - SDIO status register (SDIO_STA)
    pub sta: STA,
    ///0x38 - SDIO interrupt clear register (SDIO_ICR)
    pub icr: ICR,
    ///0x3c - SDIO mask register (SDIO_MASK)
    pub mask: MASK,
    _reserved16: [u8; 0x08],
    ///0x48 - Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 0x34],
    ///0x80 - bits 31:0 = FIFOData: Receive and transmit FIFO data
    pub fifo: FIFO,
}
///POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`
pub type POWER = crate::Reg<power::POWER_SPEC>;
///Bits 1:0 = PWRCTRL: Power supply control bits
pub mod power;
///CLKCR (rw) register accessor: an alias for `Reg<CLKCR_SPEC>`
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
///SDI clock control register (SDIO_CLKCR)
pub mod clkcr;
///ARG (rw) register accessor: an alias for `Reg<ARG_SPEC>`
pub type ARG = crate::Reg<arg::ARG_SPEC>;
///Bits 31:0 = : Command argument
pub mod arg;
///CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///SDIO command register (SDIO_CMD)
pub mod cmd;
///RESPCMD (r) register accessor: an alias for `Reg<RESPCMD_SPEC>`
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
///SDIO command register
pub mod respcmd;
///RESPI1 (r) register accessor: an alias for `Reg<RESPI1_SPEC>`
pub type RESPI1 = crate::Reg<respi1::RESPI1_SPEC>;
///Bits 31:0 = CARDSTATUS1
pub mod respi1;
///RESP2 (r) register accessor: an alias for `Reg<RESP2_SPEC>`
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
///Bits 31:0 = CARDSTATUS2
pub mod resp2;
///RESP3 (r) register accessor: an alias for `Reg<RESP3_SPEC>`
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
///Bits 31:0 = CARDSTATUS3
pub mod resp3;
///RESP4 (r) register accessor: an alias for `Reg<RESP4_SPEC>`
pub type RESP4 = crate::Reg<resp4::RESP4_SPEC>;
///Bits 31:0 = CARDSTATUS4
pub mod resp4;
///DTIMER (rw) register accessor: an alias for `Reg<DTIMER_SPEC>`
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
///Bits 31:0 = DATATIME: Data timeout period
pub mod dtimer;
///DLEN (rw) register accessor: an alias for `Reg<DLEN_SPEC>`
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
///Bits 24:0 = DATALENGTH: Data length value
pub mod dlen;
///DCTRL (rw) register accessor: an alias for `Reg<DCTRL_SPEC>`
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
///SDIO data control register (SDIO_DCTRL)
pub mod dctrl;
///DCOUNT (r) register accessor: an alias for `Reg<DCOUNT_SPEC>`
pub type DCOUNT = crate::Reg<dcount::DCOUNT_SPEC>;
///Bits 24:0 = DATACOUNT: Data count value
pub mod dcount;
///STA (r) register accessor: an alias for `Reg<STA_SPEC>`
pub type STA = crate::Reg<sta::STA_SPEC>;
///SDIO status register (SDIO_STA)
pub mod sta;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///SDIO interrupt clear register (SDIO_ICR)
pub mod icr;
///MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`
pub type MASK = crate::Reg<mask::MASK_SPEC>;
///SDIO mask register (SDIO_MASK)
pub mod mask;
///FIFOCNT (r) register accessor: an alias for `Reg<FIFOCNT_SPEC>`
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
///Bits 23:0 = FIFOCOUNT: Remaining number of words to be written to or read from the FIFO
pub mod fifocnt;
///FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
///bits 31:0 = FIFOData: Receive and transmit FIFO data
pub mod fifo;
