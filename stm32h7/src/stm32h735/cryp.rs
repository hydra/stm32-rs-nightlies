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
    ///0x20 - key registers
    pub k0lr: K0LR,
    ///0x24 - key registers
    pub k0rr: K0RR,
    ///0x28 - key registers
    pub k1lr: K1LR,
    ///0x2c - key registers
    pub k1rr: K1RR,
    ///0x30 - key registers
    pub k2lr: K2LR,
    ///0x34 - key registers
    pub k2rr: K2RR,
    ///0x38 - key registers
    pub k3lr: K3LR,
    ///0x3c - key registers
    pub k3rr: K3RR,
    ///0x40 - Initialization vector register 0L
    pub iv0lr: IV0LR,
    ///0x44 - initialization vector register 0R
    pub iv0rr: IV0RR,
    ///0x48 - Initialization vector register 1L
    pub iv1lr: IV1LR,
    ///0x4c - Initialization vector register 1R
    pub iv1rr: IV1RR,
    ///0x50 - context swap register
    pub csgcmccm0r: CSGCMCCM0R,
    ///0x54 - context swap register
    pub csgcmccm1r: CSGCMCCM1R,
    ///0x58 - context swap register
    pub csgcmccm2r: CSGCMCCM2R,
    ///0x5c - context swap register
    pub csgcmccm3r: CSGCMCCM3R,
    ///0x60 - context swap register
    pub csgcmccm4r: CSGCMCCM4R,
    ///0x64 - context swap register
    pub csgcmccm5r: CSGCMCCM5R,
    ///0x68 - context swap register
    pub csgcmccm6r: CSGCMCCM6R,
    ///0x6c - context swap register
    pub csgcmccm7r: CSGCMCCM7R,
    ///0x70 - context swap register
    pub csgcm0r: CSGCM0R,
    ///0x74 - context swap register
    pub csgcm1r: CSGCM1R,
    ///0x78 - context swap register
    pub csgcm2r: CSGCM2R,
    ///0x7c - context swap register
    pub csgcm3r: CSGCM3R,
    ///0x80 - context swap register
    pub csgcm4r: CSGCM4R,
    ///0x84 - context swap register
    pub csgcm5r: CSGCM5R,
    ///0x88 - context swap register
    pub csgcm6r: CSGCM6R,
    ///0x8c - context swap register
    pub csgcm7r: CSGCM7R,
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
///K0LR (w) register accessor: an alias for `Reg<K0LR_SPEC>`
pub type K0LR = crate::Reg<k0lr::K0LR_SPEC>;
///key registers
pub mod k0lr;
///K0RR (w) register accessor: an alias for `Reg<K0RR_SPEC>`
pub type K0RR = crate::Reg<k0rr::K0RR_SPEC>;
///key registers
pub mod k0rr;
///K1LR (w) register accessor: an alias for `Reg<K1LR_SPEC>`
pub type K1LR = crate::Reg<k1lr::K1LR_SPEC>;
///key registers
pub mod k1lr;
///K1RR (w) register accessor: an alias for `Reg<K1RR_SPEC>`
pub type K1RR = crate::Reg<k1rr::K1RR_SPEC>;
///key registers
pub mod k1rr;
///K2LR (w) register accessor: an alias for `Reg<K2LR_SPEC>`
pub type K2LR = crate::Reg<k2lr::K2LR_SPEC>;
///key registers
pub mod k2lr;
///K2RR (w) register accessor: an alias for `Reg<K2RR_SPEC>`
pub type K2RR = crate::Reg<k2rr::K2RR_SPEC>;
///key registers
pub mod k2rr;
///K3LR (w) register accessor: an alias for `Reg<K3LR_SPEC>`
pub type K3LR = crate::Reg<k3lr::K3LR_SPEC>;
///key registers
pub mod k3lr;
///K3RR (w) register accessor: an alias for `Reg<K3RR_SPEC>`
pub type K3RR = crate::Reg<k3rr::K3RR_SPEC>;
///key registers
pub mod k3rr;
///IV0LR (rw) register accessor: an alias for `Reg<IV0LR_SPEC>`
pub type IV0LR = crate::Reg<iv0lr::IV0LR_SPEC>;
///Initialization vector register 0L
pub mod iv0lr;
///IV0RR (rw) register accessor: an alias for `Reg<IV0RR_SPEC>`
pub type IV0RR = crate::Reg<iv0rr::IV0RR_SPEC>;
///initialization vector register 0R
pub mod iv0rr;
///IV1LR (rw) register accessor: an alias for `Reg<IV1LR_SPEC>`
pub type IV1LR = crate::Reg<iv1lr::IV1LR_SPEC>;
///Initialization vector register 1L
pub mod iv1lr;
///IV1RR (rw) register accessor: an alias for `Reg<IV1RR_SPEC>`
pub type IV1RR = crate::Reg<iv1rr::IV1RR_SPEC>;
///Initialization vector register 1R
pub mod iv1rr;
///CSGCMCCM0R (rw) register accessor: an alias for `Reg<CSGCMCCM0R_SPEC>`
pub type CSGCMCCM0R = crate::Reg<csgcmccm0r::CSGCMCCM0R_SPEC>;
///context swap register
pub mod csgcmccm0r;
///CSGCMCCM1R (rw) register accessor: an alias for `Reg<CSGCMCCM1R_SPEC>`
pub type CSGCMCCM1R = crate::Reg<csgcmccm1r::CSGCMCCM1R_SPEC>;
///context swap register
pub mod csgcmccm1r;
///CSGCMCCM2R (rw) register accessor: an alias for `Reg<CSGCMCCM2R_SPEC>`
pub type CSGCMCCM2R = crate::Reg<csgcmccm2r::CSGCMCCM2R_SPEC>;
///context swap register
pub mod csgcmccm2r;
///CSGCMCCM3R (rw) register accessor: an alias for `Reg<CSGCMCCM3R_SPEC>`
pub type CSGCMCCM3R = crate::Reg<csgcmccm3r::CSGCMCCM3R_SPEC>;
///context swap register
pub mod csgcmccm3r;
///CSGCMCCM4R (rw) register accessor: an alias for `Reg<CSGCMCCM4R_SPEC>`
pub type CSGCMCCM4R = crate::Reg<csgcmccm4r::CSGCMCCM4R_SPEC>;
///context swap register
pub mod csgcmccm4r;
///CSGCMCCM5R (rw) register accessor: an alias for `Reg<CSGCMCCM5R_SPEC>`
pub type CSGCMCCM5R = crate::Reg<csgcmccm5r::CSGCMCCM5R_SPEC>;
///context swap register
pub mod csgcmccm5r;
///CSGCMCCM6R (rw) register accessor: an alias for `Reg<CSGCMCCM6R_SPEC>`
pub type CSGCMCCM6R = crate::Reg<csgcmccm6r::CSGCMCCM6R_SPEC>;
///context swap register
pub mod csgcmccm6r;
///CSGCMCCM7R (rw) register accessor: an alias for `Reg<CSGCMCCM7R_SPEC>`
pub type CSGCMCCM7R = crate::Reg<csgcmccm7r::CSGCMCCM7R_SPEC>;
///context swap register
pub mod csgcmccm7r;
///CSGCM0R (rw) register accessor: an alias for `Reg<CSGCM0R_SPEC>`
pub type CSGCM0R = crate::Reg<csgcm0r::CSGCM0R_SPEC>;
///context swap register
pub mod csgcm0r;
///CSGCM1R (rw) register accessor: an alias for `Reg<CSGCM1R_SPEC>`
pub type CSGCM1R = crate::Reg<csgcm1r::CSGCM1R_SPEC>;
///context swap register
pub mod csgcm1r;
///CSGCM2R (rw) register accessor: an alias for `Reg<CSGCM2R_SPEC>`
pub type CSGCM2R = crate::Reg<csgcm2r::CSGCM2R_SPEC>;
///context swap register
pub mod csgcm2r;
///CSGCM3R (rw) register accessor: an alias for `Reg<CSGCM3R_SPEC>`
pub type CSGCM3R = crate::Reg<csgcm3r::CSGCM3R_SPEC>;
///context swap register
pub mod csgcm3r;
///CSGCM4R (rw) register accessor: an alias for `Reg<CSGCM4R_SPEC>`
pub type CSGCM4R = crate::Reg<csgcm4r::CSGCM4R_SPEC>;
///context swap register
pub mod csgcm4r;
///CSGCM5R (rw) register accessor: an alias for `Reg<CSGCM5R_SPEC>`
pub type CSGCM5R = crate::Reg<csgcm5r::CSGCM5R_SPEC>;
///context swap register
pub mod csgcm5r;
///CSGCM6R (rw) register accessor: an alias for `Reg<CSGCM6R_SPEC>`
pub type CSGCM6R = crate::Reg<csgcm6r::CSGCM6R_SPEC>;
///context swap register
pub mod csgcm6r;
///CSGCM7R (rw) register accessor: an alias for `Reg<CSGCM7R_SPEC>`
pub type CSGCM7R = crate::Reg<csgcm7r::CSGCM7R_SPEC>;
///context swap register
pub mod csgcm7r;
