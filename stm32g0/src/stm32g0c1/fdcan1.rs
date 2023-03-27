///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FDCAN core release register
    pub crel: CREL,
    ///0x04 - FDCAN endian register
    pub endn: ENDN,
    _reserved2: [u8; 0x04],
    ///0x0c - FDCAN data bit timing and prescaler register
    pub dbtp: DBTP,
    ///0x10 - FDCAN test register
    pub test: TEST,
    ///0x14 - FDCAN RAM watchdog register
    pub rwd: RWD,
    ///0x18 - FDCAN CC control register
    pub cccr: CCCR,
    ///0x1c - FDCAN nominal bit timing and prescaler register
    pub nbtp: NBTP,
    ///0x20 - FDCAN timestamp counter configuration register
    pub tscc: TSCC,
    ///0x24 - FDCAN timestamp counter value register
    pub tscv: TSCV,
    ///0x28 - FDCAN timeout counter configuration register
    pub tocc: TOCC,
    ///0x2c - FDCAN timeout counter value register
    pub tocv: TOCV,
    _reserved11: [u8; 0x10],
    ///0x40 - FDCAN error counter register
    pub ecr: ECR,
    ///0x44 - FDCAN protocol status register
    pub psr: PSR,
    ///0x48 - FDCAN transmitter delay compensation register
    pub tdcr: TDCR,
    _reserved14: [u8; 0x04],
    ///0x50 - FDCAN interrupt register
    pub ir: IR,
    ///0x54 - FDCAN interrupt enable register
    pub ie: IE,
    ///0x58 - FDCAN interrupt line select register
    pub ils: ILS,
    ///0x5c - FDCAN interrupt line enable register
    pub ile: ILE,
    _reserved18: [u8; 0x20],
    ///0x80 - FDCAN global filter configuration register
    pub rxgfc: RXGFC,
    ///0x84 - FDCAN extended ID and mask register
    pub xidam: XIDAM,
    ///0x88 - FDCAN high-priority message status register
    pub hpms: HPMS,
    _reserved21: [u8; 0x04],
    ///0x90 - FDCAN Rx FIFO 0 status register
    pub rxf0s: RXF0S,
    ///0x94 - CAN Rx FIFO 0 acknowledge register
    pub rxf0a: RXF0A,
    ///0x98 - FDCAN Rx FIFO 1 status register
    pub rxf1s: RXF1S,
    ///0x9c - FDCAN Rx FIFO 1 acknowledge register
    pub rxf1a: RXF1A,
    _reserved25: [u8; 0x20],
    ///0xc0 - FDCAN Tx buffer configuration register
    pub txbc: TXBC,
    ///0xc4 - FDCAN Tx FIFO/queue status register
    pub txfqs: TXFQS,
    ///0xc8 - FDCAN Tx buffer request pending register
    pub txbrp: TXBRP,
    ///0xcc - FDCAN Tx buffer add request register
    pub txbar: TXBAR,
    ///0xd0 - FDCAN Tx buffer cancellation request register
    pub txbcr: TXBCR,
    ///0xd4 - FDCAN Tx buffer transmission occurred register
    pub txbto: TXBTO,
    ///0xd8 - FDCAN Tx buffer cancellation finished register
    pub txbcf: TXBCF,
    ///0xdc - FDCAN Tx buffer transmission interrupt enable register
    pub txbtie: TXBTIE,
    ///0xe0 - FDCAN Tx buffer cancellation finished interrupt enable register
    pub txbcie: TXBCIE,
    ///0xe4 - FDCAN Tx event FIFO status register
    pub txefs: TXEFS,
    ///0xe8 - FDCAN Tx event FIFO acknowledge register
    pub txefa: TXEFA,
    _reserved36: [u8; 0x14],
    ///0x100 - FDCAN CFG clock divider register
    pub ckdiv: CKDIV,
}
///CREL (r) register accessor: an alias for `Reg<CREL_SPEC>`
pub type CREL = crate::Reg<crel::CREL_SPEC>;
///FDCAN core release register
pub mod crel;
///ENDN (r) register accessor: an alias for `Reg<ENDN_SPEC>`
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
///FDCAN endian register
pub mod endn;
///DBTP (rw) register accessor: an alias for `Reg<DBTP_SPEC>`
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
///FDCAN data bit timing and prescaler register
pub mod dbtp;
///TEST (rw) register accessor: an alias for `Reg<TEST_SPEC>`
pub type TEST = crate::Reg<test::TEST_SPEC>;
///FDCAN test register
pub mod test;
///RWD (rw) register accessor: an alias for `Reg<RWD_SPEC>`
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
///FDCAN RAM watchdog register
pub mod rwd;
///CCCR (rw) register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///FDCAN CC control register
pub mod cccr;
///NBTP (rw) register accessor: an alias for `Reg<NBTP_SPEC>`
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
///FDCAN nominal bit timing and prescaler register
pub mod nbtp;
///TSCC (rw) register accessor: an alias for `Reg<TSCC_SPEC>`
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
///FDCAN timestamp counter configuration register
pub mod tscc;
///TSCV (rw) register accessor: an alias for `Reg<TSCV_SPEC>`
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
///FDCAN timestamp counter value register
pub mod tscv;
///TOCC (rw) register accessor: an alias for `Reg<TOCC_SPEC>`
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
///FDCAN timeout counter configuration register
pub mod tocc;
///TOCV (rw) register accessor: an alias for `Reg<TOCV_SPEC>`
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
///FDCAN timeout counter value register
pub mod tocv;
///ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
///FDCAN error counter register
pub mod ecr;
///PSR (rw) register accessor: an alias for `Reg<PSR_SPEC>`
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///FDCAN protocol status register
pub mod psr;
///TDCR (rw) register accessor: an alias for `Reg<TDCR_SPEC>`
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
///FDCAN transmitter delay compensation register
pub mod tdcr;
///IR (rw) register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///FDCAN interrupt register
pub mod ir;
///IE (rw) register accessor: an alias for `Reg<IE_SPEC>`
pub type IE = crate::Reg<ie::IE_SPEC>;
///FDCAN interrupt enable register
pub mod ie;
///ILS (rw) register accessor: an alias for `Reg<ILS_SPEC>`
pub type ILS = crate::Reg<ils::ILS_SPEC>;
///FDCAN interrupt line select register
pub mod ils;
///ILE (rw) register accessor: an alias for `Reg<ILE_SPEC>`
pub type ILE = crate::Reg<ile::ILE_SPEC>;
///FDCAN interrupt line enable register
pub mod ile;
///RXGFC (rw) register accessor: an alias for `Reg<RXGFC_SPEC>`
pub type RXGFC = crate::Reg<rxgfc::RXGFC_SPEC>;
///FDCAN global filter configuration register
pub mod rxgfc;
///XIDAM (rw) register accessor: an alias for `Reg<XIDAM_SPEC>`
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
///FDCAN extended ID and mask register
pub mod xidam;
///HPMS (r) register accessor: an alias for `Reg<HPMS_SPEC>`
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
///FDCAN high-priority message status register
pub mod hpms;
///RXF0S (r) register accessor: an alias for `Reg<RXF0S_SPEC>`
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
///FDCAN Rx FIFO 0 status register
pub mod rxf0s;
///RXF0A (rw) register accessor: an alias for `Reg<RXF0A_SPEC>`
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
///CAN Rx FIFO 0 acknowledge register
pub mod rxf0a;
///RXF1S (r) register accessor: an alias for `Reg<RXF1S_SPEC>`
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
///FDCAN Rx FIFO 1 status register
pub mod rxf1s;
///RXF1A (rw) register accessor: an alias for `Reg<RXF1A_SPEC>`
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
///FDCAN Rx FIFO 1 acknowledge register
pub mod rxf1a;
///TXBC (rw) register accessor: an alias for `Reg<TXBC_SPEC>`
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
///FDCAN Tx buffer configuration register
pub mod txbc;
///TXFQS (r) register accessor: an alias for `Reg<TXFQS_SPEC>`
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
///FDCAN Tx FIFO/queue status register
pub mod txfqs;
///TXBRP (r) register accessor: an alias for `Reg<TXBRP_SPEC>`
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
///FDCAN Tx buffer request pending register
pub mod txbrp;
///TXBAR (rw) register accessor: an alias for `Reg<TXBAR_SPEC>`
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
///FDCAN Tx buffer add request register
pub mod txbar;
///TXBCR (rw) register accessor: an alias for `Reg<TXBCR_SPEC>`
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
///FDCAN Tx buffer cancellation request register
pub mod txbcr;
///TXBTO (r) register accessor: an alias for `Reg<TXBTO_SPEC>`
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
///FDCAN Tx buffer transmission occurred register
pub mod txbto;
///TXBCF (r) register accessor: an alias for `Reg<TXBCF_SPEC>`
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
///FDCAN Tx buffer cancellation finished register
pub mod txbcf;
///TXBTIE (rw) register accessor: an alias for `Reg<TXBTIE_SPEC>`
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
///FDCAN Tx buffer transmission interrupt enable register
pub mod txbtie;
///TXBCIE (rw) register accessor: an alias for `Reg<TXBCIE_SPEC>`
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
///FDCAN Tx buffer cancellation finished interrupt enable register
pub mod txbcie;
///TXEFS (r) register accessor: an alias for `Reg<TXEFS_SPEC>`
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
///FDCAN Tx event FIFO status register
pub mod txefs;
///TXEFA (rw) register accessor: an alias for `Reg<TXEFA_SPEC>`
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
///FDCAN Tx event FIFO acknowledge register
pub mod txefa;
///CKDIV (rw) register accessor: an alias for `Reg<CKDIV_SPEC>`
pub type CKDIV = crate::Reg<ckdiv::CKDIV_SPEC>;
///FDCAN CFG clock divider register
pub mod ckdiv;
