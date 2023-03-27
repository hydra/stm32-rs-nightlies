///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPDMA secure configuration register
    pub gpdma_seccfgr: GPDMA_SECCFGR,
    ///0x04 - GPDMA privileged configuration register
    pub gpdma_privcfgr: GPDMA_PRIVCFGR,
    ///0x08 - GPDMA configuration lock register
    pub gpdma_rcfglockr: GPDMA_RCFGLOCKR,
    ///0x0c - GPDMA non-secure masked interrupt status register
    pub gpdma_misr: GPDMA_MISR,
    ///0x10 - GPDMA secure masked interrupt status register
    pub gpdma_smisr: GPDMA_SMISR,
    _reserved5: [u8; 0x3c],
    ///0x50 - GPDMA channel 0 linked-list base address register
    pub gpdma_c0lbar: GPDMA_C0LBAR,
    _reserved6: [u8; 0x08],
    ///0x5c - GPDMA channel 0 flag clear register
    pub gpdma_c0fcr: GPDMA_C0FCR,
    ///0x60 - GPDMA channel 0 status register
    pub gpdma_c0sr: GPDMA_C0SR,
    ///0x64 - GPDMA channel 0 control register
    pub gpdma_c0cr: GPDMA_C0CR,
    _reserved9: [u8; 0x28],
    ///0x90 - GPDMA channel 0 transfer register 1
    pub gpdma_c0tr1: GPDMA_C0TR1,
    ///0x94 - GPDMA channel 0 transfer register 2
    pub gpdma_c0tr2: GPDMA_C0TR2,
    ///0x98 - GPDMA channel 0 block register 1
    pub gpdma_c0br1: GPDMA_C0BR1,
    ///0x9c - GPDMA channel 0 source address register
    pub gpdma_c0sar: GPDMA_C0SAR,
    ///0xa0 - GPDMA channel 0 destination address register
    pub gpdma_c0dar: GPDMA_C0DAR,
    _reserved14: [u8; 0x28],
    ///0xcc - GPDMA channel 0 linked-list address register
    pub gpdma_c0llr: GPDMA_C0LLR,
    ///0xd0 - GPDMA channel 1 linked-list base address register
    pub gpdma_c1lbar: GPDMA_C1LBAR,
    _reserved16: [u8; 0x08],
    ///0xdc - GPDMA channel 1 flag clear register
    pub gpdma_c1fcr: GPDMA_C1FCR,
    ///0xe0 - GPDMA channel 1 status register
    pub gpdma_c1sr: GPDMA_C1SR,
    ///0xe4 - GPDMA channel 1 control register
    pub gpdma_c1cr: GPDMA_C1CR,
    _reserved19: [u8; 0x28],
    ///0x110 - GPDMA channel 1 transfer register 1
    pub gpdma_c1tr1: GPDMA_C1TR1,
    ///0x114 - GPDMA channel 1 transfer register 2
    pub gpdma_c1tr2: GPDMA_C1TR2,
    ///0x118 - GPDMA channel 1 block register 1
    pub gpdma_c1br1: GPDMA_C1BR1,
    ///0x11c - GPDMA channel 1 source address register
    pub gpdma_c1sar: GPDMA_C1SAR,
    ///0x120 - GPDMA channel 1 destination address register
    pub gpdma_c1dar: GPDMA_C1DAR,
    _reserved24: [u8; 0x28],
    ///0x14c - GPDMA channel 1 linked-list address register
    pub gpdma_c1llr: GPDMA_C1LLR,
    ///0x150 - GPDMA channel 2 linked-list base address register
    pub gpdma_c2lbar: GPDMA_C2LBAR,
    _reserved26: [u8; 0x08],
    ///0x15c - GPDMA channel 2 flag clear register
    pub gpdma_c2fcr: GPDMA_C2FCR,
    ///0x160 - GPDMA channel 2 status register
    pub gpdma_c2sr: GPDMA_C2SR,
    ///0x164 - GPDMA channel 2 control register
    pub gpdma_c2cr: GPDMA_C2CR,
    _reserved29: [u8; 0x28],
    ///0x190 - GPDMA channel 2 transfer register 1
    pub gpdma_c2tr1: GPDMA_C2TR1,
    ///0x194 - GPDMA channel 2 transfer register 2
    pub gpdma_c2tr2: GPDMA_C2TR2,
    ///0x198 - GPDMA channel 2 block register 1
    pub gpdma_c2br1: GPDMA_C2BR1,
    ///0x19c - GPDMA channel 2 source address register
    pub gpdma_c2sar: GPDMA_C2SAR,
    ///0x1a0 - GPDMA channel 2 destination address register
    pub gpdma_c2dar: GPDMA_C2DAR,
    _reserved34: [u8; 0x28],
    ///0x1cc - GPDMA channel 2 linked-list address register
    pub gpdma_c2llr: GPDMA_C2LLR,
    ///0x1d0 - GPDMA channel 3 linked-list base address register
    pub gpdma_c3lbar: GPDMA_C3LBAR,
    _reserved36: [u8; 0x08],
    ///0x1dc - GPDMA channel 3 flag clear register
    pub gpdma_c3fcr: GPDMA_C3FCR,
    ///0x1e0 - GPDMA channel 3 status register
    pub gpdma_c3sr: GPDMA_C3SR,
    ///0x1e4 - GPDMA channel 3 control register
    pub gpdma_c3cr: GPDMA_C3CR,
    _reserved39: [u8; 0x28],
    ///0x210 - GPDMA channel 3 transfer register 1
    pub gpdma_c3tr1: GPDMA_C3TR1,
    ///0x214 - GPDMA channel 3 transfer register 2
    pub gpdma_c3tr2: GPDMA_C3TR2,
    ///0x218 - GPDMA channel 3 block register 1
    pub gpdma_c3br1: GPDMA_C3BR1,
    ///0x21c - GPDMA channel 3 source address register
    pub gpdma_c3sar: GPDMA_C3SAR,
    ///0x220 - GPDMA channel 3 destination address register
    pub gpdma_c3dar: GPDMA_C3DAR,
    _reserved44: [u8; 0x28],
    ///0x24c - GPDMA channel 3 linked-list address register
    pub gpdma_c3llr: GPDMA_C3LLR,
    ///0x250 - GPDMA channel 4 linked-list base address register
    pub gpdma_c4lbar: GPDMA_C4LBAR,
    _reserved46: [u8; 0x08],
    ///0x25c - GPDMA channel 4 flag clear register
    pub gpdma_c4fcr: GPDMA_C4FCR,
    ///0x260 - GPDMA channel 4 status register
    pub gpdma_c4sr: GPDMA_C4SR,
    ///0x264 - GPDMA channel 4 control register
    pub gpdma_c4cr: GPDMA_C4CR,
    _reserved49: [u8; 0x28],
    ///0x290 - GPDMA channel 4 transfer register 1
    pub gpdma_c4tr1: GPDMA_C4TR1,
    ///0x294 - GPDMA channel 4 transfer register 2
    pub gpdma_c4tr2: GPDMA_C4TR2,
    ///0x298 - GPDMA channel 4 block register 1
    pub gpdma_c4br1: GPDMA_C4BR1,
    ///0x29c - GPDMA channel 4 source address register
    pub gpdma_c4sar: GPDMA_C4SAR,
    ///0x2a0 - GPDMA channel 4 destination address register
    pub gpdma_c4dar: GPDMA_C4DAR,
    _reserved54: [u8; 0x28],
    ///0x2cc - GPDMA channel 4 linked-list address register
    pub gpdma_c4llr: GPDMA_C4LLR,
    ///0x2d0 - GPDMA channel 5 linked-list base address register
    pub gpdma_c5lbar: GPDMA_C5LBAR,
    _reserved56: [u8; 0x08],
    ///0x2dc - GPDMA channel 5 flag clear register
    pub gpdma_c5fcr: GPDMA_C5FCR,
    ///0x2e0 - GPDMA channel 5 status register
    pub gpdma_c5sr: GPDMA_C5SR,
    ///0x2e4 - GPDMA channel 5 control register
    pub gpdma_c5cr: GPDMA_C5CR,
    _reserved59: [u8; 0x28],
    ///0x310 - GPDMA channel 5 transfer register 1
    pub gpdma_c5tr1: GPDMA_C5TR1,
    ///0x314 - GPDMA channel 5 transfer register 2
    pub gpdma_c5tr2: GPDMA_C5TR2,
    ///0x318 - GPDMA channel 5 block register 1
    pub gpdma_c5br1: GPDMA_C5BR1,
    ///0x31c - GPDMA channel 5 source address register
    pub gpdma_c5sar: GPDMA_C5SAR,
    ///0x320 - GPDMA channel 5 destination address register
    pub gpdma_c5dar: GPDMA_C5DAR,
    _reserved64: [u8; 0x28],
    ///0x34c - GPDMA channel 5 linked-list address register
    pub gpdma_c5llr: GPDMA_C5LLR,
    ///0x350 - GPDMA channel 6 linked-list base address register
    pub gpdma_c6lbar: GPDMA_C6LBAR,
    _reserved66: [u8; 0x08],
    ///0x35c - GPDMA channel 6 flag clear register
    pub gpdma_c6fcr: GPDMA_C6FCR,
    ///0x360 - GPDMA channel 6 status register
    pub gpdma_c6sr: GPDMA_C6SR,
    ///0x364 - GPDMA channel 6 control register
    pub gpdma_c6cr: GPDMA_C6CR,
    _reserved69: [u8; 0x28],
    ///0x390 - GPDMA channel 6 transfer register 1
    pub gpdma_c6tr1: GPDMA_C6TR1,
    ///0x394 - GPDMA channel 6 transfer register 2
    pub gpdma_c6tr2: GPDMA_C6TR2,
    ///0x398 - GPDMA channel 6 block register 1
    pub gpdma_c6br1: GPDMA_C6BR1,
    ///0x39c - GPDMA channel 6 source address register
    pub gpdma_c6sar: GPDMA_C6SAR,
    ///0x3a0 - GPDMA channel 6 destination address register
    pub gpdma_c6dar: GPDMA_C6DAR,
    _reserved74: [u8; 0x28],
    ///0x3cc - GPDMA channel 6 linked-list address register
    pub gpdma_c6llr: GPDMA_C6LLR,
    ///0x3d0 - GPDMA channel 7 linked-list base address register
    pub gpdma_c7lbar: GPDMA_C7LBAR,
    _reserved76: [u8; 0x08],
    ///0x3dc - GPDMA channel 7 flag clear register
    pub gpdma_c7fcr: GPDMA_C7FCR,
    ///0x3e0 - GPDMA channel 7 status register
    pub gpdma_c7sr: GPDMA_C7SR,
    ///0x3e4 - GPDMA channel 7 control register
    pub gpdma_c7cr: GPDMA_C7CR,
    _reserved79: [u8; 0x28],
    ///0x410 - GPDMA channel 7 transfer register 1
    pub gpdma_c7tr1: GPDMA_C7TR1,
    ///0x414 - GPDMA channel 7 transfer register 2
    pub gpdma_c7tr2: GPDMA_C7TR2,
    ///0x418 - GPDMA channel 7 block register 1
    pub gpdma_c7br1: GPDMA_C7BR1,
    ///0x41c - GPDMA channel 7 source address register
    pub gpdma_c7sar: GPDMA_C7SAR,
    ///0x420 - GPDMA channel 7 destination address register
    pub gpdma_c7dar: GPDMA_C7DAR,
    _reserved84: [u8; 0x28],
    ///0x44c - GPDMA channel 7 linked-list address register
    pub gpdma_c7llr: GPDMA_C7LLR,
    ///0x450 - GPDMA channel 8 linked-list base address register
    pub gpdma_c8lbar: GPDMA_C8LBAR,
    _reserved86: [u8; 0x08],
    ///0x45c - GPDMA channel 8 flag clear register
    pub gpdma_c8fcr: GPDMA_C8FCR,
    ///0x460 - GPDMA channel 8 status register
    pub gpdma_c8sr: GPDMA_C8SR,
    ///0x464 - GPDMA channel 8 control register
    pub gpdma_c8cr: GPDMA_C8CR,
    _reserved89: [u8; 0x28],
    ///0x490 - GPDMA channel 8 transfer register 1
    pub gpdma_c8tr1: GPDMA_C8TR1,
    ///0x494 - GPDMA channel 8 transfer register 2
    pub gpdma_c8tr2: GPDMA_C8TR2,
    ///0x498 - GPDMA channel 8 block register 1
    pub gpdma_c8br1: GPDMA_C8BR1,
    ///0x49c - GPDMA channel 8 source address register
    pub gpdma_c8sar: GPDMA_C8SAR,
    ///0x4a0 - GPDMA channel 8 destination address register
    pub gpdma_c8dar: GPDMA_C8DAR,
    _reserved94: [u8; 0x28],
    ///0x4cc - GPDMA channel 8 linked-list address register
    pub gpdma_c8llr: GPDMA_C8LLR,
    ///0x4d0 - GPDMA channel 9 linked-list base address register
    pub gpdma_c9lbar: GPDMA_C9LBAR,
    _reserved96: [u8; 0x08],
    ///0x4dc - GPDMA channel 9 flag clear register
    pub gpdma_c9fcr: GPDMA_C9FCR,
    ///0x4e0 - GPDMA channel 9 status register
    pub gpdma_c9sr: GPDMA_C9SR,
    ///0x4e4 - GPDMA channel 9 control register
    pub gpdma_c9cr: GPDMA_C9CR,
    _reserved99: [u8; 0x28],
    ///0x510 - GPDMA channel 9 transfer register 1
    pub gpdma_c9tr1: GPDMA_C9TR1,
    ///0x514 - GPDMA channel 9 transfer register 2
    pub gpdma_c9tr2: GPDMA_C9TR2,
    ///0x518 - GPDMA channel 9 block register 1
    pub gpdma_c9br1: GPDMA_C9BR1,
    ///0x51c - GPDMA channel 9 source address register
    pub gpdma_c9sar: GPDMA_C9SAR,
    ///0x520 - GPDMA channel 9 destination address register
    pub gpdma_c9dar: GPDMA_C9DAR,
    _reserved104: [u8; 0x28],
    ///0x54c - GPDMA channel 9 linked-list address register
    pub gpdma_c9llr: GPDMA_C9LLR,
    ///0x550 - GPDMA channel 10 linked-list base address register
    pub gpdma_c10lbar: GPDMA_C10LBAR,
    _reserved106: [u8; 0x08],
    ///0x55c - GPDMA channel 10 flag clear register
    pub gpdma_c10fcr: GPDMA_C10FCR,
    ///0x560 - GPDMA channel 10 status register
    pub gpdma_c10sr: GPDMA_C10SR,
    ///0x564 - GPDMA channel 10 control register
    pub gpdma_c10cr: GPDMA_C10CR,
    _reserved109: [u8; 0x28],
    ///0x590 - GPDMA channel 10 transfer register 1
    pub gpdma_c10tr1: GPDMA_C10TR1,
    ///0x594 - GPDMA channel 10 transfer register 2
    pub gpdma_c10tr2: GPDMA_C10TR2,
    ///0x598 - GPDMA channel 10 block register 1
    pub gpdma_c10br1: GPDMA_C10BR1,
    ///0x59c - GPDMA channel 10 source address register
    pub gpdma_c10sar: GPDMA_C10SAR,
    ///0x5a0 - GPDMA channel 10 destination address register
    pub gpdma_c10dar: GPDMA_C10DAR,
    _reserved114: [u8; 0x28],
    ///0x5cc - GPDMA channel 10 linked-list address register
    pub gpdma_c10llr: GPDMA_C10LLR,
    ///0x5d0 - GPDMA channel 11 linked-list base address register
    pub gpdma_c11lbar: GPDMA_C11LBAR,
    _reserved116: [u8; 0x08],
    ///0x5dc - GPDMA channel 11 flag clear register
    pub gpdma_c11fcr: GPDMA_C11FCR,
    ///0x5e0 - GPDMA channel 11 status register
    pub gpdma_c11sr: GPDMA_C11SR,
    ///0x5e4 - GPDMA channel 11 control register
    pub gpdma_c11cr: GPDMA_C11CR,
    _reserved119: [u8; 0x28],
    ///0x610 - GPDMA channel 11 transfer register 1
    pub gpdma_c11tr1: GPDMA_C11TR1,
    ///0x614 - GPDMA channel 11 transfer register 2
    pub gpdma_c11tr2: GPDMA_C11TR2,
    ///0x618 - GPDMA channel 11 block register 1
    pub gpdma_c11br1: GPDMA_C11BR1,
    ///0x61c - GPDMA channel 11 source address register
    pub gpdma_c11sar: GPDMA_C11SAR,
    ///0x620 - GPDMA channel 11 destination address register
    pub gpdma_c11dar: GPDMA_C11DAR,
    _reserved124: [u8; 0x28],
    ///0x64c - GPDMA channel 11 linked-list address register
    pub gpdma_c11llr: GPDMA_C11LLR,
    ///0x650 - GPDMA channel 12 linked-list base address register
    pub gpdma_c12lbar: GPDMA_C12LBAR,
    _reserved126: [u8; 0x08],
    ///0x65c - GPDMA channel 12 flag clear register
    pub gpdma_c12fcr: GPDMA_C12FCR,
    ///0x660 - GPDMA channel 12 status register
    pub gpdma_c12sr: GPDMA_C12SR,
    ///0x664 - GPDMA channel 12 control register
    pub gpdma_c12cr: GPDMA_C12CR,
    _reserved129: [u8; 0x28],
    ///0x690 - GPDMA channel 12 transfer register 1
    pub gpdma_c12tr1: GPDMA_C12TR1,
    ///0x694 - GPDMA channel 12 transfer register 2
    pub gpdma_c12tr2: GPDMA_C12TR2,
    ///0x698 - GPDMA channel 12 alternate block register 1
    pub gpdma_c12br1: GPDMA_C12BR1,
    ///0x69c - GPDMA channel 12 source address register
    pub gpdma_c12sar: GPDMA_C12SAR,
    ///0x6a0 - GPDMA channel 12 destination address register
    pub gpdma_c12dar: GPDMA_C12DAR,
    ///0x6a4 - GPDMA channel 12 transfer register 3
    pub gpdma_c12tr3: GPDMA_C12TR3,
    ///0x6a8 - GPDMA channel 12 block register 2
    pub gpdma_c12br2: GPDMA_C12BR2,
    _reserved136: [u8; 0x20],
    ///0x6cc - GPDMA channel 12 alternate linked-list address register
    pub gpdma_c12llr: GPDMA_C12LLR,
    ///0x6d0 - GPDMA channel 13 linked-list base address register
    pub gpdma_c13lbar: GPDMA_C13LBAR,
    _reserved138: [u8; 0x08],
    ///0x6dc - GPDMA channel 13 flag clear register
    pub gpdma_c13fcr: GPDMA_C13FCR,
    ///0x6e0 - GPDMA channel 13 status register
    pub gpdma_c13sr: GPDMA_C13SR,
    ///0x6e4 - GPDMA channel 13 control register
    pub gpdma_c13cr: GPDMA_C13CR,
    _reserved141: [u8; 0x28],
    ///0x710 - GPDMA channel 13 transfer register 1
    pub gpdma_c13tr1: GPDMA_C13TR1,
    ///0x714 - GPDMA channel 13 transfer register 2
    pub gpdma_c13tr2: GPDMA_C13TR2,
    ///0x718 - GPDMA channel 13 alternate block register 1
    pub gpdma_c13br1: GPDMA_C13BR1,
    ///0x71c - GPDMA channel 13 source address register
    pub gpdma_c13sar: GPDMA_C13SAR,
    ///0x720 - GPDMA channel 13 destination address register
    pub gpdma_c13dar: GPDMA_C13DAR,
    ///0x724 - GPDMA channel 13 transfer register 3
    pub gpdma_c13tr3: GPDMA_C13TR3,
    ///0x728 - GPDMA channel 13 block register 2
    pub gpdma_c13br2: GPDMA_C13BR2,
    _reserved148: [u8; 0x20],
    ///0x74c - GPDMA channel 13 alternate linked-list address register
    pub gpdma_c13llr: GPDMA_C13LLR,
    ///0x750 - GPDMA channel 14 linked-list base address register
    pub gpdma_c14lbar: GPDMA_C14LBAR,
    _reserved150: [u8; 0x08],
    ///0x75c - GPDMA channel 14 flag clear register
    pub gpdma_c14fcr: GPDMA_C14FCR,
    ///0x760 - GPDMA channel 14 status register
    pub gpdma_c14sr: GPDMA_C14SR,
    ///0x764 - GPDMA channel 14 control register
    pub gpdma_c14cr: GPDMA_C14CR,
    _reserved153: [u8; 0x28],
    ///0x790 - GPDMA channel 14 transfer register 1
    pub gpdma_c14tr1: GPDMA_C14TR1,
    ///0x794 - GPDMA channel 14 transfer register 2
    pub gpdma_c14tr2: GPDMA_C14TR2,
    ///0x798 - GPDMA channel 14 alternate block register 1
    pub gpdma_c14br1: GPDMA_C14BR1,
    ///0x79c - GPDMA channel 14 source address register
    pub gpdma_c14sar: GPDMA_C14SAR,
    ///0x7a0 - GPDMA channel 14 destination address register
    pub gpdma_c14dar: GPDMA_C14DAR,
    ///0x7a4 - GPDMA channel 14 transfer register 3
    pub gpdma_c14tr3: GPDMA_C14TR3,
    ///0x7a8 - GPDMA channel 14 block register 2
    pub gpdma_c14br2: GPDMA_C14BR2,
    _reserved160: [u8; 0x20],
    ///0x7cc - GPDMA channel 14 alternate linked-list address register
    pub gpdma_c14llr: GPDMA_C14LLR,
    ///0x7d0 - GPDMA channel 15 linked-list base address register
    pub gpdma_c15lbar: GPDMA_C15LBAR,
    _reserved162: [u8; 0x08],
    ///0x7dc - GPDMA channel 15 flag clear register
    pub gpdma_c15fcr: GPDMA_C15FCR,
    ///0x7e0 - GPDMA channel 15 status register
    pub gpdma_c15sr: GPDMA_C15SR,
    ///0x7e4 - GPDMA channel 15 control register
    pub gpdma_c15cr: GPDMA_C15CR,
    _reserved165: [u8; 0x28],
    ///0x810 - GPDMA channel 15 transfer register 1
    pub gpdma_c15tr1: GPDMA_C15TR1,
    ///0x814 - GPDMA channel 15 transfer register 2
    pub gpdma_c15tr2: GPDMA_C15TR2,
    ///0x818 - GPDMA channel 15 alternate block register 1
    pub gpdma_c15br1: GPDMA_C15BR1,
    ///0x81c - GPDMA channel 15 source address register
    pub gpdma_c15sar: GPDMA_C15SAR,
    ///0x820 - GPDMA channel 15 destination address register
    pub gpdma_c15dar: GPDMA_C15DAR,
    ///0x824 - GPDMA channel 15 transfer register 3
    pub gpdma_c15tr3: GPDMA_C15TR3,
    ///0x828 - GPDMA channel 15 block register 2
    pub gpdma_c15br2: GPDMA_C15BR2,
    _reserved172: [u8; 0x20],
    ///0x84c - GPDMA channel 15 alternate linked-list address register
    pub gpdma_c15llr: GPDMA_C15LLR,
}
///GPDMA_SECCFGR (rw) register accessor: an alias for `Reg<GPDMA_SECCFGR_SPEC>`
pub type GPDMA_SECCFGR = crate::Reg<gpdma_seccfgr::GPDMA_SECCFGR_SPEC>;
///GPDMA secure configuration register
pub mod gpdma_seccfgr;
///GPDMA_PRIVCFGR (rw) register accessor: an alias for `Reg<GPDMA_PRIVCFGR_SPEC>`
pub type GPDMA_PRIVCFGR = crate::Reg<gpdma_privcfgr::GPDMA_PRIVCFGR_SPEC>;
///GPDMA privileged configuration register
pub mod gpdma_privcfgr;
///GPDMA_RCFGLOCKR (rw) register accessor: an alias for `Reg<GPDMA_RCFGLOCKR_SPEC>`
pub type GPDMA_RCFGLOCKR = crate::Reg<gpdma_rcfglockr::GPDMA_RCFGLOCKR_SPEC>;
///GPDMA configuration lock register
pub mod gpdma_rcfglockr;
///GPDMA_MISR (r) register accessor: an alias for `Reg<GPDMA_MISR_SPEC>`
pub type GPDMA_MISR = crate::Reg<gpdma_misr::GPDMA_MISR_SPEC>;
///GPDMA non-secure masked interrupt status register
pub mod gpdma_misr;
///GPDMA_SMISR (r) register accessor: an alias for `Reg<GPDMA_SMISR_SPEC>`
pub type GPDMA_SMISR = crate::Reg<gpdma_smisr::GPDMA_SMISR_SPEC>;
///GPDMA secure masked interrupt status register
pub mod gpdma_smisr;
///GPDMA_C0LBAR (rw) register accessor: an alias for `Reg<GPDMA_C0LBAR_SPEC>`
pub type GPDMA_C0LBAR = crate::Reg<gpdma_c0lbar::GPDMA_C0LBAR_SPEC>;
///GPDMA channel 0 linked-list base address register
pub mod gpdma_c0lbar;
///GPDMA_C0FCR (w) register accessor: an alias for `Reg<GPDMA_C0FCR_SPEC>`
pub type GPDMA_C0FCR = crate::Reg<gpdma_c0fcr::GPDMA_C0FCR_SPEC>;
///GPDMA channel 0 flag clear register
pub mod gpdma_c0fcr;
///GPDMA_C0SR (r) register accessor: an alias for `Reg<GPDMA_C0SR_SPEC>`
pub type GPDMA_C0SR = crate::Reg<gpdma_c0sr::GPDMA_C0SR_SPEC>;
///GPDMA channel 0 status register
pub mod gpdma_c0sr;
///GPDMA_C0CR (rw) register accessor: an alias for `Reg<GPDMA_C0CR_SPEC>`
pub type GPDMA_C0CR = crate::Reg<gpdma_c0cr::GPDMA_C0CR_SPEC>;
///GPDMA channel 0 control register
pub mod gpdma_c0cr;
///GPDMA_C0TR1 (rw) register accessor: an alias for `Reg<GPDMA_C0TR1_SPEC>`
pub type GPDMA_C0TR1 = crate::Reg<gpdma_c0tr1::GPDMA_C0TR1_SPEC>;
///GPDMA channel 0 transfer register 1
pub mod gpdma_c0tr1;
///GPDMA_C0TR2 (rw) register accessor: an alias for `Reg<GPDMA_C0TR2_SPEC>`
pub type GPDMA_C0TR2 = crate::Reg<gpdma_c0tr2::GPDMA_C0TR2_SPEC>;
///GPDMA channel 0 transfer register 2
pub mod gpdma_c0tr2;
///GPDMA_C0BR1 (rw) register accessor: an alias for `Reg<GPDMA_C0BR1_SPEC>`
pub type GPDMA_C0BR1 = crate::Reg<gpdma_c0br1::GPDMA_C0BR1_SPEC>;
///GPDMA channel 0 block register 1
pub mod gpdma_c0br1;
///GPDMA_C0SAR (rw) register accessor: an alias for `Reg<GPDMA_C0SAR_SPEC>`
pub type GPDMA_C0SAR = crate::Reg<gpdma_c0sar::GPDMA_C0SAR_SPEC>;
///GPDMA channel 0 source address register
pub mod gpdma_c0sar;
///GPDMA_C0DAR (rw) register accessor: an alias for `Reg<GPDMA_C0DAR_SPEC>`
pub type GPDMA_C0DAR = crate::Reg<gpdma_c0dar::GPDMA_C0DAR_SPEC>;
///GPDMA channel 0 destination address register
pub mod gpdma_c0dar;
///GPDMA_C0LLR (rw) register accessor: an alias for `Reg<GPDMA_C0LLR_SPEC>`
pub type GPDMA_C0LLR = crate::Reg<gpdma_c0llr::GPDMA_C0LLR_SPEC>;
///GPDMA channel 0 linked-list address register
pub mod gpdma_c0llr;
///GPDMA_C1LBAR (rw) register accessor: an alias for `Reg<GPDMA_C1LBAR_SPEC>`
pub type GPDMA_C1LBAR = crate::Reg<gpdma_c1lbar::GPDMA_C1LBAR_SPEC>;
///GPDMA channel 1 linked-list base address register
pub mod gpdma_c1lbar;
///GPDMA_C1FCR (w) register accessor: an alias for `Reg<GPDMA_C1FCR_SPEC>`
pub type GPDMA_C1FCR = crate::Reg<gpdma_c1fcr::GPDMA_C1FCR_SPEC>;
///GPDMA channel 1 flag clear register
pub mod gpdma_c1fcr;
///GPDMA_C1SR (r) register accessor: an alias for `Reg<GPDMA_C1SR_SPEC>`
pub type GPDMA_C1SR = crate::Reg<gpdma_c1sr::GPDMA_C1SR_SPEC>;
///GPDMA channel 1 status register
pub mod gpdma_c1sr;
///GPDMA_C1CR (rw) register accessor: an alias for `Reg<GPDMA_C1CR_SPEC>`
pub type GPDMA_C1CR = crate::Reg<gpdma_c1cr::GPDMA_C1CR_SPEC>;
///GPDMA channel 1 control register
pub mod gpdma_c1cr;
///GPDMA_C1TR1 (rw) register accessor: an alias for `Reg<GPDMA_C1TR1_SPEC>`
pub type GPDMA_C1TR1 = crate::Reg<gpdma_c1tr1::GPDMA_C1TR1_SPEC>;
///GPDMA channel 1 transfer register 1
pub mod gpdma_c1tr1;
///GPDMA_C1TR2 (rw) register accessor: an alias for `Reg<GPDMA_C1TR2_SPEC>`
pub type GPDMA_C1TR2 = crate::Reg<gpdma_c1tr2::GPDMA_C1TR2_SPEC>;
///GPDMA channel 1 transfer register 2
pub mod gpdma_c1tr2;
///GPDMA_C1BR1 (rw) register accessor: an alias for `Reg<GPDMA_C1BR1_SPEC>`
pub type GPDMA_C1BR1 = crate::Reg<gpdma_c1br1::GPDMA_C1BR1_SPEC>;
///GPDMA channel 1 block register 1
pub mod gpdma_c1br1;
///GPDMA_C1SAR (rw) register accessor: an alias for `Reg<GPDMA_C1SAR_SPEC>`
pub type GPDMA_C1SAR = crate::Reg<gpdma_c1sar::GPDMA_C1SAR_SPEC>;
///GPDMA channel 1 source address register
pub mod gpdma_c1sar;
///GPDMA_C1DAR (rw) register accessor: an alias for `Reg<GPDMA_C1DAR_SPEC>`
pub type GPDMA_C1DAR = crate::Reg<gpdma_c1dar::GPDMA_C1DAR_SPEC>;
///GPDMA channel 1 destination address register
pub mod gpdma_c1dar;
///GPDMA_C1LLR (rw) register accessor: an alias for `Reg<GPDMA_C1LLR_SPEC>`
pub type GPDMA_C1LLR = crate::Reg<gpdma_c1llr::GPDMA_C1LLR_SPEC>;
///GPDMA channel 1 linked-list address register
pub mod gpdma_c1llr;
///GPDMA_C2LBAR (rw) register accessor: an alias for `Reg<GPDMA_C2LBAR_SPEC>`
pub type GPDMA_C2LBAR = crate::Reg<gpdma_c2lbar::GPDMA_C2LBAR_SPEC>;
///GPDMA channel 2 linked-list base address register
pub mod gpdma_c2lbar;
///GPDMA_C2FCR (w) register accessor: an alias for `Reg<GPDMA_C2FCR_SPEC>`
pub type GPDMA_C2FCR = crate::Reg<gpdma_c2fcr::GPDMA_C2FCR_SPEC>;
///GPDMA channel 2 flag clear register
pub mod gpdma_c2fcr;
///GPDMA_C2SR (r) register accessor: an alias for `Reg<GPDMA_C2SR_SPEC>`
pub type GPDMA_C2SR = crate::Reg<gpdma_c2sr::GPDMA_C2SR_SPEC>;
///GPDMA channel 2 status register
pub mod gpdma_c2sr;
///GPDMA_C2CR (rw) register accessor: an alias for `Reg<GPDMA_C2CR_SPEC>`
pub type GPDMA_C2CR = crate::Reg<gpdma_c2cr::GPDMA_C2CR_SPEC>;
///GPDMA channel 2 control register
pub mod gpdma_c2cr;
///GPDMA_C2TR1 (rw) register accessor: an alias for `Reg<GPDMA_C2TR1_SPEC>`
pub type GPDMA_C2TR1 = crate::Reg<gpdma_c2tr1::GPDMA_C2TR1_SPEC>;
///GPDMA channel 2 transfer register 1
pub mod gpdma_c2tr1;
///GPDMA_C2TR2 (rw) register accessor: an alias for `Reg<GPDMA_C2TR2_SPEC>`
pub type GPDMA_C2TR2 = crate::Reg<gpdma_c2tr2::GPDMA_C2TR2_SPEC>;
///GPDMA channel 2 transfer register 2
pub mod gpdma_c2tr2;
///GPDMA_C2BR1 (rw) register accessor: an alias for `Reg<GPDMA_C2BR1_SPEC>`
pub type GPDMA_C2BR1 = crate::Reg<gpdma_c2br1::GPDMA_C2BR1_SPEC>;
///GPDMA channel 2 block register 1
pub mod gpdma_c2br1;
///GPDMA_C2SAR (rw) register accessor: an alias for `Reg<GPDMA_C2SAR_SPEC>`
pub type GPDMA_C2SAR = crate::Reg<gpdma_c2sar::GPDMA_C2SAR_SPEC>;
///GPDMA channel 2 source address register
pub mod gpdma_c2sar;
///GPDMA_C2DAR (rw) register accessor: an alias for `Reg<GPDMA_C2DAR_SPEC>`
pub type GPDMA_C2DAR = crate::Reg<gpdma_c2dar::GPDMA_C2DAR_SPEC>;
///GPDMA channel 2 destination address register
pub mod gpdma_c2dar;
///GPDMA_C2LLR (rw) register accessor: an alias for `Reg<GPDMA_C2LLR_SPEC>`
pub type GPDMA_C2LLR = crate::Reg<gpdma_c2llr::GPDMA_C2LLR_SPEC>;
///GPDMA channel 2 linked-list address register
pub mod gpdma_c2llr;
///GPDMA_C3LBAR (rw) register accessor: an alias for `Reg<GPDMA_C3LBAR_SPEC>`
pub type GPDMA_C3LBAR = crate::Reg<gpdma_c3lbar::GPDMA_C3LBAR_SPEC>;
///GPDMA channel 3 linked-list base address register
pub mod gpdma_c3lbar;
///GPDMA_C3FCR (w) register accessor: an alias for `Reg<GPDMA_C3FCR_SPEC>`
pub type GPDMA_C3FCR = crate::Reg<gpdma_c3fcr::GPDMA_C3FCR_SPEC>;
///GPDMA channel 3 flag clear register
pub mod gpdma_c3fcr;
///GPDMA_C3SR (r) register accessor: an alias for `Reg<GPDMA_C3SR_SPEC>`
pub type GPDMA_C3SR = crate::Reg<gpdma_c3sr::GPDMA_C3SR_SPEC>;
///GPDMA channel 3 status register
pub mod gpdma_c3sr;
///GPDMA_C3CR (rw) register accessor: an alias for `Reg<GPDMA_C3CR_SPEC>`
pub type GPDMA_C3CR = crate::Reg<gpdma_c3cr::GPDMA_C3CR_SPEC>;
///GPDMA channel 3 control register
pub mod gpdma_c3cr;
///GPDMA_C3TR1 (rw) register accessor: an alias for `Reg<GPDMA_C3TR1_SPEC>`
pub type GPDMA_C3TR1 = crate::Reg<gpdma_c3tr1::GPDMA_C3TR1_SPEC>;
///GPDMA channel 3 transfer register 1
pub mod gpdma_c3tr1;
///GPDMA_C3TR2 (rw) register accessor: an alias for `Reg<GPDMA_C3TR2_SPEC>`
pub type GPDMA_C3TR2 = crate::Reg<gpdma_c3tr2::GPDMA_C3TR2_SPEC>;
///GPDMA channel 3 transfer register 2
pub mod gpdma_c3tr2;
///GPDMA_C3BR1 (rw) register accessor: an alias for `Reg<GPDMA_C3BR1_SPEC>`
pub type GPDMA_C3BR1 = crate::Reg<gpdma_c3br1::GPDMA_C3BR1_SPEC>;
///GPDMA channel 3 block register 1
pub mod gpdma_c3br1;
///GPDMA_C3SAR (rw) register accessor: an alias for `Reg<GPDMA_C3SAR_SPEC>`
pub type GPDMA_C3SAR = crate::Reg<gpdma_c3sar::GPDMA_C3SAR_SPEC>;
///GPDMA channel 3 source address register
pub mod gpdma_c3sar;
///GPDMA_C3DAR (rw) register accessor: an alias for `Reg<GPDMA_C3DAR_SPEC>`
pub type GPDMA_C3DAR = crate::Reg<gpdma_c3dar::GPDMA_C3DAR_SPEC>;
///GPDMA channel 3 destination address register
pub mod gpdma_c3dar;
///GPDMA_C3LLR (rw) register accessor: an alias for `Reg<GPDMA_C3LLR_SPEC>`
pub type GPDMA_C3LLR = crate::Reg<gpdma_c3llr::GPDMA_C3LLR_SPEC>;
///GPDMA channel 3 linked-list address register
pub mod gpdma_c3llr;
///GPDMA_C4LBAR (rw) register accessor: an alias for `Reg<GPDMA_C4LBAR_SPEC>`
pub type GPDMA_C4LBAR = crate::Reg<gpdma_c4lbar::GPDMA_C4LBAR_SPEC>;
///GPDMA channel 4 linked-list base address register
pub mod gpdma_c4lbar;
///GPDMA_C4FCR (w) register accessor: an alias for `Reg<GPDMA_C4FCR_SPEC>`
pub type GPDMA_C4FCR = crate::Reg<gpdma_c4fcr::GPDMA_C4FCR_SPEC>;
///GPDMA channel 4 flag clear register
pub mod gpdma_c4fcr;
///GPDMA_C4SR (r) register accessor: an alias for `Reg<GPDMA_C4SR_SPEC>`
pub type GPDMA_C4SR = crate::Reg<gpdma_c4sr::GPDMA_C4SR_SPEC>;
///GPDMA channel 4 status register
pub mod gpdma_c4sr;
///GPDMA_C4CR (rw) register accessor: an alias for `Reg<GPDMA_C4CR_SPEC>`
pub type GPDMA_C4CR = crate::Reg<gpdma_c4cr::GPDMA_C4CR_SPEC>;
///GPDMA channel 4 control register
pub mod gpdma_c4cr;
///GPDMA_C4TR1 (rw) register accessor: an alias for `Reg<GPDMA_C4TR1_SPEC>`
pub type GPDMA_C4TR1 = crate::Reg<gpdma_c4tr1::GPDMA_C4TR1_SPEC>;
///GPDMA channel 4 transfer register 1
pub mod gpdma_c4tr1;
///GPDMA_C4TR2 (rw) register accessor: an alias for `Reg<GPDMA_C4TR2_SPEC>`
pub type GPDMA_C4TR2 = crate::Reg<gpdma_c4tr2::GPDMA_C4TR2_SPEC>;
///GPDMA channel 4 transfer register 2
pub mod gpdma_c4tr2;
///GPDMA_C4BR1 (rw) register accessor: an alias for `Reg<GPDMA_C4BR1_SPEC>`
pub type GPDMA_C4BR1 = crate::Reg<gpdma_c4br1::GPDMA_C4BR1_SPEC>;
///GPDMA channel 4 block register 1
pub mod gpdma_c4br1;
///GPDMA_C4SAR (rw) register accessor: an alias for `Reg<GPDMA_C4SAR_SPEC>`
pub type GPDMA_C4SAR = crate::Reg<gpdma_c4sar::GPDMA_C4SAR_SPEC>;
///GPDMA channel 4 source address register
pub mod gpdma_c4sar;
///GPDMA_C4DAR (rw) register accessor: an alias for `Reg<GPDMA_C4DAR_SPEC>`
pub type GPDMA_C4DAR = crate::Reg<gpdma_c4dar::GPDMA_C4DAR_SPEC>;
///GPDMA channel 4 destination address register
pub mod gpdma_c4dar;
///GPDMA_C4LLR (rw) register accessor: an alias for `Reg<GPDMA_C4LLR_SPEC>`
pub type GPDMA_C4LLR = crate::Reg<gpdma_c4llr::GPDMA_C4LLR_SPEC>;
///GPDMA channel 4 linked-list address register
pub mod gpdma_c4llr;
///GPDMA_C5LBAR (rw) register accessor: an alias for `Reg<GPDMA_C5LBAR_SPEC>`
pub type GPDMA_C5LBAR = crate::Reg<gpdma_c5lbar::GPDMA_C5LBAR_SPEC>;
///GPDMA channel 5 linked-list base address register
pub mod gpdma_c5lbar;
///GPDMA_C5FCR (w) register accessor: an alias for `Reg<GPDMA_C5FCR_SPEC>`
pub type GPDMA_C5FCR = crate::Reg<gpdma_c5fcr::GPDMA_C5FCR_SPEC>;
///GPDMA channel 5 flag clear register
pub mod gpdma_c5fcr;
///GPDMA_C5SR (r) register accessor: an alias for `Reg<GPDMA_C5SR_SPEC>`
pub type GPDMA_C5SR = crate::Reg<gpdma_c5sr::GPDMA_C5SR_SPEC>;
///GPDMA channel 5 status register
pub mod gpdma_c5sr;
///GPDMA_C5CR (rw) register accessor: an alias for `Reg<GPDMA_C5CR_SPEC>`
pub type GPDMA_C5CR = crate::Reg<gpdma_c5cr::GPDMA_C5CR_SPEC>;
///GPDMA channel 5 control register
pub mod gpdma_c5cr;
///GPDMA_C5TR1 (rw) register accessor: an alias for `Reg<GPDMA_C5TR1_SPEC>`
pub type GPDMA_C5TR1 = crate::Reg<gpdma_c5tr1::GPDMA_C5TR1_SPEC>;
///GPDMA channel 5 transfer register 1
pub mod gpdma_c5tr1;
///GPDMA_C5TR2 (rw) register accessor: an alias for `Reg<GPDMA_C5TR2_SPEC>`
pub type GPDMA_C5TR2 = crate::Reg<gpdma_c5tr2::GPDMA_C5TR2_SPEC>;
///GPDMA channel 5 transfer register 2
pub mod gpdma_c5tr2;
///GPDMA_C5BR1 (rw) register accessor: an alias for `Reg<GPDMA_C5BR1_SPEC>`
pub type GPDMA_C5BR1 = crate::Reg<gpdma_c5br1::GPDMA_C5BR1_SPEC>;
///GPDMA channel 5 block register 1
pub mod gpdma_c5br1;
///GPDMA_C5SAR (rw) register accessor: an alias for `Reg<GPDMA_C5SAR_SPEC>`
pub type GPDMA_C5SAR = crate::Reg<gpdma_c5sar::GPDMA_C5SAR_SPEC>;
///GPDMA channel 5 source address register
pub mod gpdma_c5sar;
///GPDMA_C5DAR (rw) register accessor: an alias for `Reg<GPDMA_C5DAR_SPEC>`
pub type GPDMA_C5DAR = crate::Reg<gpdma_c5dar::GPDMA_C5DAR_SPEC>;
///GPDMA channel 5 destination address register
pub mod gpdma_c5dar;
///GPDMA_C5LLR (rw) register accessor: an alias for `Reg<GPDMA_C5LLR_SPEC>`
pub type GPDMA_C5LLR = crate::Reg<gpdma_c5llr::GPDMA_C5LLR_SPEC>;
///GPDMA channel 5 linked-list address register
pub mod gpdma_c5llr;
///GPDMA_C6LBAR (rw) register accessor: an alias for `Reg<GPDMA_C6LBAR_SPEC>`
pub type GPDMA_C6LBAR = crate::Reg<gpdma_c6lbar::GPDMA_C6LBAR_SPEC>;
///GPDMA channel 6 linked-list base address register
pub mod gpdma_c6lbar;
///GPDMA_C6FCR (w) register accessor: an alias for `Reg<GPDMA_C6FCR_SPEC>`
pub type GPDMA_C6FCR = crate::Reg<gpdma_c6fcr::GPDMA_C6FCR_SPEC>;
///GPDMA channel 6 flag clear register
pub mod gpdma_c6fcr;
///GPDMA_C6SR (r) register accessor: an alias for `Reg<GPDMA_C6SR_SPEC>`
pub type GPDMA_C6SR = crate::Reg<gpdma_c6sr::GPDMA_C6SR_SPEC>;
///GPDMA channel 6 status register
pub mod gpdma_c6sr;
///GPDMA_C6CR (rw) register accessor: an alias for `Reg<GPDMA_C6CR_SPEC>`
pub type GPDMA_C6CR = crate::Reg<gpdma_c6cr::GPDMA_C6CR_SPEC>;
///GPDMA channel 6 control register
pub mod gpdma_c6cr;
///GPDMA_C6TR1 (rw) register accessor: an alias for `Reg<GPDMA_C6TR1_SPEC>`
pub type GPDMA_C6TR1 = crate::Reg<gpdma_c6tr1::GPDMA_C6TR1_SPEC>;
///GPDMA channel 6 transfer register 1
pub mod gpdma_c6tr1;
///GPDMA_C6TR2 (rw) register accessor: an alias for `Reg<GPDMA_C6TR2_SPEC>`
pub type GPDMA_C6TR2 = crate::Reg<gpdma_c6tr2::GPDMA_C6TR2_SPEC>;
///GPDMA channel 6 transfer register 2
pub mod gpdma_c6tr2;
///GPDMA_C6BR1 (rw) register accessor: an alias for `Reg<GPDMA_C6BR1_SPEC>`
pub type GPDMA_C6BR1 = crate::Reg<gpdma_c6br1::GPDMA_C6BR1_SPEC>;
///GPDMA channel 6 block register 1
pub mod gpdma_c6br1;
///GPDMA_C6SAR (rw) register accessor: an alias for `Reg<GPDMA_C6SAR_SPEC>`
pub type GPDMA_C6SAR = crate::Reg<gpdma_c6sar::GPDMA_C6SAR_SPEC>;
///GPDMA channel 6 source address register
pub mod gpdma_c6sar;
///GPDMA_C6DAR (rw) register accessor: an alias for `Reg<GPDMA_C6DAR_SPEC>`
pub type GPDMA_C6DAR = crate::Reg<gpdma_c6dar::GPDMA_C6DAR_SPEC>;
///GPDMA channel 6 destination address register
pub mod gpdma_c6dar;
///GPDMA_C6LLR (rw) register accessor: an alias for `Reg<GPDMA_C6LLR_SPEC>`
pub type GPDMA_C6LLR = crate::Reg<gpdma_c6llr::GPDMA_C6LLR_SPEC>;
///GPDMA channel 6 linked-list address register
pub mod gpdma_c6llr;
///GPDMA_C7LBAR (rw) register accessor: an alias for `Reg<GPDMA_C7LBAR_SPEC>`
pub type GPDMA_C7LBAR = crate::Reg<gpdma_c7lbar::GPDMA_C7LBAR_SPEC>;
///GPDMA channel 7 linked-list base address register
pub mod gpdma_c7lbar;
///GPDMA_C7FCR (w) register accessor: an alias for `Reg<GPDMA_C7FCR_SPEC>`
pub type GPDMA_C7FCR = crate::Reg<gpdma_c7fcr::GPDMA_C7FCR_SPEC>;
///GPDMA channel 7 flag clear register
pub mod gpdma_c7fcr;
///GPDMA_C7SR (r) register accessor: an alias for `Reg<GPDMA_C7SR_SPEC>`
pub type GPDMA_C7SR = crate::Reg<gpdma_c7sr::GPDMA_C7SR_SPEC>;
///GPDMA channel 7 status register
pub mod gpdma_c7sr;
///GPDMA_C7CR (rw) register accessor: an alias for `Reg<GPDMA_C7CR_SPEC>`
pub type GPDMA_C7CR = crate::Reg<gpdma_c7cr::GPDMA_C7CR_SPEC>;
///GPDMA channel 7 control register
pub mod gpdma_c7cr;
///GPDMA_C7TR1 (rw) register accessor: an alias for `Reg<GPDMA_C7TR1_SPEC>`
pub type GPDMA_C7TR1 = crate::Reg<gpdma_c7tr1::GPDMA_C7TR1_SPEC>;
///GPDMA channel 7 transfer register 1
pub mod gpdma_c7tr1;
///GPDMA_C7TR2 (rw) register accessor: an alias for `Reg<GPDMA_C7TR2_SPEC>`
pub type GPDMA_C7TR2 = crate::Reg<gpdma_c7tr2::GPDMA_C7TR2_SPEC>;
///GPDMA channel 7 transfer register 2
pub mod gpdma_c7tr2;
///GPDMA_C7BR1 (rw) register accessor: an alias for `Reg<GPDMA_C7BR1_SPEC>`
pub type GPDMA_C7BR1 = crate::Reg<gpdma_c7br1::GPDMA_C7BR1_SPEC>;
///GPDMA channel 7 block register 1
pub mod gpdma_c7br1;
///GPDMA_C7SAR (rw) register accessor: an alias for `Reg<GPDMA_C7SAR_SPEC>`
pub type GPDMA_C7SAR = crate::Reg<gpdma_c7sar::GPDMA_C7SAR_SPEC>;
///GPDMA channel 7 source address register
pub mod gpdma_c7sar;
///GPDMA_C7DAR (rw) register accessor: an alias for `Reg<GPDMA_C7DAR_SPEC>`
pub type GPDMA_C7DAR = crate::Reg<gpdma_c7dar::GPDMA_C7DAR_SPEC>;
///GPDMA channel 7 destination address register
pub mod gpdma_c7dar;
///GPDMA_C7LLR (rw) register accessor: an alias for `Reg<GPDMA_C7LLR_SPEC>`
pub type GPDMA_C7LLR = crate::Reg<gpdma_c7llr::GPDMA_C7LLR_SPEC>;
///GPDMA channel 7 linked-list address register
pub mod gpdma_c7llr;
///GPDMA_C8LBAR (rw) register accessor: an alias for `Reg<GPDMA_C8LBAR_SPEC>`
pub type GPDMA_C8LBAR = crate::Reg<gpdma_c8lbar::GPDMA_C8LBAR_SPEC>;
///GPDMA channel 8 linked-list base address register
pub mod gpdma_c8lbar;
///GPDMA_C8FCR (w) register accessor: an alias for `Reg<GPDMA_C8FCR_SPEC>`
pub type GPDMA_C8FCR = crate::Reg<gpdma_c8fcr::GPDMA_C8FCR_SPEC>;
///GPDMA channel 8 flag clear register
pub mod gpdma_c8fcr;
///GPDMA_C8SR (r) register accessor: an alias for `Reg<GPDMA_C8SR_SPEC>`
pub type GPDMA_C8SR = crate::Reg<gpdma_c8sr::GPDMA_C8SR_SPEC>;
///GPDMA channel 8 status register
pub mod gpdma_c8sr;
///GPDMA_C8CR (rw) register accessor: an alias for `Reg<GPDMA_C8CR_SPEC>`
pub type GPDMA_C8CR = crate::Reg<gpdma_c8cr::GPDMA_C8CR_SPEC>;
///GPDMA channel 8 control register
pub mod gpdma_c8cr;
///GPDMA_C8TR1 (rw) register accessor: an alias for `Reg<GPDMA_C8TR1_SPEC>`
pub type GPDMA_C8TR1 = crate::Reg<gpdma_c8tr1::GPDMA_C8TR1_SPEC>;
///GPDMA channel 8 transfer register 1
pub mod gpdma_c8tr1;
///GPDMA_C8TR2 (rw) register accessor: an alias for `Reg<GPDMA_C8TR2_SPEC>`
pub type GPDMA_C8TR2 = crate::Reg<gpdma_c8tr2::GPDMA_C8TR2_SPEC>;
///GPDMA channel 8 transfer register 2
pub mod gpdma_c8tr2;
///GPDMA_C8BR1 (rw) register accessor: an alias for `Reg<GPDMA_C8BR1_SPEC>`
pub type GPDMA_C8BR1 = crate::Reg<gpdma_c8br1::GPDMA_C8BR1_SPEC>;
///GPDMA channel 8 block register 1
pub mod gpdma_c8br1;
///GPDMA_C8SAR (rw) register accessor: an alias for `Reg<GPDMA_C8SAR_SPEC>`
pub type GPDMA_C8SAR = crate::Reg<gpdma_c8sar::GPDMA_C8SAR_SPEC>;
///GPDMA channel 8 source address register
pub mod gpdma_c8sar;
///GPDMA_C8DAR (rw) register accessor: an alias for `Reg<GPDMA_C8DAR_SPEC>`
pub type GPDMA_C8DAR = crate::Reg<gpdma_c8dar::GPDMA_C8DAR_SPEC>;
///GPDMA channel 8 destination address register
pub mod gpdma_c8dar;
///GPDMA_C8LLR (rw) register accessor: an alias for `Reg<GPDMA_C8LLR_SPEC>`
pub type GPDMA_C8LLR = crate::Reg<gpdma_c8llr::GPDMA_C8LLR_SPEC>;
///GPDMA channel 8 linked-list address register
pub mod gpdma_c8llr;
///GPDMA_C9LBAR (rw) register accessor: an alias for `Reg<GPDMA_C9LBAR_SPEC>`
pub type GPDMA_C9LBAR = crate::Reg<gpdma_c9lbar::GPDMA_C9LBAR_SPEC>;
///GPDMA channel 9 linked-list base address register
pub mod gpdma_c9lbar;
///GPDMA_C9FCR (w) register accessor: an alias for `Reg<GPDMA_C9FCR_SPEC>`
pub type GPDMA_C9FCR = crate::Reg<gpdma_c9fcr::GPDMA_C9FCR_SPEC>;
///GPDMA channel 9 flag clear register
pub mod gpdma_c9fcr;
///GPDMA_C9SR (r) register accessor: an alias for `Reg<GPDMA_C9SR_SPEC>`
pub type GPDMA_C9SR = crate::Reg<gpdma_c9sr::GPDMA_C9SR_SPEC>;
///GPDMA channel 9 status register
pub mod gpdma_c9sr;
///GPDMA_C9CR (rw) register accessor: an alias for `Reg<GPDMA_C9CR_SPEC>`
pub type GPDMA_C9CR = crate::Reg<gpdma_c9cr::GPDMA_C9CR_SPEC>;
///GPDMA channel 9 control register
pub mod gpdma_c9cr;
///GPDMA_C9TR1 (rw) register accessor: an alias for `Reg<GPDMA_C9TR1_SPEC>`
pub type GPDMA_C9TR1 = crate::Reg<gpdma_c9tr1::GPDMA_C9TR1_SPEC>;
///GPDMA channel 9 transfer register 1
pub mod gpdma_c9tr1;
///GPDMA_C9TR2 (rw) register accessor: an alias for `Reg<GPDMA_C9TR2_SPEC>`
pub type GPDMA_C9TR2 = crate::Reg<gpdma_c9tr2::GPDMA_C9TR2_SPEC>;
///GPDMA channel 9 transfer register 2
pub mod gpdma_c9tr2;
///GPDMA_C9BR1 (rw) register accessor: an alias for `Reg<GPDMA_C9BR1_SPEC>`
pub type GPDMA_C9BR1 = crate::Reg<gpdma_c9br1::GPDMA_C9BR1_SPEC>;
///GPDMA channel 9 block register 1
pub mod gpdma_c9br1;
///GPDMA_C9SAR (rw) register accessor: an alias for `Reg<GPDMA_C9SAR_SPEC>`
pub type GPDMA_C9SAR = crate::Reg<gpdma_c9sar::GPDMA_C9SAR_SPEC>;
///GPDMA channel 9 source address register
pub mod gpdma_c9sar;
///GPDMA_C9DAR (rw) register accessor: an alias for `Reg<GPDMA_C9DAR_SPEC>`
pub type GPDMA_C9DAR = crate::Reg<gpdma_c9dar::GPDMA_C9DAR_SPEC>;
///GPDMA channel 9 destination address register
pub mod gpdma_c9dar;
///GPDMA_C9LLR (rw) register accessor: an alias for `Reg<GPDMA_C9LLR_SPEC>`
pub type GPDMA_C9LLR = crate::Reg<gpdma_c9llr::GPDMA_C9LLR_SPEC>;
///GPDMA channel 9 linked-list address register
pub mod gpdma_c9llr;
///GPDMA_C10LBAR (rw) register accessor: an alias for `Reg<GPDMA_C10LBAR_SPEC>`
pub type GPDMA_C10LBAR = crate::Reg<gpdma_c10lbar::GPDMA_C10LBAR_SPEC>;
///GPDMA channel 10 linked-list base address register
pub mod gpdma_c10lbar;
///GPDMA_C10FCR (w) register accessor: an alias for `Reg<GPDMA_C10FCR_SPEC>`
pub type GPDMA_C10FCR = crate::Reg<gpdma_c10fcr::GPDMA_C10FCR_SPEC>;
///GPDMA channel 10 flag clear register
pub mod gpdma_c10fcr;
///GPDMA_C10SR (r) register accessor: an alias for `Reg<GPDMA_C10SR_SPEC>`
pub type GPDMA_C10SR = crate::Reg<gpdma_c10sr::GPDMA_C10SR_SPEC>;
///GPDMA channel 10 status register
pub mod gpdma_c10sr;
///GPDMA_C10CR (rw) register accessor: an alias for `Reg<GPDMA_C10CR_SPEC>`
pub type GPDMA_C10CR = crate::Reg<gpdma_c10cr::GPDMA_C10CR_SPEC>;
///GPDMA channel 10 control register
pub mod gpdma_c10cr;
///GPDMA_C10TR1 (rw) register accessor: an alias for `Reg<GPDMA_C10TR1_SPEC>`
pub type GPDMA_C10TR1 = crate::Reg<gpdma_c10tr1::GPDMA_C10TR1_SPEC>;
///GPDMA channel 10 transfer register 1
pub mod gpdma_c10tr1;
///GPDMA_C10TR2 (rw) register accessor: an alias for `Reg<GPDMA_C10TR2_SPEC>`
pub type GPDMA_C10TR2 = crate::Reg<gpdma_c10tr2::GPDMA_C10TR2_SPEC>;
///GPDMA channel 10 transfer register 2
pub mod gpdma_c10tr2;
///GPDMA_C10BR1 (rw) register accessor: an alias for `Reg<GPDMA_C10BR1_SPEC>`
pub type GPDMA_C10BR1 = crate::Reg<gpdma_c10br1::GPDMA_C10BR1_SPEC>;
///GPDMA channel 10 block register 1
pub mod gpdma_c10br1;
///GPDMA_C10SAR (rw) register accessor: an alias for `Reg<GPDMA_C10SAR_SPEC>`
pub type GPDMA_C10SAR = crate::Reg<gpdma_c10sar::GPDMA_C10SAR_SPEC>;
///GPDMA channel 10 source address register
pub mod gpdma_c10sar;
///GPDMA_C10DAR (rw) register accessor: an alias for `Reg<GPDMA_C10DAR_SPEC>`
pub type GPDMA_C10DAR = crate::Reg<gpdma_c10dar::GPDMA_C10DAR_SPEC>;
///GPDMA channel 10 destination address register
pub mod gpdma_c10dar;
///GPDMA_C10LLR (rw) register accessor: an alias for `Reg<GPDMA_C10LLR_SPEC>`
pub type GPDMA_C10LLR = crate::Reg<gpdma_c10llr::GPDMA_C10LLR_SPEC>;
///GPDMA channel 10 linked-list address register
pub mod gpdma_c10llr;
///GPDMA_C11LBAR (rw) register accessor: an alias for `Reg<GPDMA_C11LBAR_SPEC>`
pub type GPDMA_C11LBAR = crate::Reg<gpdma_c11lbar::GPDMA_C11LBAR_SPEC>;
///GPDMA channel 11 linked-list base address register
pub mod gpdma_c11lbar;
///GPDMA_C11FCR (w) register accessor: an alias for `Reg<GPDMA_C11FCR_SPEC>`
pub type GPDMA_C11FCR = crate::Reg<gpdma_c11fcr::GPDMA_C11FCR_SPEC>;
///GPDMA channel 11 flag clear register
pub mod gpdma_c11fcr;
///GPDMA_C11SR (r) register accessor: an alias for `Reg<GPDMA_C11SR_SPEC>`
pub type GPDMA_C11SR = crate::Reg<gpdma_c11sr::GPDMA_C11SR_SPEC>;
///GPDMA channel 11 status register
pub mod gpdma_c11sr;
///GPDMA_C11CR (rw) register accessor: an alias for `Reg<GPDMA_C11CR_SPEC>`
pub type GPDMA_C11CR = crate::Reg<gpdma_c11cr::GPDMA_C11CR_SPEC>;
///GPDMA channel 11 control register
pub mod gpdma_c11cr;
///GPDMA_C11TR1 (rw) register accessor: an alias for `Reg<GPDMA_C11TR1_SPEC>`
pub type GPDMA_C11TR1 = crate::Reg<gpdma_c11tr1::GPDMA_C11TR1_SPEC>;
///GPDMA channel 11 transfer register 1
pub mod gpdma_c11tr1;
///GPDMA_C11TR2 (rw) register accessor: an alias for `Reg<GPDMA_C11TR2_SPEC>`
pub type GPDMA_C11TR2 = crate::Reg<gpdma_c11tr2::GPDMA_C11TR2_SPEC>;
///GPDMA channel 11 transfer register 2
pub mod gpdma_c11tr2;
///GPDMA_C11BR1 (rw) register accessor: an alias for `Reg<GPDMA_C11BR1_SPEC>`
pub type GPDMA_C11BR1 = crate::Reg<gpdma_c11br1::GPDMA_C11BR1_SPEC>;
///GPDMA channel 11 block register 1
pub mod gpdma_c11br1;
///GPDMA_C11SAR (rw) register accessor: an alias for `Reg<GPDMA_C11SAR_SPEC>`
pub type GPDMA_C11SAR = crate::Reg<gpdma_c11sar::GPDMA_C11SAR_SPEC>;
///GPDMA channel 11 source address register
pub mod gpdma_c11sar;
///GPDMA_C11DAR (rw) register accessor: an alias for `Reg<GPDMA_C11DAR_SPEC>`
pub type GPDMA_C11DAR = crate::Reg<gpdma_c11dar::GPDMA_C11DAR_SPEC>;
///GPDMA channel 11 destination address register
pub mod gpdma_c11dar;
///GPDMA_C11LLR (rw) register accessor: an alias for `Reg<GPDMA_C11LLR_SPEC>`
pub type GPDMA_C11LLR = crate::Reg<gpdma_c11llr::GPDMA_C11LLR_SPEC>;
///GPDMA channel 11 linked-list address register
pub mod gpdma_c11llr;
///GPDMA_C12LBAR (rw) register accessor: an alias for `Reg<GPDMA_C12LBAR_SPEC>`
pub type GPDMA_C12LBAR = crate::Reg<gpdma_c12lbar::GPDMA_C12LBAR_SPEC>;
///GPDMA channel 12 linked-list base address register
pub mod gpdma_c12lbar;
///GPDMA_C12FCR (w) register accessor: an alias for `Reg<GPDMA_C12FCR_SPEC>`
pub type GPDMA_C12FCR = crate::Reg<gpdma_c12fcr::GPDMA_C12FCR_SPEC>;
///GPDMA channel 12 flag clear register
pub mod gpdma_c12fcr;
///GPDMA_C12SR (r) register accessor: an alias for `Reg<GPDMA_C12SR_SPEC>`
pub type GPDMA_C12SR = crate::Reg<gpdma_c12sr::GPDMA_C12SR_SPEC>;
///GPDMA channel 12 status register
pub mod gpdma_c12sr;
///GPDMA_C12CR (rw) register accessor: an alias for `Reg<GPDMA_C12CR_SPEC>`
pub type GPDMA_C12CR = crate::Reg<gpdma_c12cr::GPDMA_C12CR_SPEC>;
///GPDMA channel 12 control register
pub mod gpdma_c12cr;
///GPDMA_C12TR1 (rw) register accessor: an alias for `Reg<GPDMA_C12TR1_SPEC>`
pub type GPDMA_C12TR1 = crate::Reg<gpdma_c12tr1::GPDMA_C12TR1_SPEC>;
///GPDMA channel 12 transfer register 1
pub mod gpdma_c12tr1;
///GPDMA_C12TR2 (rw) register accessor: an alias for `Reg<GPDMA_C12TR2_SPEC>`
pub type GPDMA_C12TR2 = crate::Reg<gpdma_c12tr2::GPDMA_C12TR2_SPEC>;
///GPDMA channel 12 transfer register 2
pub mod gpdma_c12tr2;
///GPDMA_C12BR1 (rw) register accessor: an alias for `Reg<GPDMA_C12BR1_SPEC>`
pub type GPDMA_C12BR1 = crate::Reg<gpdma_c12br1::GPDMA_C12BR1_SPEC>;
///GPDMA channel 12 alternate block register 1
pub mod gpdma_c12br1;
///GPDMA_C12SAR (rw) register accessor: an alias for `Reg<GPDMA_C12SAR_SPEC>`
pub type GPDMA_C12SAR = crate::Reg<gpdma_c12sar::GPDMA_C12SAR_SPEC>;
///GPDMA channel 12 source address register
pub mod gpdma_c12sar;
///GPDMA_C12DAR (rw) register accessor: an alias for `Reg<GPDMA_C12DAR_SPEC>`
pub type GPDMA_C12DAR = crate::Reg<gpdma_c12dar::GPDMA_C12DAR_SPEC>;
///GPDMA channel 12 destination address register
pub mod gpdma_c12dar;
///GPDMA_C12TR3 (rw) register accessor: an alias for `Reg<GPDMA_C12TR3_SPEC>`
pub type GPDMA_C12TR3 = crate::Reg<gpdma_c12tr3::GPDMA_C12TR3_SPEC>;
///GPDMA channel 12 transfer register 3
pub mod gpdma_c12tr3;
///GPDMA_C12BR2 (rw) register accessor: an alias for `Reg<GPDMA_C12BR2_SPEC>`
pub type GPDMA_C12BR2 = crate::Reg<gpdma_c12br2::GPDMA_C12BR2_SPEC>;
///GPDMA channel 12 block register 2
pub mod gpdma_c12br2;
///GPDMA_C12LLR (rw) register accessor: an alias for `Reg<GPDMA_C12LLR_SPEC>`
pub type GPDMA_C12LLR = crate::Reg<gpdma_c12llr::GPDMA_C12LLR_SPEC>;
///GPDMA channel 12 alternate linked-list address register
pub mod gpdma_c12llr;
///GPDMA_C13LBAR (rw) register accessor: an alias for `Reg<GPDMA_C13LBAR_SPEC>`
pub type GPDMA_C13LBAR = crate::Reg<gpdma_c13lbar::GPDMA_C13LBAR_SPEC>;
///GPDMA channel 13 linked-list base address register
pub mod gpdma_c13lbar;
///GPDMA_C13FCR (w) register accessor: an alias for `Reg<GPDMA_C13FCR_SPEC>`
pub type GPDMA_C13FCR = crate::Reg<gpdma_c13fcr::GPDMA_C13FCR_SPEC>;
///GPDMA channel 13 flag clear register
pub mod gpdma_c13fcr;
///GPDMA_C13SR (r) register accessor: an alias for `Reg<GPDMA_C13SR_SPEC>`
pub type GPDMA_C13SR = crate::Reg<gpdma_c13sr::GPDMA_C13SR_SPEC>;
///GPDMA channel 13 status register
pub mod gpdma_c13sr;
///GPDMA_C13CR (rw) register accessor: an alias for `Reg<GPDMA_C13CR_SPEC>`
pub type GPDMA_C13CR = crate::Reg<gpdma_c13cr::GPDMA_C13CR_SPEC>;
///GPDMA channel 13 control register
pub mod gpdma_c13cr;
///GPDMA_C13TR1 (rw) register accessor: an alias for `Reg<GPDMA_C13TR1_SPEC>`
pub type GPDMA_C13TR1 = crate::Reg<gpdma_c13tr1::GPDMA_C13TR1_SPEC>;
///GPDMA channel 13 transfer register 1
pub mod gpdma_c13tr1;
///GPDMA_C13TR2 (rw) register accessor: an alias for `Reg<GPDMA_C13TR2_SPEC>`
pub type GPDMA_C13TR2 = crate::Reg<gpdma_c13tr2::GPDMA_C13TR2_SPEC>;
///GPDMA channel 13 transfer register 2
pub mod gpdma_c13tr2;
///GPDMA_C13BR1 (rw) register accessor: an alias for `Reg<GPDMA_C13BR1_SPEC>`
pub type GPDMA_C13BR1 = crate::Reg<gpdma_c13br1::GPDMA_C13BR1_SPEC>;
///GPDMA channel 13 alternate block register 1
pub mod gpdma_c13br1;
///GPDMA_C13SAR (rw) register accessor: an alias for `Reg<GPDMA_C13SAR_SPEC>`
pub type GPDMA_C13SAR = crate::Reg<gpdma_c13sar::GPDMA_C13SAR_SPEC>;
///GPDMA channel 13 source address register
pub mod gpdma_c13sar;
///GPDMA_C13DAR (rw) register accessor: an alias for `Reg<GPDMA_C13DAR_SPEC>`
pub type GPDMA_C13DAR = crate::Reg<gpdma_c13dar::GPDMA_C13DAR_SPEC>;
///GPDMA channel 13 destination address register
pub mod gpdma_c13dar;
///GPDMA_C13TR3 (rw) register accessor: an alias for `Reg<GPDMA_C13TR3_SPEC>`
pub type GPDMA_C13TR3 = crate::Reg<gpdma_c13tr3::GPDMA_C13TR3_SPEC>;
///GPDMA channel 13 transfer register 3
pub mod gpdma_c13tr3;
///GPDMA_C13BR2 (rw) register accessor: an alias for `Reg<GPDMA_C13BR2_SPEC>`
pub type GPDMA_C13BR2 = crate::Reg<gpdma_c13br2::GPDMA_C13BR2_SPEC>;
///GPDMA channel 13 block register 2
pub mod gpdma_c13br2;
///GPDMA_C13LLR (rw) register accessor: an alias for `Reg<GPDMA_C13LLR_SPEC>`
pub type GPDMA_C13LLR = crate::Reg<gpdma_c13llr::GPDMA_C13LLR_SPEC>;
///GPDMA channel 13 alternate linked-list address register
pub mod gpdma_c13llr;
///GPDMA_C14LBAR (rw) register accessor: an alias for `Reg<GPDMA_C14LBAR_SPEC>`
pub type GPDMA_C14LBAR = crate::Reg<gpdma_c14lbar::GPDMA_C14LBAR_SPEC>;
///GPDMA channel 14 linked-list base address register
pub mod gpdma_c14lbar;
///GPDMA_C14FCR (w) register accessor: an alias for `Reg<GPDMA_C14FCR_SPEC>`
pub type GPDMA_C14FCR = crate::Reg<gpdma_c14fcr::GPDMA_C14FCR_SPEC>;
///GPDMA channel 14 flag clear register
pub mod gpdma_c14fcr;
///GPDMA_C14SR (r) register accessor: an alias for `Reg<GPDMA_C14SR_SPEC>`
pub type GPDMA_C14SR = crate::Reg<gpdma_c14sr::GPDMA_C14SR_SPEC>;
///GPDMA channel 14 status register
pub mod gpdma_c14sr;
///GPDMA_C14CR (rw) register accessor: an alias for `Reg<GPDMA_C14CR_SPEC>`
pub type GPDMA_C14CR = crate::Reg<gpdma_c14cr::GPDMA_C14CR_SPEC>;
///GPDMA channel 14 control register
pub mod gpdma_c14cr;
///GPDMA_C14TR1 (rw) register accessor: an alias for `Reg<GPDMA_C14TR1_SPEC>`
pub type GPDMA_C14TR1 = crate::Reg<gpdma_c14tr1::GPDMA_C14TR1_SPEC>;
///GPDMA channel 14 transfer register 1
pub mod gpdma_c14tr1;
///GPDMA_C14TR2 (rw) register accessor: an alias for `Reg<GPDMA_C14TR2_SPEC>`
pub type GPDMA_C14TR2 = crate::Reg<gpdma_c14tr2::GPDMA_C14TR2_SPEC>;
///GPDMA channel 14 transfer register 2
pub mod gpdma_c14tr2;
///GPDMA_C14BR1 (rw) register accessor: an alias for `Reg<GPDMA_C14BR1_SPEC>`
pub type GPDMA_C14BR1 = crate::Reg<gpdma_c14br1::GPDMA_C14BR1_SPEC>;
///GPDMA channel 14 alternate block register 1
pub mod gpdma_c14br1;
///GPDMA_C14SAR (rw) register accessor: an alias for `Reg<GPDMA_C14SAR_SPEC>`
pub type GPDMA_C14SAR = crate::Reg<gpdma_c14sar::GPDMA_C14SAR_SPEC>;
///GPDMA channel 14 source address register
pub mod gpdma_c14sar;
///GPDMA_C14DAR (rw) register accessor: an alias for `Reg<GPDMA_C14DAR_SPEC>`
pub type GPDMA_C14DAR = crate::Reg<gpdma_c14dar::GPDMA_C14DAR_SPEC>;
///GPDMA channel 14 destination address register
pub mod gpdma_c14dar;
///GPDMA_C14TR3 (rw) register accessor: an alias for `Reg<GPDMA_C14TR3_SPEC>`
pub type GPDMA_C14TR3 = crate::Reg<gpdma_c14tr3::GPDMA_C14TR3_SPEC>;
///GPDMA channel 14 transfer register 3
pub mod gpdma_c14tr3;
///GPDMA_C14BR2 (rw) register accessor: an alias for `Reg<GPDMA_C14BR2_SPEC>`
pub type GPDMA_C14BR2 = crate::Reg<gpdma_c14br2::GPDMA_C14BR2_SPEC>;
///GPDMA channel 14 block register 2
pub mod gpdma_c14br2;
///GPDMA_C14LLR (rw) register accessor: an alias for `Reg<GPDMA_C14LLR_SPEC>`
pub type GPDMA_C14LLR = crate::Reg<gpdma_c14llr::GPDMA_C14LLR_SPEC>;
///GPDMA channel 14 alternate linked-list address register
pub mod gpdma_c14llr;
///GPDMA_C15LBAR (rw) register accessor: an alias for `Reg<GPDMA_C15LBAR_SPEC>`
pub type GPDMA_C15LBAR = crate::Reg<gpdma_c15lbar::GPDMA_C15LBAR_SPEC>;
///GPDMA channel 15 linked-list base address register
pub mod gpdma_c15lbar;
///GPDMA_C15FCR (w) register accessor: an alias for `Reg<GPDMA_C15FCR_SPEC>`
pub type GPDMA_C15FCR = crate::Reg<gpdma_c15fcr::GPDMA_C15FCR_SPEC>;
///GPDMA channel 15 flag clear register
pub mod gpdma_c15fcr;
///GPDMA_C15SR (r) register accessor: an alias for `Reg<GPDMA_C15SR_SPEC>`
pub type GPDMA_C15SR = crate::Reg<gpdma_c15sr::GPDMA_C15SR_SPEC>;
///GPDMA channel 15 status register
pub mod gpdma_c15sr;
///GPDMA_C15CR (rw) register accessor: an alias for `Reg<GPDMA_C15CR_SPEC>`
pub type GPDMA_C15CR = crate::Reg<gpdma_c15cr::GPDMA_C15CR_SPEC>;
///GPDMA channel 15 control register
pub mod gpdma_c15cr;
///GPDMA_C15TR1 (rw) register accessor: an alias for `Reg<GPDMA_C15TR1_SPEC>`
pub type GPDMA_C15TR1 = crate::Reg<gpdma_c15tr1::GPDMA_C15TR1_SPEC>;
///GPDMA channel 15 transfer register 1
pub mod gpdma_c15tr1;
///GPDMA_C15TR2 (rw) register accessor: an alias for `Reg<GPDMA_C15TR2_SPEC>`
pub type GPDMA_C15TR2 = crate::Reg<gpdma_c15tr2::GPDMA_C15TR2_SPEC>;
///GPDMA channel 15 transfer register 2
pub mod gpdma_c15tr2;
///GPDMA_C15BR1 (rw) register accessor: an alias for `Reg<GPDMA_C15BR1_SPEC>`
pub type GPDMA_C15BR1 = crate::Reg<gpdma_c15br1::GPDMA_C15BR1_SPEC>;
///GPDMA channel 15 alternate block register 1
pub mod gpdma_c15br1;
///GPDMA_C15SAR (rw) register accessor: an alias for `Reg<GPDMA_C15SAR_SPEC>`
pub type GPDMA_C15SAR = crate::Reg<gpdma_c15sar::GPDMA_C15SAR_SPEC>;
///GPDMA channel 15 source address register
pub mod gpdma_c15sar;
///GPDMA_C15DAR (rw) register accessor: an alias for `Reg<GPDMA_C15DAR_SPEC>`
pub type GPDMA_C15DAR = crate::Reg<gpdma_c15dar::GPDMA_C15DAR_SPEC>;
///GPDMA channel 15 destination address register
pub mod gpdma_c15dar;
///GPDMA_C15TR3 (rw) register accessor: an alias for `Reg<GPDMA_C15TR3_SPEC>`
pub type GPDMA_C15TR3 = crate::Reg<gpdma_c15tr3::GPDMA_C15TR3_SPEC>;
///GPDMA channel 15 transfer register 3
pub mod gpdma_c15tr3;
///GPDMA_C15BR2 (rw) register accessor: an alias for `Reg<GPDMA_C15BR2_SPEC>`
pub type GPDMA_C15BR2 = crate::Reg<gpdma_c15br2::GPDMA_C15BR2_SPEC>;
///GPDMA channel 15 block register 2
pub mod gpdma_c15br2;
///GPDMA_C15LLR (rw) register accessor: an alias for `Reg<GPDMA_C15LLR_SPEC>`
pub type GPDMA_C15LLR = crate::Reg<gpdma_c15llr::GPDMA_C15LLR_SPEC>;
///GPDMA channel 15 alternate linked-list address register
pub mod gpdma_c15llr;
