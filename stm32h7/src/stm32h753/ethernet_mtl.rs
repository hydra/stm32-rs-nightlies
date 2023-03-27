///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Operating mode Register
    pub mtlomr: MTLOMR,
    _reserved1: [u8; 0x1c],
    ///0x20 - Interrupt status Register
    pub mtlisr: MTLISR,
    _reserved2: [u8; 0xdc],
    ///0x100 - Tx queue operating mode Register
    pub mtltx_qomr: MTLTX_QOMR,
    ///0x104 - Tx queue underflow register
    pub mtltx_qur: MTLTX_QUR,
    ///0x108 - Tx queue debug Register
    pub mtltx_qdr: MTLTX_QDR,
    _reserved5: [u8; 0x20],
    ///0x12c - Queue interrupt control status Register
    pub mtlqicsr: MTLQICSR,
    ///0x130 - Rx queue operating mode register
    pub mtlrx_qomr: MTLRX_QOMR,
    ///0x134 - Rx queue missed packet and overflow counter register
    pub mtlrx_qmpocr: MTLRX_QMPOCR,
    ///0x138 - Rx queue debug register
    pub mtlrx_qdr: MTLRX_QDR,
}
///MTLOMR (rw) register accessor: an alias for `Reg<MTLOMR_SPEC>`
pub type MTLOMR = crate::Reg<mtlomr::MTLOMR_SPEC>;
///Operating mode Register
pub mod mtlomr;
///MTLISR (r) register accessor: an alias for `Reg<MTLISR_SPEC>`
pub type MTLISR = crate::Reg<mtlisr::MTLISR_SPEC>;
///Interrupt status Register
pub mod mtlisr;
///MTLTxQOMR (rw) register accessor: an alias for `Reg<MTLTX_QOMR_SPEC>`
pub type MTLTX_QOMR = crate::Reg<mtltx_qomr::MTLTX_QOMR_SPEC>;
///Tx queue operating mode Register
pub mod mtltx_qomr;
///MTLTxQUR (r) register accessor: an alias for `Reg<MTLTX_QUR_SPEC>`
pub type MTLTX_QUR = crate::Reg<mtltx_qur::MTLTX_QUR_SPEC>;
///Tx queue underflow register
pub mod mtltx_qur;
///MTLTxQDR (r) register accessor: an alias for `Reg<MTLTX_QDR_SPEC>`
pub type MTLTX_QDR = crate::Reg<mtltx_qdr::MTLTX_QDR_SPEC>;
///Tx queue debug Register
pub mod mtltx_qdr;
///MTLQICSR (rw) register accessor: an alias for `Reg<MTLQICSR_SPEC>`
pub type MTLQICSR = crate::Reg<mtlqicsr::MTLQICSR_SPEC>;
///Queue interrupt control status Register
pub mod mtlqicsr;
///MTLRxQOMR (rw) register accessor: an alias for `Reg<MTLRX_QOMR_SPEC>`
pub type MTLRX_QOMR = crate::Reg<mtlrx_qomr::MTLRX_QOMR_SPEC>;
///Rx queue operating mode register
pub mod mtlrx_qomr;
///MTLRxQMPOCR (r) register accessor: an alias for `Reg<MTLRX_QMPOCR_SPEC>`
pub type MTLRX_QMPOCR = crate::Reg<mtlrx_qmpocr::MTLRX_QMPOCR_SPEC>;
///Rx queue missed packet and overflow counter register
pub mod mtlrx_qmpocr;
///MTLRxQDR (r) register accessor: an alias for `Reg<MTLRX_QDR_SPEC>`
pub type MTLRX_QDR = crate::Reg<mtlrx_qdr::MTLRX_QDR_SPEC>;
///Rx queue debug register
pub mod mtlrx_qdr;
