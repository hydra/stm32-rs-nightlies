///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - The Operating Mode register establishes the Transmit and Receive operating modes and commands.
    pub eth_mtlomr: ETH_MTLOMR,
    _reserved1: [u8; 0x1c],
    ///0x20 - The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
    pub eth_mtlisr: ETH_MTLISR,
    _reserved2: [u8; 0xdc],
    ///0x100 - Tx queue 0 operating mode Register
    pub eth_mtltx_q0omr: ETH_MTLTX_Q0OMR,
    ///0x104 - Tx queue 0 underflow register
    pub eth_mtltx_q0ur: ETH_MTLTX_Q0UR,
    ///0x108 - Tx queue 0 underflow register
    pub eth_mtltx_q0dr: ETH_MTLTX_Q0DR,
    _reserved5: [u8; 0x08],
    ///0x114 - Tx queue x ETS status Register
    pub eth_mtltx_q0esr: ETH_MTLTX_Q0ESR,
    _reserved6: [u8; 0x14],
    ///0x12c - Queue 0 interrupt control status Register
    pub eth_mtlq0icsr: ETH_MTLQ0ICSR,
    ///0x130 - Rx queue 0 operating mode register
    pub eth_mtlrx_q0omr: ETH_MTLRX_Q0OMR,
    ///0x134 - Rx queue 0 missed packet and overflow counter register
    pub eth_mtlrx_q0mpocr: ETH_MTLRX_Q0MPOCR,
    ///0x138 - Rx queue i debug register
    pub eth_mtlrx_q0dr: ETH_MTLRX_Q0DR,
    ///0x13c - Rx queue 0 control register
    pub eth_mtlrx_q0cr: ETH_MTLRX_Q0CR,
    ///0x140 - Tx queue 1 operating mode Register
    pub eth_mtltx_q1omr: ETH_MTLTX_Q1OMR,
    ///0x144 - Tx queue 1 underflow register
    pub eth_mtltx_q1ur: ETH_MTLTX_Q1UR,
    ///0x148 - Tx queue 1 underflow register
    pub eth_mtltx_q1dr: ETH_MTLTX_Q1DR,
    _reserved14: [u8; 0x04],
    ///0x150 - The Queue ETS Control register controls the enhanced transmission selection operation.
    pub eth_mtltx_q1ecr: ETH_MTLTX_Q1ECR,
    ///0x154 - Tx queue x ETS status Register
    pub eth_mtltx_q1esr: ETH_MTLTX_Q1ESR,
    ///0x158 - This register provides the average traffic transmitted on queue 1.
    pub eth_mtltx_q1qwr: ETH_MTLTX_Q1QWR,
    ///0x15c - The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
    pub eth_mtltx_q1sscr: ETH_MTLTX_Q1SSCR,
    ///0x160 - The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
    pub eth_mtltx_q1hcr: ETH_MTLTX_Q1HCR,
    ///0x164 - The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
    pub eth_mtltx_q1lcr: ETH_MTLTX_Q1LCR,
    _reserved20: [u8; 0x04],
    ///0x16c - Queue 1 interrupt control status Register
    pub eth_mtlq1icsr: ETH_MTLQ1ICSR,
    ///0x170 - Rx queue 1 operating mode register
    pub eth_mtlrx_q1omr: ETH_MTLRX_Q1OMR,
    ///0x174 - Rx queue 1 missed packet and overflow counter register
    pub eth_mtlrx_q1mpocr: ETH_MTLRX_Q1MPOCR,
    ///0x178 - Rx queue i debug register
    pub eth_mtlrx_q1dr: ETH_MTLRX_Q1DR,
    ///0x17c - Rx queue 1 control register
    pub eth_mtlrx_q1cr: ETH_MTLRX_Q1CR,
}
///ETH_MTLOMR (rw) register accessor: an alias for `Reg<ETH_MTLOMR_SPEC>`
pub type ETH_MTLOMR = crate::Reg<eth_mtlomr::ETH_MTLOMR_SPEC>;
///The Operating Mode register establishes the Transmit and Receive operating modes and commands.
pub mod eth_mtlomr;
///ETH_MTLISR (r) register accessor: an alias for `Reg<ETH_MTLISR_SPEC>`
pub type ETH_MTLISR = crate::Reg<eth_mtlisr::ETH_MTLISR_SPEC>;
///The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
pub mod eth_mtlisr;
///ETH_MTLTxQ0OMR (rw) register accessor: an alias for `Reg<ETH_MTLTX_Q0OMR_SPEC>`
pub type ETH_MTLTX_Q0OMR = crate::Reg<eth_mtltx_q0omr::ETH_MTLTX_Q0OMR_SPEC>;
///Tx queue 0 operating mode Register
pub mod eth_mtltx_q0omr;
///ETH_MTLTxQ1OMR (rw) register accessor: an alias for `Reg<ETH_MTLTX_Q1OMR_SPEC>`
pub type ETH_MTLTX_Q1OMR = crate::Reg<eth_mtltx_q1omr::ETH_MTLTX_Q1OMR_SPEC>;
///Tx queue 1 operating mode Register
pub mod eth_mtltx_q1omr;
///ETH_MTLTxQ0UR (r) register accessor: an alias for `Reg<ETH_MTLTX_Q0UR_SPEC>`
pub type ETH_MTLTX_Q0UR = crate::Reg<eth_mtltx_q0ur::ETH_MTLTX_Q0UR_SPEC>;
///Tx queue 0 underflow register
pub mod eth_mtltx_q0ur;
///ETH_MTLTxQ1UR (r) register accessor: an alias for `Reg<ETH_MTLTX_Q1UR_SPEC>`
pub type ETH_MTLTX_Q1UR = crate::Reg<eth_mtltx_q1ur::ETH_MTLTX_Q1UR_SPEC>;
///Tx queue 1 underflow register
pub mod eth_mtltx_q1ur;
///ETH_MTLTxQ0DR (r) register accessor: an alias for `Reg<ETH_MTLTX_Q0DR_SPEC>`
pub type ETH_MTLTX_Q0DR = crate::Reg<eth_mtltx_q0dr::ETH_MTLTX_Q0DR_SPEC>;
///Tx queue 0 underflow register
pub mod eth_mtltx_q0dr;
///ETH_MTLTxQ1DR (r) register accessor: an alias for `Reg<ETH_MTLTX_Q1DR_SPEC>`
pub type ETH_MTLTX_Q1DR = crate::Reg<eth_mtltx_q1dr::ETH_MTLTX_Q1DR_SPEC>;
///Tx queue 1 underflow register
pub mod eth_mtltx_q1dr;
///ETH_MTLTxQ0ESR (r) register accessor: an alias for `Reg<ETH_MTLTX_Q0ESR_SPEC>`
pub type ETH_MTLTX_Q0ESR = crate::Reg<eth_mtltx_q0esr::ETH_MTLTX_Q0ESR_SPEC>;
///Tx queue x ETS status Register
pub mod eth_mtltx_q0esr;
///ETH_MTLTxQ1ESR (r) register accessor: an alias for `Reg<ETH_MTLTX_Q1ESR_SPEC>`
pub type ETH_MTLTX_Q1ESR = crate::Reg<eth_mtltx_q1esr::ETH_MTLTX_Q1ESR_SPEC>;
///Tx queue x ETS status Register
pub mod eth_mtltx_q1esr;
///ETH_MTLQ0ICSR (rw) register accessor: an alias for `Reg<ETH_MTLQ0ICSR_SPEC>`
pub type ETH_MTLQ0ICSR = crate::Reg<eth_mtlq0icsr::ETH_MTLQ0ICSR_SPEC>;
///Queue 0 interrupt control status Register
pub mod eth_mtlq0icsr;
///ETH_MTLQ1ICSR (rw) register accessor: an alias for `Reg<ETH_MTLQ1ICSR_SPEC>`
pub type ETH_MTLQ1ICSR = crate::Reg<eth_mtlq1icsr::ETH_MTLQ1ICSR_SPEC>;
///Queue 1 interrupt control status Register
pub mod eth_mtlq1icsr;
///ETH_MTLRxQ0OMR (rw) register accessor: an alias for `Reg<ETH_MTLRX_Q0OMR_SPEC>`
pub type ETH_MTLRX_Q0OMR = crate::Reg<eth_mtlrx_q0omr::ETH_MTLRX_Q0OMR_SPEC>;
///Rx queue 0 operating mode register
pub mod eth_mtlrx_q0omr;
///ETH_MTLRxQ1OMR (rw) register accessor: an alias for `Reg<ETH_MTLRX_Q1OMR_SPEC>`
pub type ETH_MTLRX_Q1OMR = crate::Reg<eth_mtlrx_q1omr::ETH_MTLRX_Q1OMR_SPEC>;
///Rx queue 1 operating mode register
pub mod eth_mtlrx_q1omr;
///ETH_MTLRxQ0MPOCR (r) register accessor: an alias for `Reg<ETH_MTLRX_Q0MPOCR_SPEC>`
pub type ETH_MTLRX_Q0MPOCR = crate::Reg<eth_mtlrx_q0mpocr::ETH_MTLRX_Q0MPOCR_SPEC>;
///Rx queue 0 missed packet and overflow counter register
pub mod eth_mtlrx_q0mpocr;
///ETH_MTLRxQ1MPOCR (r) register accessor: an alias for `Reg<ETH_MTLRX_Q1MPOCR_SPEC>`
pub type ETH_MTLRX_Q1MPOCR = crate::Reg<eth_mtlrx_q1mpocr::ETH_MTLRX_Q1MPOCR_SPEC>;
///Rx queue 1 missed packet and overflow counter register
pub mod eth_mtlrx_q1mpocr;
///ETH_MTLRxQ0DR (r) register accessor: an alias for `Reg<ETH_MTLRX_Q0DR_SPEC>`
pub type ETH_MTLRX_Q0DR = crate::Reg<eth_mtlrx_q0dr::ETH_MTLRX_Q0DR_SPEC>;
///Rx queue i debug register
pub mod eth_mtlrx_q0dr;
///ETH_MTLRxQ1DR (r) register accessor: an alias for `Reg<ETH_MTLRX_Q1DR_SPEC>`
pub type ETH_MTLRX_Q1DR = crate::Reg<eth_mtlrx_q1dr::ETH_MTLRX_Q1DR_SPEC>;
///Rx queue i debug register
pub mod eth_mtlrx_q1dr;
///ETH_MTLRxQ0CR (rw) register accessor: an alias for `Reg<ETH_MTLRX_Q0CR_SPEC>`
pub type ETH_MTLRX_Q0CR = crate::Reg<eth_mtlrx_q0cr::ETH_MTLRX_Q0CR_SPEC>;
///Rx queue 0 control register
pub mod eth_mtlrx_q0cr;
///ETH_MTLRxQ1CR (rw) register accessor: an alias for `Reg<ETH_MTLRX_Q1CR_SPEC>`
pub type ETH_MTLRX_Q1CR = crate::Reg<eth_mtlrx_q1cr::ETH_MTLRX_Q1CR_SPEC>;
///Rx queue 1 control register
pub mod eth_mtlrx_q1cr;
///ETH_MTLTxQ1ECR (rw) register accessor: an alias for `Reg<ETH_MTLTX_Q1ECR_SPEC>`
pub type ETH_MTLTX_Q1ECR = crate::Reg<eth_mtltx_q1ecr::ETH_MTLTX_Q1ECR_SPEC>;
///The Queue ETS Control register controls the enhanced transmission selection operation.
pub mod eth_mtltx_q1ecr;
///ETH_MTLTxQ1QWR (rw) register accessor: an alias for `Reg<ETH_MTLTX_Q1QWR_SPEC>`
pub type ETH_MTLTX_Q1QWR = crate::Reg<eth_mtltx_q1qwr::ETH_MTLTX_Q1QWR_SPEC>;
///This register provides the average traffic transmitted on queue 1.
pub mod eth_mtltx_q1qwr;
///ETH_MTLTxQ1SSCR (rw) register accessor: an alias for `Reg<ETH_MTLTX_Q1SSCR_SPEC>`
pub type ETH_MTLTX_Q1SSCR = crate::Reg<eth_mtltx_q1sscr::ETH_MTLTX_Q1SSCR_SPEC>;
///The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
pub mod eth_mtltx_q1sscr;
///ETH_MTLTxQ1HCR (rw) register accessor: an alias for `Reg<ETH_MTLTX_Q1HCR_SPEC>`
pub type ETH_MTLTX_Q1HCR = crate::Reg<eth_mtltx_q1hcr::ETH_MTLTX_Q1HCR_SPEC>;
///The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
pub mod eth_mtltx_q1hcr;
///ETH_MTLTxQ1LCR (rw) register accessor: an alias for `Reg<ETH_MTLTX_Q1LCR_SPEC>`
pub type ETH_MTLTX_Q1LCR = crate::Reg<eth_mtltx_q1lcr::ETH_MTLTX_Q1LCR_SPEC>;
///The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
pub mod eth_mtltx_q1lcr;
