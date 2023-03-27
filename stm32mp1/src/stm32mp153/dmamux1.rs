///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DMAMUX request line multiplexer channel 0 configuration register
    pub dmamux_c0cr: DMAMUX_C0CR,
    ///0x04 - DMAMUX request line multiplexer channel 1 configuration register
    pub dmamux_c1cr: DMAMUX_C1CR,
    ///0x08 - DMAMUX request line multiplexer channel 2 configuration register
    pub dmamux_c2cr: DMAMUX_C2CR,
    ///0x0c - DMAMUX request line multiplexer channel 3 configuration register
    pub dmamux_c3cr: DMAMUX_C3CR,
    ///0x10 - DMAMUX request line multiplexer channel 4 configuration register
    pub dmamux_c4cr: DMAMUX_C4CR,
    ///0x14 - DMAMUX request line multiplexer channel 5 configuration register
    pub dmamux_c5cr: DMAMUX_C5CR,
    ///0x18 - DMAMUX request line multiplexer channel 6 configuration register
    pub dmamux_c6cr: DMAMUX_C6CR,
    ///0x1c - DMAMUX request line multiplexer channel 7 configuration register
    pub dmamux_c7cr: DMAMUX_C7CR,
    ///0x20 - DMAMUX request line multiplexer channel 8 configuration register
    pub dmamux_c8cr: DMAMUX_C8CR,
    ///0x24 - DMAMUX request line multiplexer channel 9 configuration register
    pub dmamux_c9cr: DMAMUX_C9CR,
    ///0x28 - DMAMUX request line multiplexer channel 10 configuration register
    pub dmamux_c10cr: DMAMUX_C10CR,
    ///0x2c - DMAMUX request line multiplexer channel 11 configuration register
    pub dmamux_c11cr: DMAMUX_C11CR,
    ///0x30 - DMAMUX request line multiplexer channel 12 configuration register
    pub dmamux_c12cr: DMAMUX_C12CR,
    ///0x34 - DMAMUX request line multiplexer channel 13 configuration register
    pub dmamux_c13cr: DMAMUX_C13CR,
    ///0x38 - DMAMUX request line multiplexer channel 14 configuration register
    pub dmamux_c14cr: DMAMUX_C14CR,
    ///0x3c - DMAMUX request line multiplexer channel 15 configuration register
    pub dmamux_c15cr: DMAMUX_C15CR,
    _reserved16: [u8; 0x40],
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    pub dmamux_csr: DMAMUX_CSR,
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    pub dmamux_cfr: DMAMUX_CFR,
    _reserved18: [u8; 0x78],
    ///0x100 - DMAMUX request generator channel 0 configuration register
    pub dmamux_rg0cr: DMAMUX_RG0CR,
    ///0x104 - DMAMUX request generator channel 1 configuration register
    pub dmamux_rg1cr: DMAMUX_RG1CR,
    ///0x108 - DMAMUX request generator channel 2 configuration register
    pub dmamux_rg2cr: DMAMUX_RG2CR,
    ///0x10c - DMAMUX request generator channel 3 configuration register
    pub dmamux_rg3cr: DMAMUX_RG3CR,
    ///0x110 - DMAMUX request generator channel 4 configuration register
    pub dmamux_rg4cr: DMAMUX_RG4CR,
    ///0x114 - DMAMUX request generator channel 5 configuration register
    pub dmamux_rg5cr: DMAMUX_RG5CR,
    ///0x118 - DMAMUX request generator channel 6 configuration register
    pub dmamux_rg6cr: DMAMUX_RG6CR,
    ///0x11c - DMAMUX request generator channel 7 configuration register
    pub dmamux_rg7cr: DMAMUX_RG7CR,
    _reserved26: [u8; 0x20],
    ///0x140 - DMAMUX request generator interrupt status register
    pub dmamux_rgsr: DMAMUX_RGSR,
    ///0x144 - DMAMUX request generator interrupt clear flag register
    pub dmamux_rgcfr: DMAMUX_RGCFR,
    _reserved28: [u8; 0x02a4],
    ///0x3ec - DMAMUX hardware configuration 2 register
    pub dmamux_hwcfgr2: DMAMUX_HWCFGR2,
    ///0x3f0 - DMAMUX hardware configuration 1 register
    pub dmamux_hwcfgr1: DMAMUX_HWCFGR1,
    ///0x3f4 - This register identifies the IP version.
    pub dmamux_verr: DMAMUX_VERR,
    ///0x3f8 - This register identifies the IP.
    pub dmamux_ipidr: DMAMUX_IPIDR,
    ///0x3fc - DMAMUX size identification register
    pub dmamux_sidr: DMAMUX_SIDR,
}
///DMAMUX_C0CR (rw) register accessor: an alias for `Reg<DMAMUX_C0CR_SPEC>`
pub type DMAMUX_C0CR = crate::Reg<dmamux_c0cr::DMAMUX_C0CR_SPEC>;
///DMAMUX request line multiplexer channel 0 configuration register
pub mod dmamux_c0cr;
///DMAMUX_C1CR (rw) register accessor: an alias for `Reg<DMAMUX_C1CR_SPEC>`
pub type DMAMUX_C1CR = crate::Reg<dmamux_c1cr::DMAMUX_C1CR_SPEC>;
///DMAMUX request line multiplexer channel 1 configuration register
pub mod dmamux_c1cr;
///DMAMUX_C2CR (rw) register accessor: an alias for `Reg<DMAMUX_C2CR_SPEC>`
pub type DMAMUX_C2CR = crate::Reg<dmamux_c2cr::DMAMUX_C2CR_SPEC>;
///DMAMUX request line multiplexer channel 2 configuration register
pub mod dmamux_c2cr;
///DMAMUX_C3CR (rw) register accessor: an alias for `Reg<DMAMUX_C3CR_SPEC>`
pub type DMAMUX_C3CR = crate::Reg<dmamux_c3cr::DMAMUX_C3CR_SPEC>;
///DMAMUX request line multiplexer channel 3 configuration register
pub mod dmamux_c3cr;
///DMAMUX_C4CR (rw) register accessor: an alias for `Reg<DMAMUX_C4CR_SPEC>`
pub type DMAMUX_C4CR = crate::Reg<dmamux_c4cr::DMAMUX_C4CR_SPEC>;
///DMAMUX request line multiplexer channel 4 configuration register
pub mod dmamux_c4cr;
///DMAMUX_C5CR (rw) register accessor: an alias for `Reg<DMAMUX_C5CR_SPEC>`
pub type DMAMUX_C5CR = crate::Reg<dmamux_c5cr::DMAMUX_C5CR_SPEC>;
///DMAMUX request line multiplexer channel 5 configuration register
pub mod dmamux_c5cr;
///DMAMUX_C6CR (rw) register accessor: an alias for `Reg<DMAMUX_C6CR_SPEC>`
pub type DMAMUX_C6CR = crate::Reg<dmamux_c6cr::DMAMUX_C6CR_SPEC>;
///DMAMUX request line multiplexer channel 6 configuration register
pub mod dmamux_c6cr;
///DMAMUX_C7CR (rw) register accessor: an alias for `Reg<DMAMUX_C7CR_SPEC>`
pub type DMAMUX_C7CR = crate::Reg<dmamux_c7cr::DMAMUX_C7CR_SPEC>;
///DMAMUX request line multiplexer channel 7 configuration register
pub mod dmamux_c7cr;
///DMAMUX_C8CR (rw) register accessor: an alias for `Reg<DMAMUX_C8CR_SPEC>`
pub type DMAMUX_C8CR = crate::Reg<dmamux_c8cr::DMAMUX_C8CR_SPEC>;
///DMAMUX request line multiplexer channel 8 configuration register
pub mod dmamux_c8cr;
///DMAMUX_C9CR (rw) register accessor: an alias for `Reg<DMAMUX_C9CR_SPEC>`
pub type DMAMUX_C9CR = crate::Reg<dmamux_c9cr::DMAMUX_C9CR_SPEC>;
///DMAMUX request line multiplexer channel 9 configuration register
pub mod dmamux_c9cr;
///DMAMUX_C10CR (rw) register accessor: an alias for `Reg<DMAMUX_C10CR_SPEC>`
pub type DMAMUX_C10CR = crate::Reg<dmamux_c10cr::DMAMUX_C10CR_SPEC>;
///DMAMUX request line multiplexer channel 10 configuration register
pub mod dmamux_c10cr;
///DMAMUX_C11CR (rw) register accessor: an alias for `Reg<DMAMUX_C11CR_SPEC>`
pub type DMAMUX_C11CR = crate::Reg<dmamux_c11cr::DMAMUX_C11CR_SPEC>;
///DMAMUX request line multiplexer channel 11 configuration register
pub mod dmamux_c11cr;
///DMAMUX_C12CR (rw) register accessor: an alias for `Reg<DMAMUX_C12CR_SPEC>`
pub type DMAMUX_C12CR = crate::Reg<dmamux_c12cr::DMAMUX_C12CR_SPEC>;
///DMAMUX request line multiplexer channel 12 configuration register
pub mod dmamux_c12cr;
///DMAMUX_C13CR (rw) register accessor: an alias for `Reg<DMAMUX_C13CR_SPEC>`
pub type DMAMUX_C13CR = crate::Reg<dmamux_c13cr::DMAMUX_C13CR_SPEC>;
///DMAMUX request line multiplexer channel 13 configuration register
pub mod dmamux_c13cr;
///DMAMUX_C14CR (rw) register accessor: an alias for `Reg<DMAMUX_C14CR_SPEC>`
pub type DMAMUX_C14CR = crate::Reg<dmamux_c14cr::DMAMUX_C14CR_SPEC>;
///DMAMUX request line multiplexer channel 14 configuration register
pub mod dmamux_c14cr;
///DMAMUX_C15CR (rw) register accessor: an alias for `Reg<DMAMUX_C15CR_SPEC>`
pub type DMAMUX_C15CR = crate::Reg<dmamux_c15cr::DMAMUX_C15CR_SPEC>;
///DMAMUX request line multiplexer channel 15 configuration register
pub mod dmamux_c15cr;
///DMAMUX_CSR (r) register accessor: an alias for `Reg<DMAMUX_CSR_SPEC>`
pub type DMAMUX_CSR = crate::Reg<dmamux_csr::DMAMUX_CSR_SPEC>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod dmamux_csr;
///DMAMUX_CFR (w) register accessor: an alias for `Reg<DMAMUX_CFR_SPEC>`
pub type DMAMUX_CFR = crate::Reg<dmamux_cfr::DMAMUX_CFR_SPEC>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod dmamux_cfr;
///DMAMUX_RG0CR (rw) register accessor: an alias for `Reg<DMAMUX_RG0CR_SPEC>`
pub type DMAMUX_RG0CR = crate::Reg<dmamux_rg0cr::DMAMUX_RG0CR_SPEC>;
///DMAMUX request generator channel 0 configuration register
pub mod dmamux_rg0cr;
///DMAMUX_RG1CR (rw) register accessor: an alias for `Reg<DMAMUX_RG1CR_SPEC>`
pub type DMAMUX_RG1CR = crate::Reg<dmamux_rg1cr::DMAMUX_RG1CR_SPEC>;
///DMAMUX request generator channel 1 configuration register
pub mod dmamux_rg1cr;
///DMAMUX_RG2CR (rw) register accessor: an alias for `Reg<DMAMUX_RG2CR_SPEC>`
pub type DMAMUX_RG2CR = crate::Reg<dmamux_rg2cr::DMAMUX_RG2CR_SPEC>;
///DMAMUX request generator channel 2 configuration register
pub mod dmamux_rg2cr;
///DMAMUX_RG3CR (rw) register accessor: an alias for `Reg<DMAMUX_RG3CR_SPEC>`
pub type DMAMUX_RG3CR = crate::Reg<dmamux_rg3cr::DMAMUX_RG3CR_SPEC>;
///DMAMUX request generator channel 3 configuration register
pub mod dmamux_rg3cr;
///DMAMUX_RG4CR (rw) register accessor: an alias for `Reg<DMAMUX_RG4CR_SPEC>`
pub type DMAMUX_RG4CR = crate::Reg<dmamux_rg4cr::DMAMUX_RG4CR_SPEC>;
///DMAMUX request generator channel 4 configuration register
pub mod dmamux_rg4cr;
///DMAMUX_RG5CR (rw) register accessor: an alias for `Reg<DMAMUX_RG5CR_SPEC>`
pub type DMAMUX_RG5CR = crate::Reg<dmamux_rg5cr::DMAMUX_RG5CR_SPEC>;
///DMAMUX request generator channel 5 configuration register
pub mod dmamux_rg5cr;
///DMAMUX_RG6CR (rw) register accessor: an alias for `Reg<DMAMUX_RG6CR_SPEC>`
pub type DMAMUX_RG6CR = crate::Reg<dmamux_rg6cr::DMAMUX_RG6CR_SPEC>;
///DMAMUX request generator channel 6 configuration register
pub mod dmamux_rg6cr;
///DMAMUX_RG7CR (rw) register accessor: an alias for `Reg<DMAMUX_RG7CR_SPEC>`
pub type DMAMUX_RG7CR = crate::Reg<dmamux_rg7cr::DMAMUX_RG7CR_SPEC>;
///DMAMUX request generator channel 7 configuration register
pub mod dmamux_rg7cr;
///DMAMUX_RGSR (r) register accessor: an alias for `Reg<DMAMUX_RGSR_SPEC>`
pub type DMAMUX_RGSR = crate::Reg<dmamux_rgsr::DMAMUX_RGSR_SPEC>;
///DMAMUX request generator interrupt status register
pub mod dmamux_rgsr;
///DMAMUX_RGCFR (w) register accessor: an alias for `Reg<DMAMUX_RGCFR_SPEC>`
pub type DMAMUX_RGCFR = crate::Reg<dmamux_rgcfr::DMAMUX_RGCFR_SPEC>;
///DMAMUX request generator interrupt clear flag register
pub mod dmamux_rgcfr;
///DMAMUX_HWCFGR2 (r) register accessor: an alias for `Reg<DMAMUX_HWCFGR2_SPEC>`
pub type DMAMUX_HWCFGR2 = crate::Reg<dmamux_hwcfgr2::DMAMUX_HWCFGR2_SPEC>;
///DMAMUX hardware configuration 2 register
pub mod dmamux_hwcfgr2;
///DMAMUX_HWCFGR1 (r) register accessor: an alias for `Reg<DMAMUX_HWCFGR1_SPEC>`
pub type DMAMUX_HWCFGR1 = crate::Reg<dmamux_hwcfgr1::DMAMUX_HWCFGR1_SPEC>;
///DMAMUX hardware configuration 1 register
pub mod dmamux_hwcfgr1;
///DMAMUX_VERR (r) register accessor: an alias for `Reg<DMAMUX_VERR_SPEC>`
pub type DMAMUX_VERR = crate::Reg<dmamux_verr::DMAMUX_VERR_SPEC>;
///This register identifies the IP version.
pub mod dmamux_verr;
///DMAMUX_IPIDR (r) register accessor: an alias for `Reg<DMAMUX_IPIDR_SPEC>`
pub type DMAMUX_IPIDR = crate::Reg<dmamux_ipidr::DMAMUX_IPIDR_SPEC>;
///This register identifies the IP.
pub mod dmamux_ipidr;
///DMAMUX_SIDR (r) register accessor: an alias for `Reg<DMAMUX_SIDR_SPEC>`
pub type DMAMUX_SIDR = crate::Reg<dmamux_sidr::DMAMUX_SIDR_SPEC>;
///DMAMUX size identification register
pub mod dmamux_sidr;
