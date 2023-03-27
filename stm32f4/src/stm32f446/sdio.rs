///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SDIO power control register
    pub power: POWER,
    ///0x04 - CLKCR register controls the SDIO_CK output clock.
    pub clkcr: CLKCR,
    ///0x08 - The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
    pub arg: ARG,
    ///0x0c - The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
    pub cmd: CMD,
    ///0x10 - The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
    pub respcmd: RESPCMD,
    ///0x14..0x24 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    pub resp: [RESP; 4],
    ///0x24 - The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
    pub dtimer: DTIMER,
    ///0x28 - The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
    pub dlen: DLEN,
    ///0x2c - The SDMMC_DCTRL register control the data path state machine (DPSM).
    pub dctrl: DCTRL,
    ///0x30 - The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
    pub dcount: DCOUNT,
    ///0x34 - The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
    pub sta: STA,
    ///0x38 - The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
    pub icr: ICR,
    ///0x3c - The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
    pub mask: MASK,
    _reserved13: [u8; 0x08],
    ///0x48 - The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.
    pub fifocnt: FIFOCNT,
    _reserved14: [u8; 0x34],
    ///0x80 - The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
    pub fifo: FIFO,
}
impl RegisterBlock {
    ///0x14 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub fn resp1(&self) -> &RESP {
        &self.resp[0]
    }
    ///0x18 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub fn resp2(&self) -> &RESP {
        &self.resp[1]
    }
    ///0x1c - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub fn resp3(&self) -> &RESP {
        &self.resp[2]
    }
    ///0x20 - The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
    #[inline(always)]
    pub fn resp4(&self) -> &RESP {
        &self.resp[3]
    }
}
///POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`
pub type POWER = crate::Reg<power::POWER_SPEC>;
///SDIO power control register
pub mod power;
///CLKCR (rw) register accessor: an alias for `Reg<CLKCR_SPEC>`
pub type CLKCR = crate::Reg<clkcr::CLKCR_SPEC>;
///CLKCR register controls the SDIO_CK output clock.
pub mod clkcr;
///ARG (rw) register accessor: an alias for `Reg<ARG_SPEC>`
pub type ARG = crate::Reg<arg::ARG_SPEC>;
///The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.
pub mod arg;
///CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///The SDMMC_CMDR register contains the command index and command type bits. The command index is sent to a card as part of a command message. The command type bits control the command path state machine (CPSM).
pub mod cmd;
///RESP (r) register accessor: an alias for `Reg<RESP_SPEC>`
pub type RESP = crate::Reg<resp::RESP_SPEC>;
///The SDMMC_RESP1/2/3/4R registers contain the status of a card, which is part of the received response.
pub mod resp;
///DTIMER (rw) register accessor: an alias for `Reg<DTIMER_SPEC>`
pub type DTIMER = crate::Reg<dtimer::DTIMER_SPEC>;
///The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.
pub mod dtimer;
///DLEN (rw) register accessor: an alias for `Reg<DLEN_SPEC>`
pub type DLEN = crate::Reg<dlen::DLEN_SPEC>;
///The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.
pub mod dlen;
///DCTRL (rw) register accessor: an alias for `Reg<DCTRL_SPEC>`
pub type DCTRL = crate::Reg<dctrl::DCTRL_SPEC>;
///The SDMMC_DCTRL register control the data path state machine (DPSM).
pub mod dctrl;
///DCOUNT (r) register accessor: an alias for `Reg<DCOUNT_SPEC>`
pub type DCOUNT = crate::Reg<dcount::DCOUNT_SPEC>;
///The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.
pub mod dcount;
///STA (r) register accessor: an alias for `Reg<STA_SPEC>`
pub type STA = crate::Reg<sta::STA_SPEC>;
///The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)
pub mod sta;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
pub mod icr;
///MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`
pub type MASK = crate::Reg<mask::MASK_SPEC>;
///The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
pub mod mask;
///RESPCMD (r) register accessor: an alias for `Reg<RESPCMD_SPEC>`
pub type RESPCMD = crate::Reg<respcmd::RESPCMD_SPEC>;
///The SDMMC_RESPCMDR register contains the command index field of the last command response received. If the command response transmission does not contain the command index field (long or OCR response), the RESPCMD field is unknown, although it must contain 111111b (the value of the reserved field from the response).
pub mod respcmd;
///FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
///The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO.When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.
pub mod fifo;
///FIFOCNT (r) register accessor: an alias for `Reg<FIFOCNT_SPEC>`
pub type FIFOCNT = crate::Reg<fifocnt::FIFOCNT_SPEC>;
///The SDIO_FIFOCNT register contains the remaining number of words to be written to or read from the FIFO. The FIFO counter loads the value from the data length register (see SDIO_DLEN) when the data transfer enable bit, DTEN, is set in the data control register (SDIO_DCTRL register) and the DPSM is at the Idle state. If the data length is not word-aligned (multiple of 4), the remaining 1 to 3 bytes are regarded as a word.
pub mod fifocnt;
