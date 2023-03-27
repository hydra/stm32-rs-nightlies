///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CEC control register
    pub cec_cr: CEC_CR,
    ///0x04 - This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0.
    pub cec_cfgr: CEC_CFGR,
    ///0x08 - CEC Tx data register
    pub cec_txdr: CEC_TXDR,
    ///0x0c - CEC Rx data register
    pub cec_rxdr: CEC_RXDR,
    ///0x10 - CEC Interrupt and Status Register
    pub cec_isr: CEC_ISR,
    ///0x14 - CEC interrupt enable register
    pub cec_ier: CEC_IER,
}
///CEC_CR (rw) register accessor: an alias for `Reg<CEC_CR_SPEC>`
pub type CEC_CR = crate::Reg<cec_cr::CEC_CR_SPEC>;
///CEC control register
pub mod cec_cr;
///CEC_CFGR (rw) register accessor: an alias for `Reg<CEC_CFGR_SPEC>`
pub type CEC_CFGR = crate::Reg<cec_cfgr::CEC_CFGR_SPEC>;
///This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0.
pub mod cec_cfgr;
///CEC_TXDR (rw) register accessor: an alias for `Reg<CEC_TXDR_SPEC>`
pub type CEC_TXDR = crate::Reg<cec_txdr::CEC_TXDR_SPEC>;
///CEC Tx data register
pub mod cec_txdr;
///CEC_RXDR (r) register accessor: an alias for `Reg<CEC_RXDR_SPEC>`
pub type CEC_RXDR = crate::Reg<cec_rxdr::CEC_RXDR_SPEC>;
///CEC Rx data register
pub mod cec_rxdr;
///CEC_ISR (rw) register accessor: an alias for `Reg<CEC_ISR_SPEC>`
pub type CEC_ISR = crate::Reg<cec_isr::CEC_ISR_SPEC>;
///CEC Interrupt and Status Register
pub mod cec_isr;
///CEC_IER (rw) register accessor: an alias for `Reg<CEC_IER_SPEC>`
pub type CEC_IER = crate::Reg<cec_ier::CEC_IER_SPEC>;
///CEC interrupt enable register
pub mod cec_ier;
