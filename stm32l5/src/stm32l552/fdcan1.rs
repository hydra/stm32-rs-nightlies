///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FDCAN Core Release Register
    pub fdcan_crel: FDCAN_CREL,
    ///0x04 - FDCAN Core Release Register
    pub fdcan_endn: FDCAN_ENDN,
    _reserved2: [u8; 0x04],
    ///0x0c - FDCAN Data Bit Timing and Prescaler Register
    pub fdcan_dbtp: FDCAN_DBTP,
    ///0x10 - FDCAN Test Register
    pub fdcan_test: FDCAN_TEST,
    ///0x14 - FDCAN RAM Watchdog Register
    pub fdcan_rwd: FDCAN_RWD,
    ///0x18 - FDCAN CC Control Register
    pub fdcan_cccr: FDCAN_CCCR,
    ///0x1c - FDCAN Nominal Bit Timing and Prescaler Register
    pub fdcan_nbtp: FDCAN_NBTP,
    ///0x20 - FDCAN Timestamp Counter Configuration Register
    pub fdcan_tscc: FDCAN_TSCC,
    ///0x24 - FDCAN Timestamp Counter Value Register
    pub fdcan_tscv: FDCAN_TSCV,
    ///0x28 - FDCAN Timeout Counter Configuration Register
    pub fdcan_tocc: FDCAN_TOCC,
    ///0x2c - FDCAN Timeout Counter Value Register
    pub fdcan_tocv: FDCAN_TOCV,
    _reserved11: [u8; 0x10],
    ///0x40 - FDCAN Error Counter Register
    pub fdcan_ecr: FDCAN_ECR,
    ///0x44 - FDCAN Protocol Status Register
    pub fdcan_psr: FDCAN_PSR,
    ///0x48 - FDCAN Transmitter Delay Compensation Register
    pub fdcan_tdcr: FDCAN_TDCR,
    _reserved14: [u8; 0x04],
    ///0x50 - FDCAN Interrupt Register
    pub fdcan_ir: FDCAN_IR,
    ///0x54 - FDCAN Interrupt Enable Register
    pub fdcan_ie: FDCAN_IE,
    ///0x58 - FDCAN Interrupt Line Select Register
    pub fdcan_ils: FDCAN_ILS,
    ///0x5c - FDCAN Interrupt Line Enable Register
    pub fdcan_ile: FDCAN_ILE,
    _reserved18: [u8; 0x20],
    ///0x80 - FDCAN Global Filter Configuration Register
    pub fdcan_rxgfc: FDCAN_RXGFC,
    ///0x84 - FDCAN Extended ID and Mask Register
    pub fdcan_xidam: FDCAN_XIDAM,
    ///0x88 - FDCAN High Priority Message Status Register
    pub fdcan_hpms: FDCAN_HPMS,
    _reserved21: [u8; 0x04],
    ///0x90 - FDCAN Rx FIFO 0 Status Register
    pub fdcan_rxf0s: FDCAN_RXF0S,
    ///0x94 - CAN Rx FIFO 0 Acknowledge Register
    pub fdcan_rxf0a: FDCAN_RXF0A,
    ///0x98 - FDCAN Rx FIFO 1 Status Register
    pub fdcan_rxf1s: FDCAN_RXF1S,
    ///0x9c - FDCAN Rx FIFO 1 Acknowledge Register
    pub fdcan_rxf1a: FDCAN_RXF1A,
    _reserved25: [u8; 0x20],
    ///0xc0 - FDCAN Tx buffer configuration register
    pub fdcan_txbc: FDCAN_TXBC,
    ///0xc4 - FDCAN Tx FIFO/Queue Status Register
    pub fdcan_txfqs: FDCAN_TXFQS,
    ///0xc8 - FDCAN Tx Buffer Request Pending Register
    pub fdcan_txbrp: FDCAN_TXBRP,
    ///0xcc - FDCAN Tx Buffer Add Request Register
    pub fdcan_txbar: FDCAN_TXBAR,
    ///0xd0 - FDCAN Tx Buffer Cancellation Request Register
    pub fdcan_txbcr: FDCAN_TXBCR,
    ///0xd4 - FDCAN Tx Buffer Transmission Occurred Register
    pub fdcan_txbto: FDCAN_TXBTO,
    ///0xd8 - FDCAN Tx Buffer Cancellation Finished Register
    pub fdcan_txbcf: FDCAN_TXBCF,
    ///0xdc - FDCAN Tx Buffer Transmission Interrupt Enable Register
    pub fdcan_txbtie: FDCAN_TXBTIE,
    ///0xe0 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
    pub fdcan_txbcie: FDCAN_TXBCIE,
    ///0xe4 - FDCAN Tx Event FIFO Status Register
    pub fdcan_txefs: FDCAN_TXEFS,
    ///0xe8 - FDCAN Tx Event FIFO Acknowledge Register
    pub fdcan_txefa: FDCAN_TXEFA,
    _reserved36: [u8; 0x14],
    ///0x100 - FDCAN TT Trigger Memory Configuration Register
    pub fdcan_ckdiv: FDCAN_CKDIV,
}
///FDCAN_CREL (r) register accessor: an alias for `Reg<FDCAN_CREL_SPEC>`
pub type FDCAN_CREL = crate::Reg<fdcan_crel::FDCAN_CREL_SPEC>;
///FDCAN Core Release Register
pub mod fdcan_crel;
///FDCAN_ENDN (r) register accessor: an alias for `Reg<FDCAN_ENDN_SPEC>`
pub type FDCAN_ENDN = crate::Reg<fdcan_endn::FDCAN_ENDN_SPEC>;
///FDCAN Core Release Register
pub mod fdcan_endn;
///FDCAN_DBTP (rw) register accessor: an alias for `Reg<FDCAN_DBTP_SPEC>`
pub type FDCAN_DBTP = crate::Reg<fdcan_dbtp::FDCAN_DBTP_SPEC>;
///FDCAN Data Bit Timing and Prescaler Register
pub mod fdcan_dbtp;
///FDCAN_TEST (rw) register accessor: an alias for `Reg<FDCAN_TEST_SPEC>`
pub type FDCAN_TEST = crate::Reg<fdcan_test::FDCAN_TEST_SPEC>;
///FDCAN Test Register
pub mod fdcan_test;
///FDCAN_RWD (rw) register accessor: an alias for `Reg<FDCAN_RWD_SPEC>`
pub type FDCAN_RWD = crate::Reg<fdcan_rwd::FDCAN_RWD_SPEC>;
///FDCAN RAM Watchdog Register
pub mod fdcan_rwd;
///FDCAN_CCCR (rw) register accessor: an alias for `Reg<FDCAN_CCCR_SPEC>`
pub type FDCAN_CCCR = crate::Reg<fdcan_cccr::FDCAN_CCCR_SPEC>;
///FDCAN CC Control Register
pub mod fdcan_cccr;
///FDCAN_NBTP (rw) register accessor: an alias for `Reg<FDCAN_NBTP_SPEC>`
pub type FDCAN_NBTP = crate::Reg<fdcan_nbtp::FDCAN_NBTP_SPEC>;
///FDCAN Nominal Bit Timing and Prescaler Register
pub mod fdcan_nbtp;
///FDCAN_TSCC (rw) register accessor: an alias for `Reg<FDCAN_TSCC_SPEC>`
pub type FDCAN_TSCC = crate::Reg<fdcan_tscc::FDCAN_TSCC_SPEC>;
///FDCAN Timestamp Counter Configuration Register
pub mod fdcan_tscc;
///FDCAN_TSCV (rw) register accessor: an alias for `Reg<FDCAN_TSCV_SPEC>`
pub type FDCAN_TSCV = crate::Reg<fdcan_tscv::FDCAN_TSCV_SPEC>;
///FDCAN Timestamp Counter Value Register
pub mod fdcan_tscv;
///FDCAN_TOCC (rw) register accessor: an alias for `Reg<FDCAN_TOCC_SPEC>`
pub type FDCAN_TOCC = crate::Reg<fdcan_tocc::FDCAN_TOCC_SPEC>;
///FDCAN Timeout Counter Configuration Register
pub mod fdcan_tocc;
///FDCAN_TOCV (rw) register accessor: an alias for `Reg<FDCAN_TOCV_SPEC>`
pub type FDCAN_TOCV = crate::Reg<fdcan_tocv::FDCAN_TOCV_SPEC>;
///FDCAN Timeout Counter Value Register
pub mod fdcan_tocv;
///FDCAN_ECR (rw) register accessor: an alias for `Reg<FDCAN_ECR_SPEC>`
pub type FDCAN_ECR = crate::Reg<fdcan_ecr::FDCAN_ECR_SPEC>;
///FDCAN Error Counter Register
pub mod fdcan_ecr;
///FDCAN_PSR (rw) register accessor: an alias for `Reg<FDCAN_PSR_SPEC>`
pub type FDCAN_PSR = crate::Reg<fdcan_psr::FDCAN_PSR_SPEC>;
///FDCAN Protocol Status Register
pub mod fdcan_psr;
///FDCAN_TDCR (rw) register accessor: an alias for `Reg<FDCAN_TDCR_SPEC>`
pub type FDCAN_TDCR = crate::Reg<fdcan_tdcr::FDCAN_TDCR_SPEC>;
///FDCAN Transmitter Delay Compensation Register
pub mod fdcan_tdcr;
///FDCAN_IR (rw) register accessor: an alias for `Reg<FDCAN_IR_SPEC>`
pub type FDCAN_IR = crate::Reg<fdcan_ir::FDCAN_IR_SPEC>;
///FDCAN Interrupt Register
pub mod fdcan_ir;
///FDCAN_IE (rw) register accessor: an alias for `Reg<FDCAN_IE_SPEC>`
pub type FDCAN_IE = crate::Reg<fdcan_ie::FDCAN_IE_SPEC>;
///FDCAN Interrupt Enable Register
pub mod fdcan_ie;
///FDCAN_ILS (rw) register accessor: an alias for `Reg<FDCAN_ILS_SPEC>`
pub type FDCAN_ILS = crate::Reg<fdcan_ils::FDCAN_ILS_SPEC>;
///FDCAN Interrupt Line Select Register
pub mod fdcan_ils;
///FDCAN_ILE (rw) register accessor: an alias for `Reg<FDCAN_ILE_SPEC>`
pub type FDCAN_ILE = crate::Reg<fdcan_ile::FDCAN_ILE_SPEC>;
///FDCAN Interrupt Line Enable Register
pub mod fdcan_ile;
///FDCAN_RXGFC (rw) register accessor: an alias for `Reg<FDCAN_RXGFC_SPEC>`
pub type FDCAN_RXGFC = crate::Reg<fdcan_rxgfc::FDCAN_RXGFC_SPEC>;
///FDCAN Global Filter Configuration Register
pub mod fdcan_rxgfc;
///FDCAN_XIDAM (rw) register accessor: an alias for `Reg<FDCAN_XIDAM_SPEC>`
pub type FDCAN_XIDAM = crate::Reg<fdcan_xidam::FDCAN_XIDAM_SPEC>;
///FDCAN Extended ID and Mask Register
pub mod fdcan_xidam;
///FDCAN_HPMS (r) register accessor: an alias for `Reg<FDCAN_HPMS_SPEC>`
pub type FDCAN_HPMS = crate::Reg<fdcan_hpms::FDCAN_HPMS_SPEC>;
///FDCAN High Priority Message Status Register
pub mod fdcan_hpms;
///FDCAN_RXF0S (rw) register accessor: an alias for `Reg<FDCAN_RXF0S_SPEC>`
pub type FDCAN_RXF0S = crate::Reg<fdcan_rxf0s::FDCAN_RXF0S_SPEC>;
///FDCAN Rx FIFO 0 Status Register
pub mod fdcan_rxf0s;
///FDCAN_RXF0A (rw) register accessor: an alias for `Reg<FDCAN_RXF0A_SPEC>`
pub type FDCAN_RXF0A = crate::Reg<fdcan_rxf0a::FDCAN_RXF0A_SPEC>;
///CAN Rx FIFO 0 Acknowledge Register
pub mod fdcan_rxf0a;
///FDCAN_RXF1S (rw) register accessor: an alias for `Reg<FDCAN_RXF1S_SPEC>`
pub type FDCAN_RXF1S = crate::Reg<fdcan_rxf1s::FDCAN_RXF1S_SPEC>;
///FDCAN Rx FIFO 1 Status Register
pub mod fdcan_rxf1s;
///FDCAN_RXF1A (rw) register accessor: an alias for `Reg<FDCAN_RXF1A_SPEC>`
pub type FDCAN_RXF1A = crate::Reg<fdcan_rxf1a::FDCAN_RXF1A_SPEC>;
///FDCAN Rx FIFO 1 Acknowledge Register
pub mod fdcan_rxf1a;
///FDCAN_TXFQS (r) register accessor: an alias for `Reg<FDCAN_TXFQS_SPEC>`
pub type FDCAN_TXFQS = crate::Reg<fdcan_txfqs::FDCAN_TXFQS_SPEC>;
///FDCAN Tx FIFO/Queue Status Register
pub mod fdcan_txfqs;
///FDCAN_TXBRP (r) register accessor: an alias for `Reg<FDCAN_TXBRP_SPEC>`
pub type FDCAN_TXBRP = crate::Reg<fdcan_txbrp::FDCAN_TXBRP_SPEC>;
///FDCAN Tx Buffer Request Pending Register
pub mod fdcan_txbrp;
///FDCAN_TXBAR (rw) register accessor: an alias for `Reg<FDCAN_TXBAR_SPEC>`
pub type FDCAN_TXBAR = crate::Reg<fdcan_txbar::FDCAN_TXBAR_SPEC>;
///FDCAN Tx Buffer Add Request Register
pub mod fdcan_txbar;
///FDCAN_TXBCR (rw) register accessor: an alias for `Reg<FDCAN_TXBCR_SPEC>`
pub type FDCAN_TXBCR = crate::Reg<fdcan_txbcr::FDCAN_TXBCR_SPEC>;
///FDCAN Tx Buffer Cancellation Request Register
pub mod fdcan_txbcr;
///FDCAN_TXBTO (r) register accessor: an alias for `Reg<FDCAN_TXBTO_SPEC>`
pub type FDCAN_TXBTO = crate::Reg<fdcan_txbto::FDCAN_TXBTO_SPEC>;
///FDCAN Tx Buffer Transmission Occurred Register
pub mod fdcan_txbto;
///FDCAN_TXBCF (r) register accessor: an alias for `Reg<FDCAN_TXBCF_SPEC>`
pub type FDCAN_TXBCF = crate::Reg<fdcan_txbcf::FDCAN_TXBCF_SPEC>;
///FDCAN Tx Buffer Cancellation Finished Register
pub mod fdcan_txbcf;
///FDCAN_TXBTIE (rw) register accessor: an alias for `Reg<FDCAN_TXBTIE_SPEC>`
pub type FDCAN_TXBTIE = crate::Reg<fdcan_txbtie::FDCAN_TXBTIE_SPEC>;
///FDCAN Tx Buffer Transmission Interrupt Enable Register
pub mod fdcan_txbtie;
///FDCAN_TXBCIE (rw) register accessor: an alias for `Reg<FDCAN_TXBCIE_SPEC>`
pub type FDCAN_TXBCIE = crate::Reg<fdcan_txbcie::FDCAN_TXBCIE_SPEC>;
///FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
pub mod fdcan_txbcie;
///FDCAN_TXEFS (r) register accessor: an alias for `Reg<FDCAN_TXEFS_SPEC>`
pub type FDCAN_TXEFS = crate::Reg<fdcan_txefs::FDCAN_TXEFS_SPEC>;
///FDCAN Tx Event FIFO Status Register
pub mod fdcan_txefs;
///FDCAN_TXEFA (rw) register accessor: an alias for `Reg<FDCAN_TXEFA_SPEC>`
pub type FDCAN_TXEFA = crate::Reg<fdcan_txefa::FDCAN_TXEFA_SPEC>;
///FDCAN Tx Event FIFO Acknowledge Register
pub mod fdcan_txefa;
///FDCAN_CKDIV (rw) register accessor: an alias for `Reg<FDCAN_CKDIV_SPEC>`
pub type FDCAN_CKDIV = crate::Reg<fdcan_ckdiv::FDCAN_CKDIV_SPEC>;
///FDCAN TT Trigger Memory Configuration Register
pub mod fdcan_ckdiv;
///FDCAN_TXBC (rw) register accessor: an alias for `Reg<FDCAN_TXBC_SPEC>`
pub type FDCAN_TXBC = crate::Reg<fdcan_txbc::FDCAN_TXBC_SPEC>;
///FDCAN Tx buffer configuration register
pub mod fdcan_txbc;
