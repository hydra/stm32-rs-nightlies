///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GTZC1 SRAM2 MPCBB control register
    pub gtzc1_mpcbb2_cr: GTZC1_MPCBB2_CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - GTZC1 SRAM2 MPCBB configuration lock register 1
    pub gtzc1_mpcbb2_cfglock1: GTZC1_MPCBB2_CFGLOCK1,
    _reserved2: [u8; 0xec],
    ///0x100 - GTZC1 SRAM2 MPCBB security configuration for super-block 0 register
    pub gtzc1_mpcbb2_seccfgr0: GTZC1_MPCBB2_SECCFGR0,
    ///0x104 - GTZC1 SRAM2 MPCBB security configuration for super-block 1 register
    pub gtzc1_mpcbb2_seccfgr1: GTZC1_MPCBB2_SECCFGR1,
    ///0x108 - GTZC1 SRAM2 MPCBB security configuration for super-block 2 register
    pub gtzc1_mpcbb2_seccfgr2: GTZC1_MPCBB2_SECCFGR2,
    ///0x10c - GTZC1 SRAM2 MPCBB security configuration for super-block 3 register
    pub gtzc1_mpcbb2_seccfgr3: GTZC1_MPCBB2_SECCFGR3,
    ///0x110 - GTZC1 SRAM2 MPCBB security configuration for super-block 4 register
    pub gtzc1_mpcbb2_seccfgr4: GTZC1_MPCBB2_SECCFGR4,
    ///0x114 - GTZC1 SRAM2 MPCBB security configuration for super-block 5 register
    pub gtzc1_mpcbb2_seccfgr5: GTZC1_MPCBB2_SECCFGR5,
    ///0x118 - GTZC1 SRAM2 MPCBB security configuration for super-block 6 register
    pub gtzc1_mpcbb2_seccfgr6: GTZC1_MPCBB2_SECCFGR6,
    ///0x11c - GTZC1 SRAM2 MPCBB security configuration for super-block 7 register
    pub gtzc1_mpcbb2_seccfgr7: GTZC1_MPCBB2_SECCFGR7,
    ///0x120 - GTZC1 SRAM2 MPCBB security configuration for super-block 8 register
    pub gtzc1_mpcbb2_seccfgr8: GTZC1_MPCBB2_SECCFGR8,
    ///0x124 - GTZC1 SRAM2 MPCBB security configuration for super-block 9 register
    pub gtzc1_mpcbb2_seccfgr9: GTZC1_MPCBB2_SECCFGR9,
    ///0x128 - GTZC1 SRAM2 MPCBB security configuration for super-block 10 register
    pub gtzc1_mpcbb2_seccfgr10: GTZC1_MPCBB2_SECCFGR10,
    ///0x12c - GTZC1 SRAM2 MPCBB security configuration for super-block 11 register
    pub gtzc1_mpcbb2_seccfgr11: GTZC1_MPCBB2_SECCFGR11,
    ///0x130 - GTZC1 SRAM2 MPCBB security configuration for super-block 12 register
    pub gtzc1_mpcbb2_seccfgr12: GTZC1_MPCBB2_SECCFGR12,
    ///0x134 - GTZC1 SRAM2 MPCBB security configuration for super-block 13 register
    pub gtzc1_mpcbb2_seccfgr13: GTZC1_MPCBB2_SECCFGR13,
    ///0x138 - GTZC1 SRAM2 MPCBB security configuration for super-block 14 register
    pub gtzc1_mpcbb2_seccfgr14: GTZC1_MPCBB2_SECCFGR14,
    ///0x13c - GTZC1 SRAM2 MPCBB security configuration for super-block 15 register
    pub gtzc1_mpcbb2_seccfgr15: GTZC1_MPCBB2_SECCFGR15,
    ///0x140 - GTZC1 SRAM2 MPCBB security configuration for super-block 16 register
    pub gtzc1_mpcbb2_seccfgr16: GTZC1_MPCBB2_SECCFGR16,
    ///0x144 - GTZC1 SRAM2 MPCBB security configuration for super-block 17 register
    pub gtzc1_mpcbb2_seccfgr17: GTZC1_MPCBB2_SECCFGR17,
    ///0x148 - GTZC1 SRAM2 MPCBB security configuration for super-block 18 register
    pub gtzc1_mpcbb2_seccfgr18: GTZC1_MPCBB2_SECCFGR18,
    ///0x14c - GTZC1 SRAM2 MPCBB security configuration for super-block 19 register
    pub gtzc1_mpcbb2_seccfgr19: GTZC1_MPCBB2_SECCFGR19,
    ///0x150 - GTZC1 SRAM2 MPCBB security configuration for super-block 20 register
    pub gtzc1_mpcbb2_seccfgr20: GTZC1_MPCBB2_SECCFGR20,
    ///0x154 - GTZC1 SRAM2 MPCBB security configuration for super-block 21 register
    pub gtzc1_mpcbb2_seccfgr21: GTZC1_MPCBB2_SECCFGR21,
    ///0x158 - GTZC1 SRAM2 MPCBB security configuration for super-block 22 register
    pub gtzc1_mpcbb2_seccfgr22: GTZC1_MPCBB2_SECCFGR22,
    ///0x15c - GTZC1 SRAM2 MPCBB security configuration for super-block 23 register
    pub gtzc1_mpcbb2_seccfgr23: GTZC1_MPCBB2_SECCFGR23,
    ///0x160 - GTZC1 SRAM2 MPCBB security configuration for super-block 24 register
    pub gtzc1_mpcbb2_seccfgr24: GTZC1_MPCBB2_SECCFGR24,
    ///0x164 - GTZC1 SRAM2 MPCBB security configuration for super-block 25 register
    pub gtzc1_mpcbb2_seccfgr25: GTZC1_MPCBB2_SECCFGR25,
    ///0x168 - GTZC1 SRAM2 MPCBB security configuration for super-block 26 register
    pub gtzc1_mpcbb2_seccfgr26: GTZC1_MPCBB2_SECCFGR26,
    ///0x16c - GTZC1 SRAM2 MPCBB security configuration for super-block 27 register
    pub gtzc1_mpcbb2_seccfgr27: GTZC1_MPCBB2_SECCFGR27,
    ///0x170 - GTZC1 SRAM2 MPCBB security configuration for super-block 28 register
    pub gtzc1_mpcbb2_seccfgr28: GTZC1_MPCBB2_SECCFGR28,
    ///0x174 - GTZC1 SRAM2 MPCBB security configuration for super-block 29 register
    pub gtzc1_mpcbb2_seccfgr29: GTZC1_MPCBB2_SECCFGR29,
    ///0x178 - GTZC1 SRAM2 MPCBB security configuration for super-block 30 register
    pub gtzc1_mpcbb2_seccfgr30: GTZC1_MPCBB2_SECCFGR30,
    ///0x17c - GTZC1 SRAM2 MPCBB security configuration for super-block 31 register
    pub gtzc1_mpcbb2_seccfgr31: GTZC1_MPCBB2_SECCFGR31,
    _reserved34: [u8; 0x80],
    ///0x200 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register
    pub gtzc1_mpcbb2_privcfgr0: GTZC1_MPCBB2_PRIVCFGR0,
    ///0x204 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register
    pub gtzc1_mpcbb2_privcfgr1: GTZC1_MPCBB2_PRIVCFGR1,
    ///0x208 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register
    pub gtzc1_mpcbb2_privcfgr2: GTZC1_MPCBB2_PRIVCFGR2,
    ///0x20c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register
    pub gtzc1_mpcbb2_privcfgr3: GTZC1_MPCBB2_PRIVCFGR3,
    ///0x210 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register
    pub gtzc1_mpcbb2_privcfgr4: GTZC1_MPCBB2_PRIVCFGR4,
    ///0x214 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register
    pub gtzc1_mpcbb2_privcfgr5: GTZC1_MPCBB2_PRIVCFGR5,
    ///0x218 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register
    pub gtzc1_mpcbb2_privcfgr6: GTZC1_MPCBB2_PRIVCFGR6,
    ///0x21c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register
    pub gtzc1_mpcbb2_privcfgr7: GTZC1_MPCBB2_PRIVCFGR7,
    ///0x220 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register
    pub gtzc1_mpcbb2_privcfgr8: GTZC1_MPCBB2_PRIVCFGR8,
    ///0x224 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register
    pub gtzc1_mpcbb2_privcfgr9: GTZC1_MPCBB2_PRIVCFGR9,
    ///0x228 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register
    pub gtzc1_mpcbb2_privcfgr10: GTZC1_MPCBB2_PRIVCFGR10,
    ///0x22c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register
    pub gtzc1_mpcbb2_privcfgr11: GTZC1_MPCBB2_PRIVCFGR11,
    ///0x230 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register
    pub gtzc1_mpcbb2_privcfgr12: GTZC1_MPCBB2_PRIVCFGR12,
    ///0x234 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register
    pub gtzc1_mpcbb2_privcfgr13: GTZC1_MPCBB2_PRIVCFGR13,
    ///0x238 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register
    pub gtzc1_mpcbb2_privcfgr14: GTZC1_MPCBB2_PRIVCFGR14,
    ///0x23c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register
    pub gtzc1_mpcbb2_privcfgr15: GTZC1_MPCBB2_PRIVCFGR15,
    ///0x240 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register
    pub gtzc1_mpcbb2_privcfgr16: GTZC1_MPCBB2_PRIVCFGR16,
    ///0x244 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register
    pub gtzc1_mpcbb2_privcfgr17: GTZC1_MPCBB2_PRIVCFGR17,
    ///0x248 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register
    pub gtzc1_mpcbb2_privcfgr18: GTZC1_MPCBB2_PRIVCFGR18,
    ///0x24c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register
    pub gtzc1_mpcbb2_privcfgr19: GTZC1_MPCBB2_PRIVCFGR19,
    ///0x250 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register
    pub gtzc1_mpcbb2_privcfgr20: GTZC1_MPCBB2_PRIVCFGR20,
    ///0x254 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register
    pub gtzc1_mpcbb2_privcfgr21: GTZC1_MPCBB2_PRIVCFGR21,
    ///0x258 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register
    pub gtzc1_mpcbb2_privcfgr22: GTZC1_MPCBB2_PRIVCFGR22,
    ///0x25c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register
    pub gtzc1_mpcbb2_privcfgr23: GTZC1_MPCBB2_PRIVCFGR23,
    ///0x260 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register
    pub gtzc1_mpcbb2_privcfgr24: GTZC1_MPCBB2_PRIVCFGR24,
    ///0x264 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register
    pub gtzc1_mpcbb2_privcfgr25: GTZC1_MPCBB2_PRIVCFGR25,
    ///0x268 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register
    pub gtzc1_mpcbb2_privcfgr26: GTZC1_MPCBB2_PRIVCFGR26,
    ///0x26c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register
    pub gtzc1_mpcbb2_privcfgr27: GTZC1_MPCBB2_PRIVCFGR27,
    ///0x270 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register
    pub gtzc1_mpcbb2_privcfgr28: GTZC1_MPCBB2_PRIVCFGR28,
    ///0x274 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register
    pub gtzc1_mpcbb2_privcfgr29: GTZC1_MPCBB2_PRIVCFGR29,
    ///0x278 - GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register
    pub gtzc1_mpcbb2_privcfgr30: GTZC1_MPCBB2_PRIVCFGR30,
    ///0x27c - GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register
    pub gtzc1_mpcbb2_privcfgr31: GTZC1_MPCBB2_PRIVCFGR31,
}
///GTZC1_MPCBB2_CR (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_CR_SPEC>`
pub type GTZC1_MPCBB2_CR = crate::Reg<gtzc1_mpcbb2_cr::GTZC1_MPCBB2_CR_SPEC>;
///GTZC1 SRAM2 MPCBB control register
pub mod gtzc1_mpcbb2_cr;
///GTZC1_MPCBB2_CFGLOCK1 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_CFGLOCK1_SPEC>`
pub type GTZC1_MPCBB2_CFGLOCK1 = crate::Reg<gtzc1_mpcbb2_cfglock1::GTZC1_MPCBB2_CFGLOCK1_SPEC>;
///GTZC1 SRAM2 MPCBB configuration lock register 1
pub mod gtzc1_mpcbb2_cfglock1;
///GTZC1_MPCBB2_SECCFGR0 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR0_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR0 = crate::Reg<gtzc1_mpcbb2_seccfgr0::GTZC1_MPCBB2_SECCFGR0_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 0 register
pub mod gtzc1_mpcbb2_seccfgr0;
///GTZC1_MPCBB2_SECCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR1_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR1 = crate::Reg<gtzc1_mpcbb2_seccfgr1::GTZC1_MPCBB2_SECCFGR1_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 1 register
pub mod gtzc1_mpcbb2_seccfgr1;
///GTZC1_MPCBB2_SECCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR2_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR2 = crate::Reg<gtzc1_mpcbb2_seccfgr2::GTZC1_MPCBB2_SECCFGR2_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 2 register
pub mod gtzc1_mpcbb2_seccfgr2;
///GTZC1_MPCBB2_SECCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR3_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR3 = crate::Reg<gtzc1_mpcbb2_seccfgr3::GTZC1_MPCBB2_SECCFGR3_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 3 register
pub mod gtzc1_mpcbb2_seccfgr3;
///GTZC1_MPCBB2_SECCFGR4 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR4_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR4 = crate::Reg<gtzc1_mpcbb2_seccfgr4::GTZC1_MPCBB2_SECCFGR4_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 4 register
pub mod gtzc1_mpcbb2_seccfgr4;
///GTZC1_MPCBB2_SECCFGR5 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR5_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR5 = crate::Reg<gtzc1_mpcbb2_seccfgr5::GTZC1_MPCBB2_SECCFGR5_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 5 register
pub mod gtzc1_mpcbb2_seccfgr5;
///GTZC1_MPCBB2_SECCFGR6 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR6_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR6 = crate::Reg<gtzc1_mpcbb2_seccfgr6::GTZC1_MPCBB2_SECCFGR6_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 6 register
pub mod gtzc1_mpcbb2_seccfgr6;
///GTZC1_MPCBB2_SECCFGR7 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR7_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR7 = crate::Reg<gtzc1_mpcbb2_seccfgr7::GTZC1_MPCBB2_SECCFGR7_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 7 register
pub mod gtzc1_mpcbb2_seccfgr7;
///GTZC1_MPCBB2_SECCFGR8 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR8_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR8 = crate::Reg<gtzc1_mpcbb2_seccfgr8::GTZC1_MPCBB2_SECCFGR8_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 8 register
pub mod gtzc1_mpcbb2_seccfgr8;
///GTZC1_MPCBB2_SECCFGR9 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR9_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR9 = crate::Reg<gtzc1_mpcbb2_seccfgr9::GTZC1_MPCBB2_SECCFGR9_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 9 register
pub mod gtzc1_mpcbb2_seccfgr9;
///GTZC1_MPCBB2_SECCFGR10 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR10_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR10 = crate::Reg<gtzc1_mpcbb2_seccfgr10::GTZC1_MPCBB2_SECCFGR10_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 10 register
pub mod gtzc1_mpcbb2_seccfgr10;
///GTZC1_MPCBB2_SECCFGR11 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR11_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR11 = crate::Reg<gtzc1_mpcbb2_seccfgr11::GTZC1_MPCBB2_SECCFGR11_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 11 register
pub mod gtzc1_mpcbb2_seccfgr11;
///GTZC1_MPCBB2_SECCFGR12 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR12_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR12 = crate::Reg<gtzc1_mpcbb2_seccfgr12::GTZC1_MPCBB2_SECCFGR12_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 12 register
pub mod gtzc1_mpcbb2_seccfgr12;
///GTZC1_MPCBB2_SECCFGR13 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR13_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR13 = crate::Reg<gtzc1_mpcbb2_seccfgr13::GTZC1_MPCBB2_SECCFGR13_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 13 register
pub mod gtzc1_mpcbb2_seccfgr13;
///GTZC1_MPCBB2_SECCFGR14 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR14_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR14 = crate::Reg<gtzc1_mpcbb2_seccfgr14::GTZC1_MPCBB2_SECCFGR14_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 14 register
pub mod gtzc1_mpcbb2_seccfgr14;
///GTZC1_MPCBB2_SECCFGR15 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR15_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR15 = crate::Reg<gtzc1_mpcbb2_seccfgr15::GTZC1_MPCBB2_SECCFGR15_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 15 register
pub mod gtzc1_mpcbb2_seccfgr15;
///GTZC1_MPCBB2_SECCFGR16 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR16_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR16 = crate::Reg<gtzc1_mpcbb2_seccfgr16::GTZC1_MPCBB2_SECCFGR16_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 16 register
pub mod gtzc1_mpcbb2_seccfgr16;
///GTZC1_MPCBB2_SECCFGR17 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR17_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR17 = crate::Reg<gtzc1_mpcbb2_seccfgr17::GTZC1_MPCBB2_SECCFGR17_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 17 register
pub mod gtzc1_mpcbb2_seccfgr17;
///GTZC1_MPCBB2_SECCFGR18 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR18_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR18 = crate::Reg<gtzc1_mpcbb2_seccfgr18::GTZC1_MPCBB2_SECCFGR18_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 18 register
pub mod gtzc1_mpcbb2_seccfgr18;
///GTZC1_MPCBB2_SECCFGR19 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR19_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR19 = crate::Reg<gtzc1_mpcbb2_seccfgr19::GTZC1_MPCBB2_SECCFGR19_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 19 register
pub mod gtzc1_mpcbb2_seccfgr19;
///GTZC1_MPCBB2_SECCFGR20 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR20_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR20 = crate::Reg<gtzc1_mpcbb2_seccfgr20::GTZC1_MPCBB2_SECCFGR20_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 20 register
pub mod gtzc1_mpcbb2_seccfgr20;
///GTZC1_MPCBB2_SECCFGR21 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR21_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR21 = crate::Reg<gtzc1_mpcbb2_seccfgr21::GTZC1_MPCBB2_SECCFGR21_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 21 register
pub mod gtzc1_mpcbb2_seccfgr21;
///GTZC1_MPCBB2_SECCFGR22 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR22_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR22 = crate::Reg<gtzc1_mpcbb2_seccfgr22::GTZC1_MPCBB2_SECCFGR22_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 22 register
pub mod gtzc1_mpcbb2_seccfgr22;
///GTZC1_MPCBB2_SECCFGR23 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR23_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR23 = crate::Reg<gtzc1_mpcbb2_seccfgr23::GTZC1_MPCBB2_SECCFGR23_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 23 register
pub mod gtzc1_mpcbb2_seccfgr23;
///GTZC1_MPCBB2_SECCFGR24 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR24_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR24 = crate::Reg<gtzc1_mpcbb2_seccfgr24::GTZC1_MPCBB2_SECCFGR24_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 24 register
pub mod gtzc1_mpcbb2_seccfgr24;
///GTZC1_MPCBB2_SECCFGR25 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR25_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR25 = crate::Reg<gtzc1_mpcbb2_seccfgr25::GTZC1_MPCBB2_SECCFGR25_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 25 register
pub mod gtzc1_mpcbb2_seccfgr25;
///GTZC1_MPCBB2_SECCFGR26 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR26_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR26 = crate::Reg<gtzc1_mpcbb2_seccfgr26::GTZC1_MPCBB2_SECCFGR26_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 26 register
pub mod gtzc1_mpcbb2_seccfgr26;
///GTZC1_MPCBB2_SECCFGR27 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR27_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR27 = crate::Reg<gtzc1_mpcbb2_seccfgr27::GTZC1_MPCBB2_SECCFGR27_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 27 register
pub mod gtzc1_mpcbb2_seccfgr27;
///GTZC1_MPCBB2_SECCFGR28 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR28_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR28 = crate::Reg<gtzc1_mpcbb2_seccfgr28::GTZC1_MPCBB2_SECCFGR28_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 28 register
pub mod gtzc1_mpcbb2_seccfgr28;
///GTZC1_MPCBB2_SECCFGR29 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR29_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR29 = crate::Reg<gtzc1_mpcbb2_seccfgr29::GTZC1_MPCBB2_SECCFGR29_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 29 register
pub mod gtzc1_mpcbb2_seccfgr29;
///GTZC1_MPCBB2_SECCFGR30 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR30_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR30 = crate::Reg<gtzc1_mpcbb2_seccfgr30::GTZC1_MPCBB2_SECCFGR30_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 30 register
pub mod gtzc1_mpcbb2_seccfgr30;
///GTZC1_MPCBB2_SECCFGR31 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_SECCFGR31_SPEC>`
pub type GTZC1_MPCBB2_SECCFGR31 = crate::Reg<gtzc1_mpcbb2_seccfgr31::GTZC1_MPCBB2_SECCFGR31_SPEC>;
///GTZC1 SRAM2 MPCBB security configuration for super-block 31 register
pub mod gtzc1_mpcbb2_seccfgr31;
///GTZC1_MPCBB2_PRIVCFGR0 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR0_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR0 = crate::Reg<gtzc1_mpcbb2_privcfgr0::GTZC1_MPCBB2_PRIVCFGR0_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 0 register
pub mod gtzc1_mpcbb2_privcfgr0;
///GTZC1_MPCBB2_PRIVCFGR1 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR1_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR1 = crate::Reg<gtzc1_mpcbb2_privcfgr1::GTZC1_MPCBB2_PRIVCFGR1_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 1 register
pub mod gtzc1_mpcbb2_privcfgr1;
///GTZC1_MPCBB2_PRIVCFGR2 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR2_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR2 = crate::Reg<gtzc1_mpcbb2_privcfgr2::GTZC1_MPCBB2_PRIVCFGR2_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 2 register
pub mod gtzc1_mpcbb2_privcfgr2;
///GTZC1_MPCBB2_PRIVCFGR3 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR3_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR3 = crate::Reg<gtzc1_mpcbb2_privcfgr3::GTZC1_MPCBB2_PRIVCFGR3_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 3 register
pub mod gtzc1_mpcbb2_privcfgr3;
///GTZC1_MPCBB2_PRIVCFGR4 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR4_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR4 = crate::Reg<gtzc1_mpcbb2_privcfgr4::GTZC1_MPCBB2_PRIVCFGR4_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 4 register
pub mod gtzc1_mpcbb2_privcfgr4;
///GTZC1_MPCBB2_PRIVCFGR5 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR5_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR5 = crate::Reg<gtzc1_mpcbb2_privcfgr5::GTZC1_MPCBB2_PRIVCFGR5_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 5 register
pub mod gtzc1_mpcbb2_privcfgr5;
///GTZC1_MPCBB2_PRIVCFGR6 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR6_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR6 = crate::Reg<gtzc1_mpcbb2_privcfgr6::GTZC1_MPCBB2_PRIVCFGR6_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 6 register
pub mod gtzc1_mpcbb2_privcfgr6;
///GTZC1_MPCBB2_PRIVCFGR7 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR7_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR7 = crate::Reg<gtzc1_mpcbb2_privcfgr7::GTZC1_MPCBB2_PRIVCFGR7_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 7 register
pub mod gtzc1_mpcbb2_privcfgr7;
///GTZC1_MPCBB2_PRIVCFGR8 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR8_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR8 = crate::Reg<gtzc1_mpcbb2_privcfgr8::GTZC1_MPCBB2_PRIVCFGR8_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 8 register
pub mod gtzc1_mpcbb2_privcfgr8;
///GTZC1_MPCBB2_PRIVCFGR9 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR9_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR9 = crate::Reg<gtzc1_mpcbb2_privcfgr9::GTZC1_MPCBB2_PRIVCFGR9_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 9 register
pub mod gtzc1_mpcbb2_privcfgr9;
///GTZC1_MPCBB2_PRIVCFGR10 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR10_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR10 =
    crate::Reg<gtzc1_mpcbb2_privcfgr10::GTZC1_MPCBB2_PRIVCFGR10_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 10 register
pub mod gtzc1_mpcbb2_privcfgr10;
///GTZC1_MPCBB2_PRIVCFGR11 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR11_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR11 =
    crate::Reg<gtzc1_mpcbb2_privcfgr11::GTZC1_MPCBB2_PRIVCFGR11_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 11 register
pub mod gtzc1_mpcbb2_privcfgr11;
///GTZC1_MPCBB2_PRIVCFGR12 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR12_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR12 =
    crate::Reg<gtzc1_mpcbb2_privcfgr12::GTZC1_MPCBB2_PRIVCFGR12_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 12 register
pub mod gtzc1_mpcbb2_privcfgr12;
///GTZC1_MPCBB2_PRIVCFGR13 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR13_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR13 =
    crate::Reg<gtzc1_mpcbb2_privcfgr13::GTZC1_MPCBB2_PRIVCFGR13_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 13 register
pub mod gtzc1_mpcbb2_privcfgr13;
///GTZC1_MPCBB2_PRIVCFGR14 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR14_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR14 =
    crate::Reg<gtzc1_mpcbb2_privcfgr14::GTZC1_MPCBB2_PRIVCFGR14_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 14 register
pub mod gtzc1_mpcbb2_privcfgr14;
///GTZC1_MPCBB2_PRIVCFGR15 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR15_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR15 =
    crate::Reg<gtzc1_mpcbb2_privcfgr15::GTZC1_MPCBB2_PRIVCFGR15_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 15 register
pub mod gtzc1_mpcbb2_privcfgr15;
///GTZC1_MPCBB2_PRIVCFGR16 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR16_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR16 =
    crate::Reg<gtzc1_mpcbb2_privcfgr16::GTZC1_MPCBB2_PRIVCFGR16_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 16 register
pub mod gtzc1_mpcbb2_privcfgr16;
///GTZC1_MPCBB2_PRIVCFGR17 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR17_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR17 =
    crate::Reg<gtzc1_mpcbb2_privcfgr17::GTZC1_MPCBB2_PRIVCFGR17_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 17 register
pub mod gtzc1_mpcbb2_privcfgr17;
///GTZC1_MPCBB2_PRIVCFGR18 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR18_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR18 =
    crate::Reg<gtzc1_mpcbb2_privcfgr18::GTZC1_MPCBB2_PRIVCFGR18_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 18 register
pub mod gtzc1_mpcbb2_privcfgr18;
///GTZC1_MPCBB2_PRIVCFGR19 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR19_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR19 =
    crate::Reg<gtzc1_mpcbb2_privcfgr19::GTZC1_MPCBB2_PRIVCFGR19_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 19 register
pub mod gtzc1_mpcbb2_privcfgr19;
///GTZC1_MPCBB2_PRIVCFGR20 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR20_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR20 =
    crate::Reg<gtzc1_mpcbb2_privcfgr20::GTZC1_MPCBB2_PRIVCFGR20_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 20 register
pub mod gtzc1_mpcbb2_privcfgr20;
///GTZC1_MPCBB2_PRIVCFGR21 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR21_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR21 =
    crate::Reg<gtzc1_mpcbb2_privcfgr21::GTZC1_MPCBB2_PRIVCFGR21_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 21 register
pub mod gtzc1_mpcbb2_privcfgr21;
///GTZC1_MPCBB2_PRIVCFGR22 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR22_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR22 =
    crate::Reg<gtzc1_mpcbb2_privcfgr22::GTZC1_MPCBB2_PRIVCFGR22_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 22 register
pub mod gtzc1_mpcbb2_privcfgr22;
///GTZC1_MPCBB2_PRIVCFGR23 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR23_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR23 =
    crate::Reg<gtzc1_mpcbb2_privcfgr23::GTZC1_MPCBB2_PRIVCFGR23_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 23 register
pub mod gtzc1_mpcbb2_privcfgr23;
///GTZC1_MPCBB2_PRIVCFGR24 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR24_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR24 =
    crate::Reg<gtzc1_mpcbb2_privcfgr24::GTZC1_MPCBB2_PRIVCFGR24_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 24 register
pub mod gtzc1_mpcbb2_privcfgr24;
///GTZC1_MPCBB2_PRIVCFGR25 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR25_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR25 =
    crate::Reg<gtzc1_mpcbb2_privcfgr25::GTZC1_MPCBB2_PRIVCFGR25_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 25 register
pub mod gtzc1_mpcbb2_privcfgr25;
///GTZC1_MPCBB2_PRIVCFGR26 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR26_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR26 =
    crate::Reg<gtzc1_mpcbb2_privcfgr26::GTZC1_MPCBB2_PRIVCFGR26_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 26 register
pub mod gtzc1_mpcbb2_privcfgr26;
///GTZC1_MPCBB2_PRIVCFGR27 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR27_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR27 =
    crate::Reg<gtzc1_mpcbb2_privcfgr27::GTZC1_MPCBB2_PRIVCFGR27_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 27 register
pub mod gtzc1_mpcbb2_privcfgr27;
///GTZC1_MPCBB2_PRIVCFGR28 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR28_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR28 =
    crate::Reg<gtzc1_mpcbb2_privcfgr28::GTZC1_MPCBB2_PRIVCFGR28_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 28 register
pub mod gtzc1_mpcbb2_privcfgr28;
///GTZC1_MPCBB2_PRIVCFGR29 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR29_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR29 =
    crate::Reg<gtzc1_mpcbb2_privcfgr29::GTZC1_MPCBB2_PRIVCFGR29_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 29 register
pub mod gtzc1_mpcbb2_privcfgr29;
///GTZC1_MPCBB2_PRIVCFGR30 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR30_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR30 =
    crate::Reg<gtzc1_mpcbb2_privcfgr30::GTZC1_MPCBB2_PRIVCFGR30_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 30 register
pub mod gtzc1_mpcbb2_privcfgr30;
///GTZC1_MPCBB2_PRIVCFGR31 (rw) register accessor: an alias for `Reg<GTZC1_MPCBB2_PRIVCFGR31_SPEC>`
pub type GTZC1_MPCBB2_PRIVCFGR31 =
    crate::Reg<gtzc1_mpcbb2_privcfgr31::GTZC1_MPCBB2_PRIVCFGR31_SPEC>;
///GTZC1 SRAM2 MPCBB privileged configuration for super-block 31 register
pub mod gtzc1_mpcbb2_privcfgr31;
