///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub spdifrx_cr: SPDIFRX_CR,
    ///0x04 - Interrupt mask register
    pub spdifrx_imr: SPDIFRX_IMR,
    ///0x08 - Status register
    pub spdifrx_sr: SPDIFRX_SR,
    ///0x0c - Interrupt flag clear register
    pub spdifrx_ifcr: SPDIFRX_IFCR,
    ///0x10 - This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:
    pub spdifrx_fmt0_dr: SPDIFRX_FMT0_DR,
    ///0x14 - Channel status register
    pub spdifrx_csr: SPDIFRX_CSR,
    ///0x18 - Debug information register
    pub spdifrx_dir: SPDIFRX_DIR,
    _reserved7: [u8; 0x03d8],
    ///0x3f4 - SPDIFRX version register
    pub spdifrx_verr: SPDIFRX_VERR,
    ///0x3f8 - SPDIFRX identification register
    pub spdifrx_ipidr: SPDIFRX_IPIDR,
    ///0x3fc - SPDIFRX size identification register
    pub spdifrx_sidr: SPDIFRX_SIDR,
}
///SPDIFRX_CR (rw) register accessor: an alias for `Reg<SPDIFRX_CR_SPEC>`
pub type SPDIFRX_CR = crate::Reg<spdifrx_cr::SPDIFRX_CR_SPEC>;
///Control register
pub mod spdifrx_cr;
///SPDIFRX_IMR (rw) register accessor: an alias for `Reg<SPDIFRX_IMR_SPEC>`
pub type SPDIFRX_IMR = crate::Reg<spdifrx_imr::SPDIFRX_IMR_SPEC>;
///Interrupt mask register
pub mod spdifrx_imr;
///SPDIFRX_SR (r) register accessor: an alias for `Reg<SPDIFRX_SR_SPEC>`
pub type SPDIFRX_SR = crate::Reg<spdifrx_sr::SPDIFRX_SR_SPEC>;
///Status register
pub mod spdifrx_sr;
///SPDIFRX_IFCR (rw) register accessor: an alias for `Reg<SPDIFRX_IFCR_SPEC>`
pub type SPDIFRX_IFCR = crate::Reg<spdifrx_ifcr::SPDIFRX_IFCR_SPEC>;
///Interrupt flag clear register
pub mod spdifrx_ifcr;
///SPDIFRX_FMT0_DR (r) register accessor: an alias for `Reg<SPDIFRX_FMT0_DR_SPEC>`
pub type SPDIFRX_FMT0_DR = crate::Reg<spdifrx_fmt0_dr::SPDIFRX_FMT0_DR_SPEC>;
///This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:
pub mod spdifrx_fmt0_dr;
///SPDIFRX_CSR (r) register accessor: an alias for `Reg<SPDIFRX_CSR_SPEC>`
pub type SPDIFRX_CSR = crate::Reg<spdifrx_csr::SPDIFRX_CSR_SPEC>;
///Channel status register
pub mod spdifrx_csr;
///SPDIFRX_DIR (r) register accessor: an alias for `Reg<SPDIFRX_DIR_SPEC>`
pub type SPDIFRX_DIR = crate::Reg<spdifrx_dir::SPDIFRX_DIR_SPEC>;
///Debug information register
pub mod spdifrx_dir;
///SPDIFRX_VERR (r) register accessor: an alias for `Reg<SPDIFRX_VERR_SPEC>`
pub type SPDIFRX_VERR = crate::Reg<spdifrx_verr::SPDIFRX_VERR_SPEC>;
///SPDIFRX version register
pub mod spdifrx_verr;
///SPDIFRX_IPIDR (r) register accessor: an alias for `Reg<SPDIFRX_IPIDR_SPEC>`
pub type SPDIFRX_IPIDR = crate::Reg<spdifrx_ipidr::SPDIFRX_IPIDR_SPEC>;
///SPDIFRX identification register
pub mod spdifrx_ipidr;
///SPDIFRX_SIDR (r) register accessor: an alias for `Reg<SPDIFRX_SIDR_SPEC>`
pub type SPDIFRX_SIDR = crate::Reg<spdifrx_sidr::SPDIFRX_SIDR_SPEC>;
///SPDIFRX size identification register
pub mod spdifrx_sidr;
