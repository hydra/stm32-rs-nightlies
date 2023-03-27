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
    _reserved7: [u8; 0xe4],
    ///0x100 - DMAMux - DMA request generator channel x control register
    pub rg0cr: RG0CR,
    ///0x104 - DMAMux - DMA request generator channel x control register
    pub rg1cr: RG1CR,
    ///0x108 - DMAMux - DMA request generator channel x control register
    pub rg2cr: RG2CR,
    ///0x10c - DMAMux - DMA request generator channel x control register
    pub rg3cr: RG3CR,
    _reserved11: [u8; 0x30],
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
