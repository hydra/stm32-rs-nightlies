///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub cr: CR,
    ///0x04 - Interrupt mask register
    pub imr: IMR,
    ///0x08 - Status register
    pub sr: SR,
    ///0x0c - Interrupt Flag Clear register
    pub ifcr: IFCR,
    ///0x10 - Data input register
    pub fmt0_dr: FMT0_DR,
    ///0x14 - Channel Status register
    pub csr: CSR,
    ///0x18 - Debug Information register
    pub dir: DIR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`
pub type IMR = crate::Reg<imr::IMR_SPEC>;
///Interrupt mask register
pub mod imr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///Interrupt Flag Clear register
pub mod ifcr;
///FMT0_DR (r) register accessor: an alias for `Reg<FMT0_DR_SPEC>`
pub type FMT0_DR = crate::Reg<fmt0_dr::FMT0_DR_SPEC>;
///Data input register
pub mod fmt0_dr;
///CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Channel Status register
pub mod csr;
///DIR (r) register accessor: an alias for `Reg<DIR_SPEC>`
pub type DIR = crate::Reg<dir::DIR_SPEC>;
///Debug Information register
pub mod dir;
