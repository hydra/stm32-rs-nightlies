///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - power control register
    pub power: POWER,
    ///0x04 - SDI clock control register
    pub clkcr: CLKCR,
    ///0x08 - argument register
    pub arg: ARG,
    ///0x0c - command register
    pub cmd: CMD,
    ///0x10 - command response register
    pub respcmd: RESPCMD,
    ///0x14 - response 1..4 register
    pub resp1: RESP1,
    ///0x18 - response 1..4 register
    pub resp2: RESP2,
    ///0x1c - response 1..4 register
    pub resp3: RESP3,
    ///0x20 - response 1..4 register
    pub resp4: RESP4,
    ///0x24 - data timer register
    pub dtimer: DTIMER,
    ///0x28 - data length register
    pub dlen: DLEN,
    ///0x2c - data control register
    pub dctrl: DCTRL,
    ///0x30 - data counter register
    pub dcount: DCOUNT,
    ///0x34 - status register
    pub sta: STA,
    ///0x38 - interrupt clear register
    pub icr: ICR,
    ///0x3c - mask register
    pub mask: MASK,
    _reserved16: [u8; 0x08],
    ///0x48 - FIFO counter register
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 0x34],
    ///0x80 - data FIFO register
    pub fifo: FIFO,
}
///POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`
pub type POWER = crate::Reg<power::POWER_SPEC>;
///power control register
pub mod power;
///CLKCR (rw) register accessor: an alias for `Reg<CLKCR_SPEC>`
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
///SDI clock control register
pub mod clkcr;
///ARG (rw) register accessor: an alias for `Reg<ARG_SPEC>`
pub type ARG = crate::Reg<arg::ARG_SPEC>;
///argument register
pub mod arg;
///CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///command register
pub mod cmd;
///RESPCMD (r) register accessor: an alias for `Reg<RESPCMD_SPEC>`
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
///command response register
pub mod respcmd;
///RESP1 (r) register accessor: an alias for `Reg<RESP1_SPEC>`
pub type RESP1 = crate::Reg<resp1::RESP1_SPEC>;
///response 1..4 register
pub mod resp1;
///RESP2 (r) register accessor: an alias for `Reg<RESP2_SPEC>`
pub type RESP2 = crate::Reg<resp2::RESP2_SPEC>;
///response 1..4 register
pub mod resp2;
///RESP3 (r) register accessor: an alias for `Reg<RESP3_SPEC>`
pub type RESP3 = crate::Reg<resp3::RESP3_SPEC>;
///response 1..4 register
pub mod resp3;
///RESP4 (r) register accessor: an alias for `Reg<RESP4_SPEC>`
pub type RESP4 = crate::Reg<resp4::RESP4_SPEC>;
///response 1..4 register
pub mod resp4;
///DTIMER (rw) register accessor: an alias for `Reg<DTIMER_SPEC>`
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
///data timer register
pub mod dtimer;
///DLEN (rw) register accessor: an alias for `Reg<DLEN_SPEC>`
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
///data length register
pub mod dlen;
///DCTRL (rw) register accessor: an alias for `Reg<DCTRL_SPEC>`
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
///data control register
pub mod dctrl;
///DCOUNT (r) register accessor: an alias for `Reg<DCOUNT_SPEC>`
pub type DCOUNT = crate::Reg<dcount::DCOUNT_SPEC>;
///data counter register
pub mod dcount;
///STA (r) register accessor: an alias for `Reg<STA_SPEC>`
pub type STA = crate::Reg<sta::STA_SPEC>;
///status register
pub mod sta;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt clear register
pub mod icr;
///MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`
pub type MASK = crate::Reg<mask::MASK_SPEC>;
///mask register
pub mod mask;
///FIFOCNT (r) register accessor: an alias for `Reg<FIFOCNT_SPEC>`
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
///FIFO counter register
pub mod fifocnt;
///FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
///data FIFO register
pub mod fifo;
