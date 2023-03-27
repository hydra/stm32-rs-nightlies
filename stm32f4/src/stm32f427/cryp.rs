///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - status register
    pub sr: SR,
    ///0x08 - data input register
    pub din: DIN,
    ///0x0c - data output register
    pub dout: DOUT,
    ///0x10 - DMA control register
    pub dmacr: DMACR,
    ///0x14 - interrupt mask set/clear register
    pub imscr: IMSCR,
    ///0x18 - raw interrupt status register
    pub risr: RISR,
    ///0x1c - masked interrupt status register
    pub misr: MISR,
    ///0x20..0x40 - Cluster KEY%s, containing K?LR, K?RR
    pub key: [KEY; 4],
    ///0x40..0x50 - Cluster INIT%s, containing IV?LR, IV?RR
    pub init: [INIT; 2],
    ///0x50..0x70 - context swap register
    pub csgcmccmr: [CSGCMCCMR; 8],
    ///0x70..0x90 - context swap register
    pub csgcmr: [CSGCMR; 8],
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///DIN (rw) register accessor: an alias for `Reg<DIN_SPEC>`
pub type DIN = crate::Reg<din::DIN_SPEC>;
///data input register
pub mod din;
///DOUT (r) register accessor: an alias for `Reg<DOUT_SPEC>`
pub type DOUT = crate::Reg<dout::DOUT_SPEC>;
///data output register
pub mod dout;
///DMACR (rw) register accessor: an alias for `Reg<DMACR_SPEC>`
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
///DMA control register
pub mod dmacr;
///IMSCR (rw) register accessor: an alias for `Reg<IMSCR_SPEC>`
pub type IMSCR = crate::Reg<imscr::IMSCR_SPEC>;
///interrupt mask set/clear register
pub mod imscr;
///RISR (r) register accessor: an alias for `Reg<RISR_SPEC>`
pub type RISR = crate::Reg<risr::RISR_SPEC>;
///raw interrupt status register
pub mod risr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///masked interrupt status register
pub mod misr;
///Cluster KEY%s, containing K?LR, K?RR
pub use self::key::KEY;
///Cluster
///Cluster KEY%s, containing K?LR, K?RR
pub mod key;
///Cluster INIT%s, containing IV?LR, IV?RR
pub use self::init::INIT;
///Cluster
///Cluster INIT%s, containing IV?LR, IV?RR
pub mod init;
///CSGCMCCMR (rw) register accessor: an alias for `Reg<CSGCMCCMR_SPEC>`
pub type CSGCMCCMR = crate::Reg<csgcmccmr::CSGCMCCMR_SPEC>;
///context swap register
pub mod csgcmccmr;
///CSGCMR (rw) register accessor: an alias for `Reg<CSGCMR_SPEC>`
pub type CSGCMR = crate::Reg<csgcmr::CSGCMR_SPEC>;
///context swap register
pub mod csgcmr;
