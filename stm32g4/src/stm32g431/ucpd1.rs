///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - UCPD configuration register 1
    pub cfg1: CFG1,
    ///0x04 - UCPD configuration register 2
    pub cfg2: CFG2,
    _reserved2: [u8; 0x04],
    ///0x0c - UCPD configuration register 2
    pub cr: CR,
    ///0x10 - UCPD Interrupt Mask Register
    pub imr: IMR,
    ///0x14 - UCPD Status Register
    pub sr: SR,
    ///0x18 - UCPD Interrupt Clear Register
    pub icr: ICR,
    ///0x1c - UCPD Tx Ordered Set Type Register
    pub tx_ordset: TX_ORDSET,
    ///0x20 - UCPD Tx Paysize Register
    pub tx_paysz: TX_PAYSZ,
    ///0x24 - UCPD Tx Data Register
    pub txdr: TXDR,
    ///0x28 - UCPD Rx Ordered Set Register
    pub rx_ordset: RX_ORDSET,
    ///0x2c - UCPD Rx Paysize Register
    pub rx_paysz: RX_PAYSZ,
    ///0x30 - UCPD Rx Data Register
    pub rxdr: RXDR,
    ///0x34 - UCPD Rx Ordered Set Extension Register 1
    pub rx_ordext1: RX_ORDEXT1,
    ///0x38 - UCPD Rx Ordered Set Extension Register 2
    pub rx_ordext2: RX_ORDEXT2,
}
///CFG1 (rw) register accessor: an alias for `Reg<CFG1_SPEC>`
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
///UCPD configuration register 1
pub mod cfg1;
///CFG2 (rw) register accessor: an alias for `Reg<CFG2_SPEC>`
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
///UCPD configuration register 2
pub mod cfg2;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///UCPD configuration register 2
pub mod cr;
///IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`
pub type IMR = crate::Reg<imr::IMR_SPEC>;
///UCPD Interrupt Mask Register
pub mod imr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///UCPD Status Register
pub mod sr;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///UCPD Interrupt Clear Register
pub mod icr;
///TX_ORDSET (rw) register accessor: an alias for `Reg<TX_ORDSET_SPEC>`
pub type TX_ORDSET = crate::Reg<tx_ordset::TX_ORDSET_SPEC>;
///UCPD Tx Ordered Set Type Register
pub mod tx_ordset;
///TX_PAYSZ (rw) register accessor: an alias for `Reg<TX_PAYSZ_SPEC>`
pub type TX_PAYSZ = crate::Reg<tx_paysz::TX_PAYSZ_SPEC>;
///UCPD Tx Paysize Register
pub mod tx_paysz;
///TXDR (rw) register accessor: an alias for `Reg<TXDR_SPEC>`
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
///UCPD Tx Data Register
pub mod txdr;
///RX_ORDSET (r) register accessor: an alias for `Reg<RX_ORDSET_SPEC>`
pub type RX_ORDSET = crate::Reg<rx_ordset::RX_ORDSET_SPEC>;
///UCPD Rx Ordered Set Register
pub mod rx_ordset;
///RX_PAYSZ (r) register accessor: an alias for `Reg<RX_PAYSZ_SPEC>`
pub type RX_PAYSZ = crate::Reg<rx_paysz::RX_PAYSZ_SPEC>;
///UCPD Rx Paysize Register
pub mod rx_paysz;
///RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
///UCPD Rx Data Register
pub mod rxdr;
///RX_ORDEXT1 (rw) register accessor: an alias for `Reg<RX_ORDEXT1_SPEC>`
pub type RX_ORDEXT1 = crate::Reg<rx_ordext1::RX_ORDEXT1_SPEC>;
///UCPD Rx Ordered Set Extension Register 1
pub mod rx_ordext1;
///RX_ORDEXT2 (rw) register accessor: an alias for `Reg<RX_ORDEXT2_SPEC>`
pub type RX_ORDEXT2 = crate::Reg<rx_ordext2::RX_ORDEXT2_SPEC>;
///UCPD Rx Ordered Set Extension Register 2
pub mod rx_ordext2;
