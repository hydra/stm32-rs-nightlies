///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DMAMux - DMA request line multiplexer channel x control register
    pub c0cr: C0CR,
    ///0x04 - DMAMux - DMA request line multiplexer channel x control register
    pub c1cr: C1CR,
    ///0x08 - DMAMux - DMA request line multiplexer channel x control register
    pub c2cr: C2CR,
    ///0x0c - DMAMux - DMA request line multiplexer channel x control register
    pub c3cr: C3CR,
    ///0x10 - DMAMux - DMA request line multiplexer channel x control register
    pub c4cr: C4CR,
    ///0x14 - DMAMux - DMA request line multiplexer channel x control register
    pub c5cr: C5CR,
    ///0x18 - DMAMux - DMA request line multiplexer channel x control register
    pub c6cr: C6CR,
    ///0x1c - DMAMux - DMA request line multiplexer channel x control register
    pub c7cr: C7CR,
    ///0x20 - DMAMux - DMA request line multiplexer channel x control register
    pub c8cr: C8CR,
    ///0x24 - DMAMux - DMA request line multiplexer channel x control register
    pub c9cr: C9CR,
    ///0x28 - DMAMux - DMA request line multiplexer channel x control register
    pub c10cr: C10CR,
    ///0x2c - DMAMux - DMA request line multiplexer channel x control register
    pub c11cr: C11CR,
    ///0x30 - DMAMux - DMA request line multiplexer channel x control register
    pub c12cr: C12CR,
    ///0x34 - DMAMux - DMA request line multiplexer channel x control register
    pub c13cr: C13CR,
    ///0x38 - DMAMux - DMA request line multiplexer channel x control register
    pub c14cr: C14CR,
    ///0x3c - DMAMux - DMA request line multiplexer channel x control register
    pub c15cr: C15CR,
    _reserved16: [u8; 0x40],
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    pub csr: CSR,
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    pub cfr: CFR,
    _reserved18: [u8; 0x78],
    ///0x100 - DMAMux - DMA request generator channel x control register
    pub rg0cr: RG0CR,
    ///0x104 - DMAMux - DMA request generator channel x control register
    pub rg1cr: RG1CR,
    ///0x108 - DMAMux - DMA request generator channel x control register
    pub rg2cr: RG2CR,
    ///0x10c - DMAMux - DMA request generator channel x control register
    pub rg3cr: RG3CR,
    _reserved22: [u8; 0x30],
    ///0x140 - DMAMux - DMA request generator status register
    pub rgsr: RGSR,
    ///0x144 - DMAMux - DMA request generator clear flag register
    pub rgcfr: RGCFR,
}
///C0CR (rw) register accessor: an alias for `Reg<C0CR_SPEC>`
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c0cr;
///C1CR (rw) register accessor: an alias for `Reg<C1CR_SPEC>`
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c1cr;
///C2CR (rw) register accessor: an alias for `Reg<C2CR_SPEC>`
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c2cr;
///C3CR (rw) register accessor: an alias for `Reg<C3CR_SPEC>`
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c3cr;
///C4CR (rw) register accessor: an alias for `Reg<C4CR_SPEC>`
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c4cr;
///C5CR (rw) register accessor: an alias for `Reg<C5CR_SPEC>`
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c5cr;
///C6CR (rw) register accessor: an alias for `Reg<C6CR_SPEC>`
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c6cr;
///C7CR (rw) register accessor: an alias for `Reg<C7CR_SPEC>`
pub type C7CR = crate::Reg<c7cr::C7CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c7cr;
///C8CR (rw) register accessor: an alias for `Reg<C8CR_SPEC>`
pub type C8CR = crate::Reg<c8cr::C8CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c8cr;
///C9CR (rw) register accessor: an alias for `Reg<C9CR_SPEC>`
pub type C9CR = crate::Reg<c9cr::C9CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c9cr;
///C10CR (rw) register accessor: an alias for `Reg<C10CR_SPEC>`
pub type C10CR = crate::Reg<c10cr::C10CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c10cr;
///C11CR (rw) register accessor: an alias for `Reg<C11CR_SPEC>`
pub type C11CR = crate::Reg<c11cr::C11CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c11cr;
///C12CR (rw) register accessor: an alias for `Reg<C12CR_SPEC>`
pub type C12CR = crate::Reg<c12cr::C12CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c12cr;
///C13CR (rw) register accessor: an alias for `Reg<C13CR_SPEC>`
pub type C13CR = crate::Reg<c13cr::C13CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c13cr;
///C14CR (rw) register accessor: an alias for `Reg<C14CR_SPEC>`
pub type C14CR = crate::Reg<c14cr::C14CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c14cr;
///C15CR (rw) register accessor: an alias for `Reg<C15CR_SPEC>`
pub type C15CR = crate::Reg<c15cr::C15CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c15cr;
///RG0CR (rw) register accessor: an alias for `Reg<RG0CR_SPEC>`
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg0cr;
///RG1CR (rw) register accessor: an alias for `Reg<RG1CR_SPEC>`
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg1cr;
///RG2CR (rw) register accessor: an alias for `Reg<RG2CR_SPEC>`
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg2cr;
///RG3CR (rw) register accessor: an alias for `Reg<RG3CR_SPEC>`
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg3cr;
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
