///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FDCAN Core Release Register
    pub crel: CREL,
    ///0x04 - FDCAN Core Release Register
    pub endn: ENDN,
    _reserved2: [u8; 0x04],
    ///0x0c - FDCAN Data Bit Timing and Prescaler Register
    pub dbtp: DBTP,
    ///0x10 - FDCAN Test Register
    pub test: TEST,
    ///0x14 - FDCAN RAM Watchdog Register
    pub rwd: RWD,
    ///0x18 - FDCAN CC Control Register
    pub cccr: CCCR,
    ///0x1c - FDCAN Nominal Bit Timing and Prescaler Register
    pub nbtp: NBTP,
    ///0x20 - FDCAN Timestamp Counter Configuration Register
    pub tscc: TSCC,
    ///0x24 - FDCAN Timestamp Counter Value Register
    pub tscv: TSCV,
    ///0x28 - FDCAN Timeout Counter Configuration Register
    pub tocc: TOCC,
    ///0x2c - FDCAN Timeout Counter Value Register
    pub tocv: TOCV,
    _reserved11: [u8; 0x10],
    ///0x40 - FDCAN Error Counter Register
    pub ecr: ECR,
    ///0x44 - FDCAN Protocol Status Register
    pub psr: PSR,
    ///0x48 - FDCAN Transmitter Delay Compensation Register
    pub tdcr: TDCR,
    _reserved14: [u8; 0x04],
    ///0x50 - FDCAN Interrupt Register
    pub ir: IR,
    ///0x54 - FDCAN Interrupt Enable Register
    pub ie: IE,
    ///0x58 - FDCAN Interrupt Line Select Register
    pub ils: ILS,
    ///0x5c - FDCAN Interrupt Line Enable Register
    pub ile: ILE,
    _reserved18: [u8; 0x20],
    ///0x80 - FDCAN Global Filter Configuration Register
    pub gfc: GFC,
    ///0x84 - FDCAN Standard ID Filter Configuration Register
    pub sidfc: SIDFC,
    ///0x88 - FDCAN Extended ID Filter Configuration Register
    pub xidfc: XIDFC,
    _reserved21: [u8; 0x04],
    ///0x90 - FDCAN Extended ID and Mask Register
    pub xidam: XIDAM,
    ///0x94 - FDCAN High Priority Message Status Register
    pub hpms: HPMS,
    ///0x98 - FDCAN New Data 1 Register
    pub ndat1: NDAT1,
    ///0x9c - FDCAN New Data 2 Register
    pub ndat2: NDAT2,
    ///0xa0 - FDCAN Rx FIFO 0 Configuration Register
    pub rxf0c: RXF0C,
    ///0xa4 - FDCAN Rx FIFO 0 Status Register
    pub rxf0s: RXF0S,
    ///0xa8 - CAN Rx FIFO 0 Acknowledge Register
    pub rxf0a: RXF0A,
    ///0xac - FDCAN Rx Buffer Configuration Register
    pub rxbc: RXBC,
    ///0xb0 - FDCAN Rx FIFO 1 Configuration Register
    pub rxf1c: RXF1C,
    ///0xb4 - FDCAN Rx FIFO 1 Status Register
    pub rxf1s: RXF1S,
    ///0xb8 - FDCAN Rx FIFO 1 Acknowledge Register
    pub rxf1a: RXF1A,
    ///0xbc - FDCAN Rx Buffer Element Size Configuration Register
    pub rxesc: RXESC,
    ///0xc0 - FDCAN Tx Buffer Configuration Register
    pub txbc: TXBC,
    ///0xc4 - FDCAN Tx FIFO/Queue Status Register
    pub txfqs: TXFQS,
    ///0xc8 - FDCAN Tx Buffer Element Size Configuration Register
    pub txesc: TXESC,
    ///0xcc - FDCAN Tx Buffer Request Pending Register
    pub txbrp: TXBRP,
    ///0xd0 - FDCAN Tx Buffer Add Request Register
    pub txbar: TXBAR,
    ///0xd4 - FDCAN Tx Buffer Cancellation Request Register
    pub txbcr: TXBCR,
    ///0xd8 - FDCAN Tx Buffer Transmission Occurred Register
    pub txbto: TXBTO,
    ///0xdc - FDCAN Tx Buffer Cancellation Finished Register
    pub txbcf: TXBCF,
    ///0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register
    pub txbtie: TXBTIE,
    ///0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
    pub txbcie: TXBCIE,
    _reserved43: [u8; 0x08],
    ///0xf0 - FDCAN Tx Event FIFO Configuration Register
    pub txefc: TXEFC,
    ///0xf4 - FDCAN Tx Event FIFO Status Register
    pub txefs: TXEFS,
    ///0xf8 - FDCAN Tx Event FIFO Acknowledge Register
    pub txefa: TXEFA,
    _reserved46: [u8; 0x04],
    ///0x100 - FDCAN TT Trigger Memory Configuration Register
    pub tttmc: TTTMC,
    ///0x104 - FDCAN TT Reference Message Configuration Register
    pub ttrmc: TTRMC,
    ///0x108 - FDCAN TT Operation Configuration Register
    pub ttocf: TTOCF,
    ///0x10c - FDCAN TT Matrix Limits Register
    pub ttmlm: TTMLM,
    ///0x110 - FDCAN TUR Configuration Register
    pub turcf: TURCF,
    ///0x114 - FDCAN TT Operation Control Register
    pub ttocn: TTOCN,
    ///0x118 - FDCAN TT Global Time Preset Register
    pub ttgtp: TTGTP,
    ///0x11c - FDCAN TT Time Mark Register
    pub tttmk: TTTMK,
    ///0x120 - FDCAN TT Interrupt Register
    pub ttir: TTIR,
    ///0x124 - FDCAN TT Interrupt Enable Register
    pub ttie: TTIE,
    ///0x128 - FDCAN TT Interrupt Line Select Register
    pub ttils: TTILS,
    ///0x12c - FDCAN TT Operation Status Register
    pub ttost: TTOST,
    ///0x130 - FDCAN TUR Numerator Actual Register
    pub turna: TURNA,
    ///0x134 - FDCAN TT Local and Global Time Register
    pub ttlgt: TTLGT,
    ///0x138 - FDCAN TT Cycle Time and Count Register
    pub ttctc: TTCTC,
    ///0x13c - FDCAN TT Capture Time Register
    pub ttcpt: TTCPT,
    ///0x140 - FDCAN TT Cycle Sync Mark Register
    pub ttcsm: TTCSM,
    _reserved63: [u8; 0x01bc],
    ///0x300 - FDCAN TT Trigger Select Register
    pub ttts: TTTS,
}
///CREL (r) register accessor: an alias for `Reg<CREL_SPEC>`
pub type CREL = crate::Reg<crel::CREL_SPEC>;
///FDCAN Core Release Register
pub mod crel;
///ENDN (r) register accessor: an alias for `Reg<ENDN_SPEC>`
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
///FDCAN Core Release Register
pub mod endn;
///DBTP (rw) register accessor: an alias for `Reg<DBTP_SPEC>`
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
///FDCAN Data Bit Timing and Prescaler Register
pub mod dbtp;
///TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`
pub type TEST = crate::Reg<test::TEST_SPEC>;
///FDCAN Test Register
pub mod test;
///RWD (r) register accessor: an alias for `Reg<RWD_SPEC>`
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
///FDCAN RAM Watchdog Register
pub mod rwd;
///CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///FDCAN CC Control Register
pub mod cccr;
///NBTP (rw) register accessor: an alias for `Reg<NBTP_SPEC>`
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
///FDCAN Nominal Bit Timing and Prescaler Register
pub mod nbtp;
///TSCC (rw) register accessor: an alias for `Reg<TSCC_SPEC>`
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
///FDCAN Timestamp Counter Configuration Register
pub mod tscc;
///TSCV (rw) register accessor: an alias for `Reg<TSCV_SPEC>`
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
///FDCAN Timestamp Counter Value Register
pub mod tscv;
///TOCC (rw) register accessor: an alias for `Reg<TOCC_SPEC>`
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
///FDCAN Timeout Counter Configuration Register
pub mod tocc;
///TOCV (rw) register accessor: an alias for `Reg<TOCV_SPEC>`
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
///FDCAN Timeout Counter Value Register
pub mod tocv;
///ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
///FDCAN Error Counter Register
pub mod ecr;
///PSR (rw) register accessor: an alias for `Reg<PSR_SPEC>`
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///FDCAN Protocol Status Register
pub mod psr;
///TDCR (rw) register accessor: an alias for `Reg<TDCR_SPEC>`
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
///FDCAN Transmitter Delay Compensation Register
pub mod tdcr;
///IR (rw) register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///FDCAN Interrupt Register
pub mod ir;
///IE (rw) register accessor: an alias for `Reg<IE_SPEC>`
pub type IE = crate::Reg<ie::IE_SPEC>;
///FDCAN Interrupt Enable Register
pub mod ie;
///ILS (rw) register accessor: an alias for `Reg<ILS_SPEC>`
pub type ILS = crate::Reg<ils::ILS_SPEC>;
///FDCAN Interrupt Line Select Register
pub mod ils;
///ILE (rw) register accessor: an alias for `Reg<ILE_SPEC>`
pub type ILE = crate::Reg<ile::ILE_SPEC>;
///FDCAN Interrupt Line Enable Register
pub mod ile;
///GFC (rw) register accessor: an alias for `Reg<GFC_SPEC>`
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
///FDCAN Global Filter Configuration Register
pub mod gfc;
///SIDFC (rw) register accessor: an alias for `Reg<SIDFC_SPEC>`
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
///FDCAN Standard ID Filter Configuration Register
pub mod sidfc;
///XIDFC (rw) register accessor: an alias for `Reg<XIDFC_SPEC>`
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
///FDCAN Extended ID Filter Configuration Register
pub mod xidfc;
///XIDAM (rw) register accessor: an alias for `Reg<XIDAM_SPEC>`
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
///FDCAN Extended ID and Mask Register
pub mod xidam;
///HPMS (r) register accessor: an alias for `Reg<HPMS_SPEC>`
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
///FDCAN High Priority Message Status Register
pub mod hpms;
///NDAT1 (rw) register accessor: an alias for `Reg<NDAT1_SPEC>`
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
///FDCAN New Data 1 Register
pub mod ndat1;
///NDAT2 (rw) register accessor: an alias for `Reg<NDAT2_SPEC>`
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
///FDCAN New Data 2 Register
pub mod ndat2;
///RXF0C (rw) register accessor: an alias for `Reg<RXF0C_SPEC>`
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
///FDCAN Rx FIFO 0 Configuration Register
pub mod rxf0c;
///RXF0S (rw) register accessor: an alias for `Reg<RXF0S_SPEC>`
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
///FDCAN Rx FIFO 0 Status Register
pub mod rxf0s;
///RXF0A (rw) register accessor: an alias for `Reg<RXF0A_SPEC>`
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
///CAN Rx FIFO 0 Acknowledge Register
pub mod rxf0a;
///RXBC (rw) register accessor: an alias for `Reg<RXBC_SPEC>`
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
///FDCAN Rx Buffer Configuration Register
pub mod rxbc;
///RXF1C (rw) register accessor: an alias for `Reg<RXF1C_SPEC>`
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
///FDCAN Rx FIFO 1 Configuration Register
pub mod rxf1c;
///RXF1S (rw) register accessor: an alias for `Reg<RXF1S_SPEC>`
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
///FDCAN Rx FIFO 1 Status Register
pub mod rxf1s;
///RXF1A (rw) register accessor: an alias for `Reg<RXF1A_SPEC>`
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
///FDCAN Rx FIFO 1 Acknowledge Register
pub mod rxf1a;
///RXESC (rw) register accessor: an alias for `Reg<RXESC_SPEC>`
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
///FDCAN Rx Buffer Element Size Configuration Register
pub mod rxesc;
///TXBC (rw) register accessor: an alias for `Reg<TXBC_SPEC>`
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
///FDCAN Tx Buffer Configuration Register
pub mod txbc;
///TXFQS (r) register accessor: an alias for `Reg<TXFQS_SPEC>`
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
///FDCAN Tx FIFO/Queue Status Register
pub mod txfqs;
///TXESC (rw) register accessor: an alias for `Reg<TXESC_SPEC>`
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
///FDCAN Tx Buffer Element Size Configuration Register
pub mod txesc;
///TXBRP (r) register accessor: an alias for `Reg<TXBRP_SPEC>`
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
///FDCAN Tx Buffer Request Pending Register
pub mod txbrp;
///TXBAR (rw) register accessor: an alias for `Reg<TXBAR_SPEC>`
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
///FDCAN Tx Buffer Add Request Register
pub mod txbar;
///TXBCR (rw) register accessor: an alias for `Reg<TXBCR_SPEC>`
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
///FDCAN Tx Buffer Cancellation Request Register
pub mod txbcr;
///TXBTO (rw) register accessor: an alias for `Reg<TXBTO_SPEC>`
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
///FDCAN Tx Buffer Transmission Occurred Register
pub mod txbto;
///TXBCF (r) register accessor: an alias for `Reg<TXBCF_SPEC>`
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
///FDCAN Tx Buffer Cancellation Finished Register
pub mod txbcf;
///TXBTIE (rw) register accessor: an alias for `Reg<TXBTIE_SPEC>`
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
///FDCAN Tx Buffer Transmission Interrupt Enable Register
pub mod txbtie;
///TXBCIE (rw) register accessor: an alias for `Reg<TXBCIE_SPEC>`
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
///FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register
pub mod txbcie;
///TXEFC (rw) register accessor: an alias for `Reg<TXEFC_SPEC>`
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
///FDCAN Tx Event FIFO Configuration Register
pub mod txefc;
///TXEFS (rw) register accessor: an alias for `Reg<TXEFS_SPEC>`
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
///FDCAN Tx Event FIFO Status Register
pub mod txefs;
///TXEFA (rw) register accessor: an alias for `Reg<TXEFA_SPEC>`
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
///FDCAN Tx Event FIFO Acknowledge Register
pub mod txefa;
///TTTMC (rw) register accessor: an alias for `Reg<TTTMC_SPEC>`
pub type TTTMC = crate::Reg<tttmc::TTTMC_SPEC>;
///FDCAN TT Trigger Memory Configuration Register
pub mod tttmc;
///TTRMC (rw) register accessor: an alias for `Reg<TTRMC_SPEC>`
pub type TTRMC = crate::Reg<ttrmc::TTRMC_SPEC>;
///FDCAN TT Reference Message Configuration Register
pub mod ttrmc;
///TTOCF (rw) register accessor: an alias for `Reg<TTOCF_SPEC>`
pub type TTOCF = crate::Reg<ttocf::TTOCF_SPEC>;
///FDCAN TT Operation Configuration Register
pub mod ttocf;
///TTMLM (rw) register accessor: an alias for `Reg<TTMLM_SPEC>`
pub type TTMLM = crate::Reg<ttmlm::TTMLM_SPEC>;
///FDCAN TT Matrix Limits Register
pub mod ttmlm;
///TURCF (rw) register accessor: an alias for `Reg<TURCF_SPEC>`
pub type TURCF = crate::Reg<turcf::TURCF_SPEC>;
///FDCAN TUR Configuration Register
pub mod turcf;
///TTOCN (rw) register accessor: an alias for `Reg<TTOCN_SPEC>`
pub type TTOCN = crate::Reg<ttocn::TTOCN_SPEC>;
///FDCAN TT Operation Control Register
pub mod ttocn;
///TTGTP (rw) register accessor: an alias for `Reg<TTGTP_SPEC>`
pub type TTGTP = crate::Reg<ttgtp::TTGTP_SPEC>;
///FDCAN TT Global Time Preset Register
pub mod ttgtp;
///TTTMK (rw) register accessor: an alias for `Reg<TTTMK_SPEC>`
pub type TTTMK = crate::Reg<tttmk::TTTMK_SPEC>;
///FDCAN TT Time Mark Register
pub mod tttmk;
///TTIR (rw) register accessor: an alias for `Reg<TTIR_SPEC>`
pub type TTIR = crate::Reg<ttir::TTIR_SPEC>;
///FDCAN TT Interrupt Register
pub mod ttir;
///TTIE (rw) register accessor: an alias for `Reg<TTIE_SPEC>`
pub type TTIE = crate::Reg<ttie::TTIE_SPEC>;
///FDCAN TT Interrupt Enable Register
pub mod ttie;
///TTILS (rw) register accessor: an alias for `Reg<TTILS_SPEC>`
pub type TTILS = crate::Reg<ttils::TTILS_SPEC>;
///FDCAN TT Interrupt Line Select Register
pub mod ttils;
///TTOST (r) register accessor: an alias for `Reg<TTOST_SPEC>`
pub type TTOST = crate::Reg<ttost::TTOST_SPEC>;
///FDCAN TT Operation Status Register
pub mod ttost;
///TURNA (r) register accessor: an alias for `Reg<TURNA_SPEC>`
pub type TURNA = crate::Reg<turna::TURNA_SPEC>;
///FDCAN TUR Numerator Actual Register
pub mod turna;
///TTLGT (r) register accessor: an alias for `Reg<TTLGT_SPEC>`
pub type TTLGT = crate::Reg<ttlgt::TTLGT_SPEC>;
///FDCAN TT Local and Global Time Register
pub mod ttlgt;
///TTCTC (r) register accessor: an alias for `Reg<TTCTC_SPEC>`
pub type TTCTC = crate::Reg<ttctc::TTCTC_SPEC>;
///FDCAN TT Cycle Time and Count Register
pub mod ttctc;
///TTCPT (r) register accessor: an alias for `Reg<TTCPT_SPEC>`
pub type TTCPT = crate::Reg<ttcpt::TTCPT_SPEC>;
///FDCAN TT Capture Time Register
pub mod ttcpt;
///TTCSM (r) register accessor: an alias for `Reg<TTCSM_SPEC>`
pub type TTCSM = crate::Reg<ttcsm::TTCSM_SPEC>;
///FDCAN TT Cycle Sync Mark Register
pub mod ttcsm;
///TTTS (rw) register accessor: an alias for `Reg<TTTS_SPEC>`
pub type TTTS = crate::Reg<ttts::TTTS_SPEC>;
///FDCAN TT Trigger Select Register
pub mod ttts;
