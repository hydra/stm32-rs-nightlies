///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x40 - DMAMux - DMA request line multiplexer channel x control register
    pub ccr: [CCR; 16],
    _reserved1: [u8; 0x40],
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    pub csr: CSR,
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    pub cfr: CFR,
    _reserved3: [u8; 0x78],
    ///0x100..0x120 - DMAMux - DMA request generator channel x control register
    pub rgcr: [RGCR; 8],
    _reserved4: [u8; 0x20],
    ///0x140 - DMAMux - DMA request generator status register
    pub rgsr: RGSR,
    ///0x144 - DMAMux - DMA request generator clear flag register
    pub rgcfr: RGCFR,
}
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod ccr;
///RGCR (rw) register accessor: an alias for `Reg<RGCR_SPEC>`
pub type RGCR = crate::Reg<rgcr::RGCR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rgcr;
///RGSR (r) register accessor: an alias for `Reg<RGSR_SPEC>`
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
///DMAMux - DMA request generator status register
pub mod rgsr;
///RGCFR (w) register accessor: an alias for `Reg<RGCFR_SPEC>`
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
///DMAMux - DMA request generator clear flag register
pub mod rgcfr;
///CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
///CFR (w) register accessor: an alias for `Reg<CFR_SPEC>`
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
