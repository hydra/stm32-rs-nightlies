///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - LPDMA secure configuration register
    pub lpdma_seccfgr: LPDMA_SECCFGR,
    ///0x04 - LPDMA privileged configuration register
    pub lpdma_privcfgr: LPDMA_PRIVCFGR,
    ///0x08 - LPDMA configuration lock register
    pub lpdma_rcfglockr: LPDMA_RCFGLOCKR,
    ///0x0c - LPDMA non-secure masked interrupt status register
    pub lpdma_misr: LPDMA_MISR,
    ///0x10 - LPDMA secure masked interrupt status register
    pub lpdma_smisr: LPDMA_SMISR,
    _reserved5: [u8; 0x3c],
    ///0x50 - LPDMA channel 0 linked-list base address register
    pub lpdma_c0lbar: LPDMA_C0LBAR,
    _reserved6: [u8; 0x08],
    ///0x5c - LPDMA channel 0 flag clear register
    pub lpdma_c0fcr: LPDMA_C0FCR,
    ///0x60 - LPDMA channel 0 status register
    pub lpdma_c0sr: LPDMA_C0SR,
    ///0x64 - LPDMA channel 0 control register
    pub lpdma_c0cr: LPDMA_C0CR,
    _reserved9: [u8; 0x28],
    ///0x90 - LPDMA channel 0 transfer register 1
    pub lpdma_c0tr1: LPDMA_C0TR1,
    ///0x94 - LPDMA channel 0 transfer register 2
    pub lpdma_c0tr2: LPDMA_C0TR2,
    ///0x98 - LPDMA channel 0 block register 1
    pub lpdma_c0br1: LPDMA_C0BR1,
    ///0x9c - LPDMA channel 0 source address register
    pub lpdma_c0sar: LPDMA_C0SAR,
    ///0xa0 - LPDMA channel 0 destination address register
    pub lpdma_c0dar: LPDMA_C0DAR,
    _reserved14: [u8; 0x28],
    ///0xcc - LPDMA channel 0 linked-list address register
    pub lpdma_c0llr: LPDMA_C0LLR,
    ///0xd0 - LPDMA channel 1 linked-list base address register
    pub lpdma_c1lbar: LPDMA_C1LBAR,
    _reserved16: [u8; 0x08],
    ///0xdc - LPDMA channel 1 flag clear register
    pub lpdma_c1fcr: LPDMA_C1FCR,
    ///0xe0 - LPDMA channel 1 status register
    pub lpdma_c1sr: LPDMA_C1SR,
    ///0xe4 - LPDMA channel 1 control register
    pub lpdma_c1cr: LPDMA_C1CR,
    _reserved19: [u8; 0x28],
    ///0x110 - LPDMA channel 1 transfer register 1
    pub lpdma_c1tr1: LPDMA_C1TR1,
    ///0x114 - LPDMA channel 1 transfer register 2
    pub lpdma_c1tr2: LPDMA_C1TR2,
    ///0x118 - LPDMA channel 1 block register 1
    pub lpdma_c1br1: LPDMA_C1BR1,
    ///0x11c - LPDMA channel 1 source address register
    pub lpdma_c1sar: LPDMA_C1SAR,
    ///0x120 - LPDMA channel 1 destination address register
    pub lpdma_c1dar: LPDMA_C1DAR,
    _reserved24: [u8; 0x28],
    ///0x14c - LPDMA channel 1 linked-list address register
    pub lpdma_c1llr: LPDMA_C1LLR,
    ///0x150 - LPDMA channel 2 linked-list base address register
    pub lpdma_c2lbar: LPDMA_C2LBAR,
    _reserved26: [u8; 0x08],
    ///0x15c - LPDMA channel 2 flag clear register
    pub lpdma_c2fcr: LPDMA_C2FCR,
    ///0x160 - LPDMA channel 2 status register
    pub lpdma_c2sr: LPDMA_C2SR,
    ///0x164 - LPDMA channel 2 control register
    pub lpdma_c2cr: LPDMA_C2CR,
    _reserved29: [u8; 0x28],
    ///0x190 - LPDMA channel 2 transfer register 1
    pub lpdma_c2tr1: LPDMA_C2TR1,
    ///0x194 - LPDMA channel 2 transfer register 2
    pub lpdma_c2tr2: LPDMA_C2TR2,
    ///0x198 - LPDMA channel 2 block register 1
    pub lpdma_c2br1: LPDMA_C2BR1,
    ///0x19c - LPDMA channel 2 source address register
    pub lpdma_c2sar: LPDMA_C2SAR,
    ///0x1a0 - LPDMA channel 2 destination address register
    pub lpdma_c2dar: LPDMA_C2DAR,
    _reserved34: [u8; 0x28],
    ///0x1cc - LPDMA channel 2 linked-list address register
    pub lpdma_c2llr: LPDMA_C2LLR,
    ///0x1d0 - LPDMA channel 3 linked-list base address register
    pub lpdma_c3lbar: LPDMA_C3LBAR,
    _reserved36: [u8; 0x08],
    ///0x1dc - LPDMA channel 3 flag clear register
    pub lpdma_c3fcr: LPDMA_C3FCR,
    ///0x1e0 - LPDMA channel 3 status register
    pub lpdma_c3sr: LPDMA_C3SR,
    ///0x1e4 - LPDMA channel 3 control register
    pub lpdma_c3cr: LPDMA_C3CR,
    _reserved39: [u8; 0x28],
    ///0x210 - LPDMA channel 3 transfer register 1
    pub lpdma_c3tr1: LPDMA_C3TR1,
    ///0x214 - LPDMA channel 3 transfer register 2
    pub lpdma_c3tr2: LPDMA_C3TR2,
    ///0x218 - LPDMA channel 3 block register 1
    pub lpdma_c3br1: LPDMA_C3BR1,
    ///0x21c - LPDMA channel 3 source address register
    pub lpdma_c3sar: LPDMA_C3SAR,
    ///0x220 - LPDMA channel 3 destination address register
    pub lpdma_c3dar: LPDMA_C3DAR,
    _reserved44: [u8; 0x28],
    ///0x24c - LPDMA channel 3 linked-list address register
    pub lpdma_c3llr: LPDMA_C3LLR,
}
///LPDMA_SECCFGR (rw) register accessor: an alias for `Reg<LPDMA_SECCFGR_SPEC>`
pub type LPDMA_SECCFGR = crate::Reg<lpdma_seccfgr::LPDMA_SECCFGR_SPEC>;
///LPDMA secure configuration register
pub mod lpdma_seccfgr;
///LPDMA_PRIVCFGR (rw) register accessor: an alias for `Reg<LPDMA_PRIVCFGR_SPEC>`
pub type LPDMA_PRIVCFGR = crate::Reg<lpdma_privcfgr::LPDMA_PRIVCFGR_SPEC>;
///LPDMA privileged configuration register
pub mod lpdma_privcfgr;
///LPDMA_RCFGLOCKR (rw) register accessor: an alias for `Reg<LPDMA_RCFGLOCKR_SPEC>`
pub type LPDMA_RCFGLOCKR = crate::Reg<lpdma_rcfglockr::LPDMA_RCFGLOCKR_SPEC>;
///LPDMA configuration lock register
pub mod lpdma_rcfglockr;
///LPDMA_MISR (r) register accessor: an alias for `Reg<LPDMA_MISR_SPEC>`
pub type LPDMA_MISR = crate::Reg<lpdma_misr::LPDMA_MISR_SPEC>;
///LPDMA non-secure masked interrupt status register
pub mod lpdma_misr;
///LPDMA_SMISR (r) register accessor: an alias for `Reg<LPDMA_SMISR_SPEC>`
pub type LPDMA_SMISR = crate::Reg<lpdma_smisr::LPDMA_SMISR_SPEC>;
///LPDMA secure masked interrupt status register
pub mod lpdma_smisr;
///LPDMA_C0LBAR (rw) register accessor: an alias for `Reg<LPDMA_C0LBAR_SPEC>`
pub type LPDMA_C0LBAR = crate::Reg<lpdma_c0lbar::LPDMA_C0LBAR_SPEC>;
///LPDMA channel 0 linked-list base address register
pub mod lpdma_c0lbar;
///LPDMA_C0FCR (w) register accessor: an alias for `Reg<LPDMA_C0FCR_SPEC>`
pub type LPDMA_C0FCR = crate::Reg<lpdma_c0fcr::LPDMA_C0FCR_SPEC>;
///LPDMA channel 0 flag clear register
pub mod lpdma_c0fcr;
///LPDMA_C0SR (r) register accessor: an alias for `Reg<LPDMA_C0SR_SPEC>`
pub type LPDMA_C0SR = crate::Reg<lpdma_c0sr::LPDMA_C0SR_SPEC>;
///LPDMA channel 0 status register
pub mod lpdma_c0sr;
///LPDMA_C0CR (rw) register accessor: an alias for `Reg<LPDMA_C0CR_SPEC>`
pub type LPDMA_C0CR = crate::Reg<lpdma_c0cr::LPDMA_C0CR_SPEC>;
///LPDMA channel 0 control register
pub mod lpdma_c0cr;
///LPDMA_C0TR1 (rw) register accessor: an alias for `Reg<LPDMA_C0TR1_SPEC>`
pub type LPDMA_C0TR1 = crate::Reg<lpdma_c0tr1::LPDMA_C0TR1_SPEC>;
///LPDMA channel 0 transfer register 1
pub mod lpdma_c0tr1;
///LPDMA_C0TR2 (rw) register accessor: an alias for `Reg<LPDMA_C0TR2_SPEC>`
pub type LPDMA_C0TR2 = crate::Reg<lpdma_c0tr2::LPDMA_C0TR2_SPEC>;
///LPDMA channel 0 transfer register 2
pub mod lpdma_c0tr2;
///LPDMA_C0BR1 (rw) register accessor: an alias for `Reg<LPDMA_C0BR1_SPEC>`
pub type LPDMA_C0BR1 = crate::Reg<lpdma_c0br1::LPDMA_C0BR1_SPEC>;
///LPDMA channel 0 block register 1
pub mod lpdma_c0br1;
///LPDMA_C0SAR (rw) register accessor: an alias for `Reg<LPDMA_C0SAR_SPEC>`
pub type LPDMA_C0SAR = crate::Reg<lpdma_c0sar::LPDMA_C0SAR_SPEC>;
///LPDMA channel 0 source address register
pub mod lpdma_c0sar;
///LPDMA_C0DAR (rw) register accessor: an alias for `Reg<LPDMA_C0DAR_SPEC>`
pub type LPDMA_C0DAR = crate::Reg<lpdma_c0dar::LPDMA_C0DAR_SPEC>;
///LPDMA channel 0 destination address register
pub mod lpdma_c0dar;
///LPDMA_C0LLR (rw) register accessor: an alias for `Reg<LPDMA_C0LLR_SPEC>`
pub type LPDMA_C0LLR = crate::Reg<lpdma_c0llr::LPDMA_C0LLR_SPEC>;
///LPDMA channel 0 linked-list address register
pub mod lpdma_c0llr;
///LPDMA_C1LBAR (rw) register accessor: an alias for `Reg<LPDMA_C1LBAR_SPEC>`
pub type LPDMA_C1LBAR = crate::Reg<lpdma_c1lbar::LPDMA_C1LBAR_SPEC>;
///LPDMA channel 1 linked-list base address register
pub mod lpdma_c1lbar;
///LPDMA_C1FCR (w) register accessor: an alias for `Reg<LPDMA_C1FCR_SPEC>`
pub type LPDMA_C1FCR = crate::Reg<lpdma_c1fcr::LPDMA_C1FCR_SPEC>;
///LPDMA channel 1 flag clear register
pub mod lpdma_c1fcr;
///LPDMA_C1SR (r) register accessor: an alias for `Reg<LPDMA_C1SR_SPEC>`
pub type LPDMA_C1SR = crate::Reg<lpdma_c1sr::LPDMA_C1SR_SPEC>;
///LPDMA channel 1 status register
pub mod lpdma_c1sr;
///LPDMA_C1CR (rw) register accessor: an alias for `Reg<LPDMA_C1CR_SPEC>`
pub type LPDMA_C1CR = crate::Reg<lpdma_c1cr::LPDMA_C1CR_SPEC>;
///LPDMA channel 1 control register
pub mod lpdma_c1cr;
///LPDMA_C1TR1 (rw) register accessor: an alias for `Reg<LPDMA_C1TR1_SPEC>`
pub type LPDMA_C1TR1 = crate::Reg<lpdma_c1tr1::LPDMA_C1TR1_SPEC>;
///LPDMA channel 1 transfer register 1
pub mod lpdma_c1tr1;
///LPDMA_C1TR2 (rw) register accessor: an alias for `Reg<LPDMA_C1TR2_SPEC>`
pub type LPDMA_C1TR2 = crate::Reg<lpdma_c1tr2::LPDMA_C1TR2_SPEC>;
///LPDMA channel 1 transfer register 2
pub mod lpdma_c1tr2;
///LPDMA_C1BR1 (rw) register accessor: an alias for `Reg<LPDMA_C1BR1_SPEC>`
pub type LPDMA_C1BR1 = crate::Reg<lpdma_c1br1::LPDMA_C1BR1_SPEC>;
///LPDMA channel 1 block register 1
pub mod lpdma_c1br1;
///LPDMA_C1SAR (rw) register accessor: an alias for `Reg<LPDMA_C1SAR_SPEC>`
pub type LPDMA_C1SAR = crate::Reg<lpdma_c1sar::LPDMA_C1SAR_SPEC>;
///LPDMA channel 1 source address register
pub mod lpdma_c1sar;
///LPDMA_C1DAR (rw) register accessor: an alias for `Reg<LPDMA_C1DAR_SPEC>`
pub type LPDMA_C1DAR = crate::Reg<lpdma_c1dar::LPDMA_C1DAR_SPEC>;
///LPDMA channel 1 destination address register
pub mod lpdma_c1dar;
///LPDMA_C1LLR (rw) register accessor: an alias for `Reg<LPDMA_C1LLR_SPEC>`
pub type LPDMA_C1LLR = crate::Reg<lpdma_c1llr::LPDMA_C1LLR_SPEC>;
///LPDMA channel 1 linked-list address register
pub mod lpdma_c1llr;
///LPDMA_C2LBAR (rw) register accessor: an alias for `Reg<LPDMA_C2LBAR_SPEC>`
pub type LPDMA_C2LBAR = crate::Reg<lpdma_c2lbar::LPDMA_C2LBAR_SPEC>;
///LPDMA channel 2 linked-list base address register
pub mod lpdma_c2lbar;
///LPDMA_C2FCR (w) register accessor: an alias for `Reg<LPDMA_C2FCR_SPEC>`
pub type LPDMA_C2FCR = crate::Reg<lpdma_c2fcr::LPDMA_C2FCR_SPEC>;
///LPDMA channel 2 flag clear register
pub mod lpdma_c2fcr;
///LPDMA_C2SR (r) register accessor: an alias for `Reg<LPDMA_C2SR_SPEC>`
pub type LPDMA_C2SR = crate::Reg<lpdma_c2sr::LPDMA_C2SR_SPEC>;
///LPDMA channel 2 status register
pub mod lpdma_c2sr;
///LPDMA_C2CR (rw) register accessor: an alias for `Reg<LPDMA_C2CR_SPEC>`
pub type LPDMA_C2CR = crate::Reg<lpdma_c2cr::LPDMA_C2CR_SPEC>;
///LPDMA channel 2 control register
pub mod lpdma_c2cr;
///LPDMA_C2TR1 (rw) register accessor: an alias for `Reg<LPDMA_C2TR1_SPEC>`
pub type LPDMA_C2TR1 = crate::Reg<lpdma_c2tr1::LPDMA_C2TR1_SPEC>;
///LPDMA channel 2 transfer register 1
pub mod lpdma_c2tr1;
///LPDMA_C2TR2 (rw) register accessor: an alias for `Reg<LPDMA_C2TR2_SPEC>`
pub type LPDMA_C2TR2 = crate::Reg<lpdma_c2tr2::LPDMA_C2TR2_SPEC>;
///LPDMA channel 2 transfer register 2
pub mod lpdma_c2tr2;
///LPDMA_C2BR1 (rw) register accessor: an alias for `Reg<LPDMA_C2BR1_SPEC>`
pub type LPDMA_C2BR1 = crate::Reg<lpdma_c2br1::LPDMA_C2BR1_SPEC>;
///LPDMA channel 2 block register 1
pub mod lpdma_c2br1;
///LPDMA_C2SAR (rw) register accessor: an alias for `Reg<LPDMA_C2SAR_SPEC>`
pub type LPDMA_C2SAR = crate::Reg<lpdma_c2sar::LPDMA_C2SAR_SPEC>;
///LPDMA channel 2 source address register
pub mod lpdma_c2sar;
///LPDMA_C2DAR (rw) register accessor: an alias for `Reg<LPDMA_C2DAR_SPEC>`
pub type LPDMA_C2DAR = crate::Reg<lpdma_c2dar::LPDMA_C2DAR_SPEC>;
///LPDMA channel 2 destination address register
pub mod lpdma_c2dar;
///LPDMA_C2LLR (rw) register accessor: an alias for `Reg<LPDMA_C2LLR_SPEC>`
pub type LPDMA_C2LLR = crate::Reg<lpdma_c2llr::LPDMA_C2LLR_SPEC>;
///LPDMA channel 2 linked-list address register
pub mod lpdma_c2llr;
///LPDMA_C3LBAR (rw) register accessor: an alias for `Reg<LPDMA_C3LBAR_SPEC>`
pub type LPDMA_C3LBAR = crate::Reg<lpdma_c3lbar::LPDMA_C3LBAR_SPEC>;
///LPDMA channel 3 linked-list base address register
pub mod lpdma_c3lbar;
///LPDMA_C3FCR (w) register accessor: an alias for `Reg<LPDMA_C3FCR_SPEC>`
pub type LPDMA_C3FCR = crate::Reg<lpdma_c3fcr::LPDMA_C3FCR_SPEC>;
///LPDMA channel 3 flag clear register
pub mod lpdma_c3fcr;
///LPDMA_C3SR (r) register accessor: an alias for `Reg<LPDMA_C3SR_SPEC>`
pub type LPDMA_C3SR = crate::Reg<lpdma_c3sr::LPDMA_C3SR_SPEC>;
///LPDMA channel 3 status register
pub mod lpdma_c3sr;
///LPDMA_C3CR (rw) register accessor: an alias for `Reg<LPDMA_C3CR_SPEC>`
pub type LPDMA_C3CR = crate::Reg<lpdma_c3cr::LPDMA_C3CR_SPEC>;
///LPDMA channel 3 control register
pub mod lpdma_c3cr;
///LPDMA_C3TR1 (rw) register accessor: an alias for `Reg<LPDMA_C3TR1_SPEC>`
pub type LPDMA_C3TR1 = crate::Reg<lpdma_c3tr1::LPDMA_C3TR1_SPEC>;
///LPDMA channel 3 transfer register 1
pub mod lpdma_c3tr1;
///LPDMA_C3TR2 (rw) register accessor: an alias for `Reg<LPDMA_C3TR2_SPEC>`
pub type LPDMA_C3TR2 = crate::Reg<lpdma_c3tr2::LPDMA_C3TR2_SPEC>;
///LPDMA channel 3 transfer register 2
pub mod lpdma_c3tr2;
///LPDMA_C3BR1 (rw) register accessor: an alias for `Reg<LPDMA_C3BR1_SPEC>`
pub type LPDMA_C3BR1 = crate::Reg<lpdma_c3br1::LPDMA_C3BR1_SPEC>;
///LPDMA channel 3 block register 1
pub mod lpdma_c3br1;
///LPDMA_C3SAR (rw) register accessor: an alias for `Reg<LPDMA_C3SAR_SPEC>`
pub type LPDMA_C3SAR = crate::Reg<lpdma_c3sar::LPDMA_C3SAR_SPEC>;
///LPDMA channel 3 source address register
pub mod lpdma_c3sar;
///LPDMA_C3DAR (rw) register accessor: an alias for `Reg<LPDMA_C3DAR_SPEC>`
pub type LPDMA_C3DAR = crate::Reg<lpdma_c3dar::LPDMA_C3DAR_SPEC>;
///LPDMA channel 3 destination address register
pub mod lpdma_c3dar;
///LPDMA_C3LLR (rw) register accessor: an alias for `Reg<LPDMA_C3LLR_SPEC>`
pub type LPDMA_C3LLR = crate::Reg<lpdma_c3llr::LPDMA_C3LLR_SPEC>;
///LPDMA channel 3 linked-list address register
pub mod lpdma_c3llr;
