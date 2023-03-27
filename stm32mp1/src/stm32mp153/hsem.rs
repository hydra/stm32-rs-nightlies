///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r0: HSEM_R0,
    ///0x04 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r1: HSEM_R1,
    ///0x08 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r2: HSEM_R2,
    ///0x0c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r3: HSEM_R3,
    ///0x10 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r4: HSEM_R4,
    ///0x14 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r5: HSEM_R5,
    ///0x18 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r6: HSEM_R6,
    ///0x1c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r7: HSEM_R7,
    ///0x20 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r8: HSEM_R8,
    ///0x24 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r9: HSEM_R9,
    ///0x28 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r10: HSEM_R10,
    ///0x2c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r11: HSEM_R11,
    ///0x30 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r12: HSEM_R12,
    ///0x34 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r13: HSEM_R13,
    ///0x38 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r14: HSEM_R14,
    ///0x3c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r15: HSEM_R15,
    ///0x40 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r16: HSEM_R16,
    ///0x44 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r17: HSEM_R17,
    ///0x48 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r18: HSEM_R18,
    ///0x4c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r19: HSEM_R19,
    ///0x50 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r20: HSEM_R20,
    ///0x54 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r21: HSEM_R21,
    ///0x58 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r22: HSEM_R22,
    ///0x5c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r23: HSEM_R23,
    ///0x60 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r24: HSEM_R24,
    ///0x64 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r25: HSEM_R25,
    ///0x68 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r26: HSEM_R26,
    ///0x6c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r27: HSEM_R27,
    ///0x70 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r28: HSEM_R28,
    ///0x74 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r29: HSEM_R29,
    ///0x78 - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r30: HSEM_R30,
    ///0x7c - The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_r31: HSEM_R31,
    ///0x80 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr0: HSEM_RLR0,
    ///0x84 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr1: HSEM_RLR1,
    ///0x88 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr2: HSEM_RLR2,
    ///0x8c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr3: HSEM_RLR3,
    ///0x90 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr4: HSEM_RLR4,
    ///0x94 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr5: HSEM_RLR5,
    ///0x98 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr6: HSEM_RLR6,
    ///0x9c - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr7: HSEM_RLR7,
    ///0xa0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr8: HSEM_RLR8,
    ///0xa4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr9: HSEM_RLR9,
    ///0xa8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr10: HSEM_RLR10,
    ///0xac - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr11: HSEM_RLR11,
    ///0xb0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr12: HSEM_RLR12,
    ///0xb4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr13: HSEM_RLR13,
    ///0xb8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr14: HSEM_RLR14,
    ///0xbc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr15: HSEM_RLR15,
    ///0xc0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr16: HSEM_RLR16,
    ///0xc4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr17: HSEM_RLR17,
    ///0xc8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr18: HSEM_RLR18,
    ///0xcc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr19: HSEM_RLR19,
    ///0xd0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr20: HSEM_RLR20,
    ///0xd4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr21: HSEM_RLR21,
    ///0xd8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr22: HSEM_RLR22,
    ///0xdc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr23: HSEM_RLR23,
    ///0xe0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr24: HSEM_RLR24,
    ///0xe4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr25: HSEM_RLR25,
    ///0xe8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr26: HSEM_RLR26,
    ///0xec - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr27: HSEM_RLR27,
    ///0xf0 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr28: HSEM_RLR28,
    ///0xf4 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr29: HSEM_RLR29,
    ///0xf8 - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr30: HSEM_RLR30,
    ///0xfc - Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_rlr31: HSEM_RLR31,
    ///0x100 - HSEM i1terrupt enable register
    pub hsem_c1ier: HSEM_C1IER,
    ///0x104 - HSEM i1terrupt clear register
    pub hsem_c1icr: HSEM_C1ICR,
    ///0x108 - HSEM i1terrupt status register
    pub hsem_c1isr: HSEM_C1ISR,
    ///0x10c - HSEM i1terrupt status register
    pub hsem_c1misr: HSEM_C1MISR,
    ///0x110 - HSEM i2terrupt enable register
    pub hsem_c2ier: HSEM_C2IER,
    ///0x114 - HSEM i2terrupt clear register
    pub hsem_c2icr: HSEM_C2ICR,
    ///0x118 - HSEM i2terrupt status register
    pub hsem_c2isr: HSEM_C2ISR,
    ///0x11c - HSEM i2terrupt status register
    pub hsem_c2misr: HSEM_C2MISR,
    _reserved72: [u8; 0x20],
    ///0x140 - Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
    pub hsem_cr: HSEM_CR,
    ///0x144 - HSEM interrupt clear register
    pub hsem_keyr: HSEM_KEYR,
    _reserved74: [u8; 0x02a4],
    ///0x3ec - HSEM hardware configuration register 2
    pub hsem_hwcfgr2: HSEM_HWCFGR2,
    ///0x3f0 - HSEM hardware configuration register 1
    pub hsem_hwcfgr1: HSEM_HWCFGR1,
    ///0x3f4 - HSEM IP version register
    pub hsem_verr: HSEM_VERR,
    ///0x3f8 - HSEM IP identification register
    pub hsem_ipidr: HSEM_IPIDR,
    ///0x3fc - HSEM size identification register
    pub hsem_sidr: HSEM_SIDR,
}
///HSEM_R0 (rw) register accessor: an alias for `Reg<HSEM_R0_SPEC>`
pub type HSEM_R0 = crate::Reg<hsem_r0::HSEM_R0_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r0;
///HSEM_R1 (rw) register accessor: an alias for `Reg<HSEM_R1_SPEC>`
pub type HSEM_R1 = crate::Reg<hsem_r1::HSEM_R1_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r1;
///HSEM_R2 (rw) register accessor: an alias for `Reg<HSEM_R2_SPEC>`
pub type HSEM_R2 = crate::Reg<hsem_r2::HSEM_R2_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r2;
///HSEM_R3 (rw) register accessor: an alias for `Reg<HSEM_R3_SPEC>`
pub type HSEM_R3 = crate::Reg<hsem_r3::HSEM_R3_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r3;
///HSEM_R4 (rw) register accessor: an alias for `Reg<HSEM_R4_SPEC>`
pub type HSEM_R4 = crate::Reg<hsem_r4::HSEM_R4_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r4;
///HSEM_R5 (rw) register accessor: an alias for `Reg<HSEM_R5_SPEC>`
pub type HSEM_R5 = crate::Reg<hsem_r5::HSEM_R5_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r5;
///HSEM_R6 (rw) register accessor: an alias for `Reg<HSEM_R6_SPEC>`
pub type HSEM_R6 = crate::Reg<hsem_r6::HSEM_R6_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r6;
///HSEM_R7 (rw) register accessor: an alias for `Reg<HSEM_R7_SPEC>`
pub type HSEM_R7 = crate::Reg<hsem_r7::HSEM_R7_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r7;
///HSEM_R8 (rw) register accessor: an alias for `Reg<HSEM_R8_SPEC>`
pub type HSEM_R8 = crate::Reg<hsem_r8::HSEM_R8_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r8;
///HSEM_R9 (rw) register accessor: an alias for `Reg<HSEM_R9_SPEC>`
pub type HSEM_R9 = crate::Reg<hsem_r9::HSEM_R9_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r9;
///HSEM_R10 (rw) register accessor: an alias for `Reg<HSEM_R10_SPEC>`
pub type HSEM_R10 = crate::Reg<hsem_r10::HSEM_R10_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r10;
///HSEM_R11 (rw) register accessor: an alias for `Reg<HSEM_R11_SPEC>`
pub type HSEM_R11 = crate::Reg<hsem_r11::HSEM_R11_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r11;
///HSEM_R12 (rw) register accessor: an alias for `Reg<HSEM_R12_SPEC>`
pub type HSEM_R12 = crate::Reg<hsem_r12::HSEM_R12_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r12;
///HSEM_R13 (rw) register accessor: an alias for `Reg<HSEM_R13_SPEC>`
pub type HSEM_R13 = crate::Reg<hsem_r13::HSEM_R13_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r13;
///HSEM_R14 (rw) register accessor: an alias for `Reg<HSEM_R14_SPEC>`
pub type HSEM_R14 = crate::Reg<hsem_r14::HSEM_R14_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r14;
///HSEM_R15 (rw) register accessor: an alias for `Reg<HSEM_R15_SPEC>`
pub type HSEM_R15 = crate::Reg<hsem_r15::HSEM_R15_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r15;
///HSEM_R16 (rw) register accessor: an alias for `Reg<HSEM_R16_SPEC>`
pub type HSEM_R16 = crate::Reg<hsem_r16::HSEM_R16_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r16;
///HSEM_R17 (rw) register accessor: an alias for `Reg<HSEM_R17_SPEC>`
pub type HSEM_R17 = crate::Reg<hsem_r17::HSEM_R17_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r17;
///HSEM_R18 (rw) register accessor: an alias for `Reg<HSEM_R18_SPEC>`
pub type HSEM_R18 = crate::Reg<hsem_r18::HSEM_R18_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r18;
///HSEM_R19 (rw) register accessor: an alias for `Reg<HSEM_R19_SPEC>`
pub type HSEM_R19 = crate::Reg<hsem_r19::HSEM_R19_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r19;
///HSEM_R20 (rw) register accessor: an alias for `Reg<HSEM_R20_SPEC>`
pub type HSEM_R20 = crate::Reg<hsem_r20::HSEM_R20_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r20;
///HSEM_R21 (rw) register accessor: an alias for `Reg<HSEM_R21_SPEC>`
pub type HSEM_R21 = crate::Reg<hsem_r21::HSEM_R21_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r21;
///HSEM_R22 (rw) register accessor: an alias for `Reg<HSEM_R22_SPEC>`
pub type HSEM_R22 = crate::Reg<hsem_r22::HSEM_R22_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r22;
///HSEM_R23 (rw) register accessor: an alias for `Reg<HSEM_R23_SPEC>`
pub type HSEM_R23 = crate::Reg<hsem_r23::HSEM_R23_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r23;
///HSEM_R24 (rw) register accessor: an alias for `Reg<HSEM_R24_SPEC>`
pub type HSEM_R24 = crate::Reg<hsem_r24::HSEM_R24_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r24;
///HSEM_R25 (rw) register accessor: an alias for `Reg<HSEM_R25_SPEC>`
pub type HSEM_R25 = crate::Reg<hsem_r25::HSEM_R25_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r25;
///HSEM_R26 (rw) register accessor: an alias for `Reg<HSEM_R26_SPEC>`
pub type HSEM_R26 = crate::Reg<hsem_r26::HSEM_R26_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r26;
///HSEM_R27 (rw) register accessor: an alias for `Reg<HSEM_R27_SPEC>`
pub type HSEM_R27 = crate::Reg<hsem_r27::HSEM_R27_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r27;
///HSEM_R28 (rw) register accessor: an alias for `Reg<HSEM_R28_SPEC>`
pub type HSEM_R28 = crate::Reg<hsem_r28::HSEM_R28_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r28;
///HSEM_R29 (rw) register accessor: an alias for `Reg<HSEM_R29_SPEC>`
pub type HSEM_R29 = crate::Reg<hsem_r29::HSEM_R29_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r29;
///HSEM_R30 (rw) register accessor: an alias for `Reg<HSEM_R30_SPEC>`
pub type HSEM_R30 = crate::Reg<hsem_r30::HSEM_R30_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r30;
///HSEM_R31 (rw) register accessor: an alias for `Reg<HSEM_R31_SPEC>`
pub type HSEM_R31 = crate::Reg<hsem_r31::HSEM_R31_SPEC>;
///The HSEM_Rx shall be used to perform a 2-step Write lock and Read back. Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_r31;
///HSEM_RLR0 (r) register accessor: an alias for `Reg<HSEM_RLR0_SPEC>`
pub type HSEM_RLR0 = crate::Reg<hsem_rlr0::HSEM_RLR0_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr0;
///HSEM_RLR1 (r) register accessor: an alias for `Reg<HSEM_RLR1_SPEC>`
pub type HSEM_RLR1 = crate::Reg<hsem_rlr1::HSEM_RLR1_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr1;
///HSEM_RLR2 (r) register accessor: an alias for `Reg<HSEM_RLR2_SPEC>`
pub type HSEM_RLR2 = crate::Reg<hsem_rlr2::HSEM_RLR2_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr2;
///HSEM_RLR3 (r) register accessor: an alias for `Reg<HSEM_RLR3_SPEC>`
pub type HSEM_RLR3 = crate::Reg<hsem_rlr3::HSEM_RLR3_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr3;
///HSEM_RLR4 (r) register accessor: an alias for `Reg<HSEM_RLR4_SPEC>`
pub type HSEM_RLR4 = crate::Reg<hsem_rlr4::HSEM_RLR4_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr4;
///HSEM_RLR5 (r) register accessor: an alias for `Reg<HSEM_RLR5_SPEC>`
pub type HSEM_RLR5 = crate::Reg<hsem_rlr5::HSEM_RLR5_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr5;
///HSEM_RLR6 (r) register accessor: an alias for `Reg<HSEM_RLR6_SPEC>`
pub type HSEM_RLR6 = crate::Reg<hsem_rlr6::HSEM_RLR6_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr6;
///HSEM_RLR7 (r) register accessor: an alias for `Reg<HSEM_RLR7_SPEC>`
pub type HSEM_RLR7 = crate::Reg<hsem_rlr7::HSEM_RLR7_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr7;
///HSEM_RLR8 (r) register accessor: an alias for `Reg<HSEM_RLR8_SPEC>`
pub type HSEM_RLR8 = crate::Reg<hsem_rlr8::HSEM_RLR8_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr8;
///HSEM_RLR9 (r) register accessor: an alias for `Reg<HSEM_RLR9_SPEC>`
pub type HSEM_RLR9 = crate::Reg<hsem_rlr9::HSEM_RLR9_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr9;
///HSEM_RLR10 (r) register accessor: an alias for `Reg<HSEM_RLR10_SPEC>`
pub type HSEM_RLR10 = crate::Reg<hsem_rlr10::HSEM_RLR10_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr10;
///HSEM_RLR11 (r) register accessor: an alias for `Reg<HSEM_RLR11_SPEC>`
pub type HSEM_RLR11 = crate::Reg<hsem_rlr11::HSEM_RLR11_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr11;
///HSEM_RLR12 (r) register accessor: an alias for `Reg<HSEM_RLR12_SPEC>`
pub type HSEM_RLR12 = crate::Reg<hsem_rlr12::HSEM_RLR12_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr12;
///HSEM_RLR13 (r) register accessor: an alias for `Reg<HSEM_RLR13_SPEC>`
pub type HSEM_RLR13 = crate::Reg<hsem_rlr13::HSEM_RLR13_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr13;
///HSEM_RLR14 (r) register accessor: an alias for `Reg<HSEM_RLR14_SPEC>`
pub type HSEM_RLR14 = crate::Reg<hsem_rlr14::HSEM_RLR14_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr14;
///HSEM_RLR15 (r) register accessor: an alias for `Reg<HSEM_RLR15_SPEC>`
pub type HSEM_RLR15 = crate::Reg<hsem_rlr15::HSEM_RLR15_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr15;
///HSEM_RLR16 (r) register accessor: an alias for `Reg<HSEM_RLR16_SPEC>`
pub type HSEM_RLR16 = crate::Reg<hsem_rlr16::HSEM_RLR16_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr16;
///HSEM_RLR17 (r) register accessor: an alias for `Reg<HSEM_RLR17_SPEC>`
pub type HSEM_RLR17 = crate::Reg<hsem_rlr17::HSEM_RLR17_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr17;
///HSEM_RLR18 (r) register accessor: an alias for `Reg<HSEM_RLR18_SPEC>`
pub type HSEM_RLR18 = crate::Reg<hsem_rlr18::HSEM_RLR18_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr18;
///HSEM_RLR19 (r) register accessor: an alias for `Reg<HSEM_RLR19_SPEC>`
pub type HSEM_RLR19 = crate::Reg<hsem_rlr19::HSEM_RLR19_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr19;
///HSEM_RLR20 (r) register accessor: an alias for `Reg<HSEM_RLR20_SPEC>`
pub type HSEM_RLR20 = crate::Reg<hsem_rlr20::HSEM_RLR20_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr20;
///HSEM_RLR21 (r) register accessor: an alias for `Reg<HSEM_RLR21_SPEC>`
pub type HSEM_RLR21 = crate::Reg<hsem_rlr21::HSEM_RLR21_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr21;
///HSEM_RLR22 (r) register accessor: an alias for `Reg<HSEM_RLR22_SPEC>`
pub type HSEM_RLR22 = crate::Reg<hsem_rlr22::HSEM_RLR22_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr22;
///HSEM_RLR23 (r) register accessor: an alias for `Reg<HSEM_RLR23_SPEC>`
pub type HSEM_RLR23 = crate::Reg<hsem_rlr23::HSEM_RLR23_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr23;
///HSEM_RLR24 (r) register accessor: an alias for `Reg<HSEM_RLR24_SPEC>`
pub type HSEM_RLR24 = crate::Reg<hsem_rlr24::HSEM_RLR24_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr24;
///HSEM_RLR25 (r) register accessor: an alias for `Reg<HSEM_RLR25_SPEC>`
pub type HSEM_RLR25 = crate::Reg<hsem_rlr25::HSEM_RLR25_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr25;
///HSEM_RLR26 (r) register accessor: an alias for `Reg<HSEM_RLR26_SPEC>`
pub type HSEM_RLR26 = crate::Reg<hsem_rlr26::HSEM_RLR26_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr26;
///HSEM_RLR27 (r) register accessor: an alias for `Reg<HSEM_RLR27_SPEC>`
pub type HSEM_RLR27 = crate::Reg<hsem_rlr27::HSEM_RLR27_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr27;
///HSEM_RLR28 (r) register accessor: an alias for `Reg<HSEM_RLR28_SPEC>`
pub type HSEM_RLR28 = crate::Reg<hsem_rlr28::HSEM_RLR28_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr28;
///HSEM_RLR29 (r) register accessor: an alias for `Reg<HSEM_RLR29_SPEC>`
pub type HSEM_RLR29 = crate::Reg<hsem_rlr29::HSEM_RLR29_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr29;
///HSEM_RLR30 (r) register accessor: an alias for `Reg<HSEM_RLR30_SPEC>`
pub type HSEM_RLR30 = crate::Reg<hsem_rlr30::HSEM_RLR30_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr30;
///HSEM_RLR31 (r) register accessor: an alias for `Reg<HSEM_RLR31_SPEC>`
pub type HSEM_RLR31 = crate::Reg<hsem_rlr31::HSEM_RLR31_SPEC>;
///Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_rlr31;
///HSEM_C1IER (rw) register accessor: an alias for `Reg<HSEM_C1IER_SPEC>`
pub type HSEM_C1IER = crate::Reg<hsem_c1ier::HSEM_C1IER_SPEC>;
///HSEM i1terrupt enable register
pub mod hsem_c1ier;
///HSEM_C1ICR (rw) register accessor: an alias for `Reg<HSEM_C1ICR_SPEC>`
pub type HSEM_C1ICR = crate::Reg<hsem_c1icr::HSEM_C1ICR_SPEC>;
///HSEM i1terrupt clear register
pub mod hsem_c1icr;
///HSEM_C1ISR (r) register accessor: an alias for `Reg<HSEM_C1ISR_SPEC>`
pub type HSEM_C1ISR = crate::Reg<hsem_c1isr::HSEM_C1ISR_SPEC>;
///HSEM i1terrupt status register
pub mod hsem_c1isr;
///HSEM_C1MISR (r) register accessor: an alias for `Reg<HSEM_C1MISR_SPEC>`
pub type HSEM_C1MISR = crate::Reg<hsem_c1misr::HSEM_C1MISR_SPEC>;
///HSEM i1terrupt status register
pub mod hsem_c1misr;
///HSEM_C2IER (rw) register accessor: an alias for `Reg<HSEM_C2IER_SPEC>`
pub type HSEM_C2IER = crate::Reg<hsem_c2ier::HSEM_C2IER_SPEC>;
///HSEM i2terrupt enable register
pub mod hsem_c2ier;
///HSEM_C2ICR (rw) register accessor: an alias for `Reg<HSEM_C2ICR_SPEC>`
pub type HSEM_C2ICR = crate::Reg<hsem_c2icr::HSEM_C2ICR_SPEC>;
///HSEM i2terrupt clear register
pub mod hsem_c2icr;
///HSEM_C2ISR (r) register accessor: an alias for `Reg<HSEM_C2ISR_SPEC>`
pub type HSEM_C2ISR = crate::Reg<hsem_c2isr::HSEM_C2ISR_SPEC>;
///HSEM i2terrupt status register
pub mod hsem_c2isr;
///HSEM_C2MISR (r) register accessor: an alias for `Reg<HSEM_C2MISR_SPEC>`
pub type HSEM_C2MISR = crate::Reg<hsem_c2misr::HSEM_C2MISR_SPEC>;
///HSEM i2terrupt status register
pub mod hsem_c2misr;
///HSEM_CR (w) register accessor: an alias for `Reg<HSEM_CR_SPEC>`
pub type HSEM_CR = crate::Reg<hsem_cr::HSEM_CR_SPEC>;
///Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.
pub mod hsem_cr;
///HSEM_KEYR (rw) register accessor: an alias for `Reg<HSEM_KEYR_SPEC>`
pub type HSEM_KEYR = crate::Reg<hsem_keyr::HSEM_KEYR_SPEC>;
///HSEM interrupt clear register
pub mod hsem_keyr;
///HSEM_HWCFGR2 (r) register accessor: an alias for `Reg<HSEM_HWCFGR2_SPEC>`
pub type HSEM_HWCFGR2 = crate::Reg<hsem_hwcfgr2::HSEM_HWCFGR2_SPEC>;
///HSEM hardware configuration register 2
pub mod hsem_hwcfgr2;
///HSEM_HWCFGR1 (r) register accessor: an alias for `Reg<HSEM_HWCFGR1_SPEC>`
pub type HSEM_HWCFGR1 = crate::Reg<hsem_hwcfgr1::HSEM_HWCFGR1_SPEC>;
///HSEM hardware configuration register 1
pub mod hsem_hwcfgr1;
///HSEM_VERR (r) register accessor: an alias for `Reg<HSEM_VERR_SPEC>`
pub type HSEM_VERR = crate::Reg<hsem_verr::HSEM_VERR_SPEC>;
///HSEM IP version register
pub mod hsem_verr;
///HSEM_IPIDR (r) register accessor: an alias for `Reg<HSEM_IPIDR_SPEC>`
pub type HSEM_IPIDR = crate::Reg<hsem_ipidr::HSEM_IPIDR_SPEC>;
///HSEM IP identification register
pub mod hsem_ipidr;
///HSEM_SIDR (r) register accessor: an alias for `Reg<HSEM_SIDR_SPEC>`
pub type HSEM_SIDR = crate::Reg<hsem_sidr::HSEM_SIDR_SPEC>;
///HSEM size identification register
pub mod hsem_sidr;
