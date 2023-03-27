///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MPCBB control register
    pub mpcbb3_cr: MPCBB3_CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - GTZC1 SRAMz MPCBB configuration lock register
    pub mpcbb3_cfglockr1: MPCBB3_CFGLOCKR1,
    _reserved2: [u8; 0xec],
    ///0x100 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr0: MPCBB3_SECCFGR0,
    ///0x104 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr1: MPCBB3_SECCFGR1,
    ///0x108 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr2: MPCBB3_SECCFGR2,
    ///0x10c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr3: MPCBB3_SECCFGR3,
    ///0x110 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr4: MPCBB3_SECCFGR4,
    ///0x114 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr5: MPCBB3_SECCFGR5,
    ///0x118 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr6: MPCBB3_SECCFGR6,
    ///0x11c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr7: MPCBB3_SECCFGR7,
    ///0x120 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr8: MPCBB3_SECCFGR8,
    ///0x124 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr9: MPCBB3_SECCFGR9,
    ///0x128 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr10: MPCBB3_SECCFGR10,
    ///0x12c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr11: MPCBB3_SECCFGR11,
    ///0x130 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr12: MPCBB3_SECCFGR12,
    ///0x134 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr13: MPCBB3_SECCFGR13,
    ///0x138 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr14: MPCBB3_SECCFGR14,
    ///0x13c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr15: MPCBB3_SECCFGR15,
    ///0x140 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr16: MPCBB3_SECCFGR16,
    ///0x144 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr17: MPCBB3_SECCFGR17,
    ///0x148 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr18: MPCBB3_SECCFGR18,
    ///0x14c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr19: MPCBB3_SECCFGR19,
    ///0x150 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr20: MPCBB3_SECCFGR20,
    ///0x154 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr21: MPCBB3_SECCFGR21,
    ///0x158 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr22: MPCBB3_SECCFGR22,
    ///0x15c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr23: MPCBB3_SECCFGR23,
    ///0x160 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr24: MPCBB3_SECCFGR24,
    ///0x164 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr25: MPCBB3_SECCFGR25,
    ///0x168 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr26: MPCBB3_SECCFGR26,
    ///0x16c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr27: MPCBB3_SECCFGR27,
    ///0x170 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr28: MPCBB3_SECCFGR28,
    ///0x174 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr29: MPCBB3_SECCFGR29,
    ///0x178 - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr30: MPCBB3_SECCFGR30,
    ///0x17c - MPCBBx security configuration for super-block x register
    pub mpcbb3_seccfgr31: MPCBB3_SECCFGR31,
    _reserved34: [u8; 0x80],
    ///0x200 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr0: MPCBB3_PRIVCFGR0,
    ///0x204 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr1: MPCBB3_PRIVCFGR1,
    ///0x208 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr2: MPCBB3_PRIVCFGR2,
    ///0x20c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr3: MPCBB3_PRIVCFGR3,
    ///0x210 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr4: MPCBB3_PRIVCFGR4,
    ///0x214 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr5: MPCBB3_PRIVCFGR5,
    ///0x218 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr6: MPCBB3_PRIVCFGR6,
    ///0x21c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr7: MPCBB3_PRIVCFGR7,
    ///0x220 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr8: MPCBB3_PRIVCFGR8,
    ///0x224 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr9: MPCBB3_PRIVCFGR9,
    ///0x228 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr10: MPCBB3_PRIVCFGR10,
    ///0x22c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr11: MPCBB3_PRIVCFGR11,
    ///0x230 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr12: MPCBB3_PRIVCFGR12,
    ///0x234 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr13: MPCBB3_PRIVCFGR13,
    ///0x238 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr14: MPCBB3_PRIVCFGR14,
    ///0x23c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr15: MPCBB3_PRIVCFGR15,
    ///0x240 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr16: MPCBB3_PRIVCFGR16,
    ///0x244 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr17: MPCBB3_PRIVCFGR17,
    ///0x248 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr18: MPCBB3_PRIVCFGR18,
    ///0x24c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr19: MPCBB3_PRIVCFGR19,
    ///0x250 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr20: MPCBB3_PRIVCFGR20,
    ///0x254 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr21: MPCBB3_PRIVCFGR21,
    ///0x258 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr22: MPCBB3_PRIVCFGR22,
    ///0x25c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr23: MPCBB3_PRIVCFGR23,
    ///0x260 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr24: MPCBB3_PRIVCFGR24,
    ///0x264 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr25: MPCBB3_PRIVCFGR25,
    ///0x268 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr26: MPCBB3_PRIVCFGR26,
    ///0x26c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr27: MPCBB3_PRIVCFGR27,
    ///0x270 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr28: MPCBB3_PRIVCFGR28,
    ///0x274 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr29: MPCBB3_PRIVCFGR29,
    ///0x278 - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr30: MPCBB3_PRIVCFGR30,
    ///0x27c - MPCBB privileged configuration for super-block x register
    pub mpcbb3_privcfgr31: MPCBB3_PRIVCFGR31,
}
///MPCBB3_CR (rw) register accessor: an alias for `Reg<MPCBB3_CR_SPEC>`
pub type MPCBB3_CR = crate::Reg<mpcbb3_cr::MPCBB3_CR_SPEC>;
///MPCBB control register
pub mod mpcbb3_cr;
///MPCBB3_CFGLOCKR1 (rw) register accessor: an alias for `Reg<MPCBB3_CFGLOCKR1_SPEC>`
pub type MPCBB3_CFGLOCKR1 = crate::Reg<mpcbb3_cfglockr1::MPCBB3_CFGLOCKR1_SPEC>;
///GTZC1 SRAMz MPCBB configuration lock register
pub mod mpcbb3_cfglockr1;
///MPCBB3_SECCFGR0 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR0_SPEC>`
pub type MPCBB3_SECCFGR0 = crate::Reg<mpcbb3_seccfgr0::MPCBB3_SECCFGR0_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr0;
///MPCBB3_SECCFGR1 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR1_SPEC>`
pub type MPCBB3_SECCFGR1 = crate::Reg<mpcbb3_seccfgr1::MPCBB3_SECCFGR1_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr1;
///MPCBB3_SECCFGR2 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR2_SPEC>`
pub type MPCBB3_SECCFGR2 = crate::Reg<mpcbb3_seccfgr2::MPCBB3_SECCFGR2_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr2;
///MPCBB3_SECCFGR3 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR3_SPEC>`
pub type MPCBB3_SECCFGR3 = crate::Reg<mpcbb3_seccfgr3::MPCBB3_SECCFGR3_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr3;
///MPCBB3_SECCFGR4 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR4_SPEC>`
pub type MPCBB3_SECCFGR4 = crate::Reg<mpcbb3_seccfgr4::MPCBB3_SECCFGR4_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr4;
///MPCBB3_SECCFGR5 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR5_SPEC>`
pub type MPCBB3_SECCFGR5 = crate::Reg<mpcbb3_seccfgr5::MPCBB3_SECCFGR5_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr5;
///MPCBB3_SECCFGR6 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR6_SPEC>`
pub type MPCBB3_SECCFGR6 = crate::Reg<mpcbb3_seccfgr6::MPCBB3_SECCFGR6_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr6;
///MPCBB3_SECCFGR7 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR7_SPEC>`
pub type MPCBB3_SECCFGR7 = crate::Reg<mpcbb3_seccfgr7::MPCBB3_SECCFGR7_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr7;
///MPCBB3_SECCFGR8 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR8_SPEC>`
pub type MPCBB3_SECCFGR8 = crate::Reg<mpcbb3_seccfgr8::MPCBB3_SECCFGR8_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr8;
///MPCBB3_SECCFGR9 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR9_SPEC>`
pub type MPCBB3_SECCFGR9 = crate::Reg<mpcbb3_seccfgr9::MPCBB3_SECCFGR9_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr9;
///MPCBB3_SECCFGR10 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR10_SPEC>`
pub type MPCBB3_SECCFGR10 = crate::Reg<mpcbb3_seccfgr10::MPCBB3_SECCFGR10_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr10;
///MPCBB3_SECCFGR11 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR11_SPEC>`
pub type MPCBB3_SECCFGR11 = crate::Reg<mpcbb3_seccfgr11::MPCBB3_SECCFGR11_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr11;
///MPCBB3_SECCFGR12 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR12_SPEC>`
pub type MPCBB3_SECCFGR12 = crate::Reg<mpcbb3_seccfgr12::MPCBB3_SECCFGR12_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr12;
///MPCBB3_SECCFGR13 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR13_SPEC>`
pub type MPCBB3_SECCFGR13 = crate::Reg<mpcbb3_seccfgr13::MPCBB3_SECCFGR13_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr13;
///MPCBB3_SECCFGR14 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR14_SPEC>`
pub type MPCBB3_SECCFGR14 = crate::Reg<mpcbb3_seccfgr14::MPCBB3_SECCFGR14_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr14;
///MPCBB3_SECCFGR15 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR15_SPEC>`
pub type MPCBB3_SECCFGR15 = crate::Reg<mpcbb3_seccfgr15::MPCBB3_SECCFGR15_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr15;
///MPCBB3_SECCFGR16 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR16_SPEC>`
pub type MPCBB3_SECCFGR16 = crate::Reg<mpcbb3_seccfgr16::MPCBB3_SECCFGR16_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr16;
///MPCBB3_SECCFGR17 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR17_SPEC>`
pub type MPCBB3_SECCFGR17 = crate::Reg<mpcbb3_seccfgr17::MPCBB3_SECCFGR17_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr17;
///MPCBB3_SECCFGR18 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR18_SPEC>`
pub type MPCBB3_SECCFGR18 = crate::Reg<mpcbb3_seccfgr18::MPCBB3_SECCFGR18_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr18;
///MPCBB3_SECCFGR19 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR19_SPEC>`
pub type MPCBB3_SECCFGR19 = crate::Reg<mpcbb3_seccfgr19::MPCBB3_SECCFGR19_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr19;
///MPCBB3_SECCFGR20 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR20_SPEC>`
pub type MPCBB3_SECCFGR20 = crate::Reg<mpcbb3_seccfgr20::MPCBB3_SECCFGR20_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr20;
///MPCBB3_SECCFGR21 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR21_SPEC>`
pub type MPCBB3_SECCFGR21 = crate::Reg<mpcbb3_seccfgr21::MPCBB3_SECCFGR21_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr21;
///MPCBB3_SECCFGR22 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR22_SPEC>`
pub type MPCBB3_SECCFGR22 = crate::Reg<mpcbb3_seccfgr22::MPCBB3_SECCFGR22_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr22;
///MPCBB3_SECCFGR23 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR23_SPEC>`
pub type MPCBB3_SECCFGR23 = crate::Reg<mpcbb3_seccfgr23::MPCBB3_SECCFGR23_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr23;
///MPCBB3_SECCFGR24 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR24_SPEC>`
pub type MPCBB3_SECCFGR24 = crate::Reg<mpcbb3_seccfgr24::MPCBB3_SECCFGR24_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr24;
///MPCBB3_SECCFGR25 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR25_SPEC>`
pub type MPCBB3_SECCFGR25 = crate::Reg<mpcbb3_seccfgr25::MPCBB3_SECCFGR25_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr25;
///MPCBB3_SECCFGR26 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR26_SPEC>`
pub type MPCBB3_SECCFGR26 = crate::Reg<mpcbb3_seccfgr26::MPCBB3_SECCFGR26_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr26;
///MPCBB3_SECCFGR27 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR27_SPEC>`
pub type MPCBB3_SECCFGR27 = crate::Reg<mpcbb3_seccfgr27::MPCBB3_SECCFGR27_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr27;
///MPCBB3_SECCFGR28 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR28_SPEC>`
pub type MPCBB3_SECCFGR28 = crate::Reg<mpcbb3_seccfgr28::MPCBB3_SECCFGR28_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr28;
///MPCBB3_SECCFGR29 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR29_SPEC>`
pub type MPCBB3_SECCFGR29 = crate::Reg<mpcbb3_seccfgr29::MPCBB3_SECCFGR29_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr29;
///MPCBB3_SECCFGR30 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR30_SPEC>`
pub type MPCBB3_SECCFGR30 = crate::Reg<mpcbb3_seccfgr30::MPCBB3_SECCFGR30_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr30;
///MPCBB3_SECCFGR31 (rw) register accessor: an alias for `Reg<MPCBB3_SECCFGR31_SPEC>`
pub type MPCBB3_SECCFGR31 = crate::Reg<mpcbb3_seccfgr31::MPCBB3_SECCFGR31_SPEC>;
///MPCBBx security configuration for super-block x register
pub mod mpcbb3_seccfgr31;
///MPCBB3_PRIVCFGR0 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR0_SPEC>`
pub type MPCBB3_PRIVCFGR0 = crate::Reg<mpcbb3_privcfgr0::MPCBB3_PRIVCFGR0_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr0;
///MPCBB3_PRIVCFGR1 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR1_SPEC>`
pub type MPCBB3_PRIVCFGR1 = crate::Reg<mpcbb3_privcfgr1::MPCBB3_PRIVCFGR1_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr1;
///MPCBB3_PRIVCFGR2 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR2_SPEC>`
pub type MPCBB3_PRIVCFGR2 = crate::Reg<mpcbb3_privcfgr2::MPCBB3_PRIVCFGR2_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr2;
///MPCBB3_PRIVCFGR3 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR3_SPEC>`
pub type MPCBB3_PRIVCFGR3 = crate::Reg<mpcbb3_privcfgr3::MPCBB3_PRIVCFGR3_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr3;
///MPCBB3_PRIVCFGR4 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR4_SPEC>`
pub type MPCBB3_PRIVCFGR4 = crate::Reg<mpcbb3_privcfgr4::MPCBB3_PRIVCFGR4_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr4;
///MPCBB3_PRIVCFGR5 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR5_SPEC>`
pub type MPCBB3_PRIVCFGR5 = crate::Reg<mpcbb3_privcfgr5::MPCBB3_PRIVCFGR5_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr5;
///MPCBB3_PRIVCFGR6 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR6_SPEC>`
pub type MPCBB3_PRIVCFGR6 = crate::Reg<mpcbb3_privcfgr6::MPCBB3_PRIVCFGR6_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr6;
///MPCBB3_PRIVCFGR7 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR7_SPEC>`
pub type MPCBB3_PRIVCFGR7 = crate::Reg<mpcbb3_privcfgr7::MPCBB3_PRIVCFGR7_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr7;
///MPCBB3_PRIVCFGR8 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR8_SPEC>`
pub type MPCBB3_PRIVCFGR8 = crate::Reg<mpcbb3_privcfgr8::MPCBB3_PRIVCFGR8_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr8;
///MPCBB3_PRIVCFGR9 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR9_SPEC>`
pub type MPCBB3_PRIVCFGR9 = crate::Reg<mpcbb3_privcfgr9::MPCBB3_PRIVCFGR9_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr9;
///MPCBB3_PRIVCFGR10 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR10_SPEC>`
pub type MPCBB3_PRIVCFGR10 = crate::Reg<mpcbb3_privcfgr10::MPCBB3_PRIVCFGR10_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr10;
///MPCBB3_PRIVCFGR11 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR11_SPEC>`
pub type MPCBB3_PRIVCFGR11 = crate::Reg<mpcbb3_privcfgr11::MPCBB3_PRIVCFGR11_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr11;
///MPCBB3_PRIVCFGR12 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR12_SPEC>`
pub type MPCBB3_PRIVCFGR12 = crate::Reg<mpcbb3_privcfgr12::MPCBB3_PRIVCFGR12_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr12;
///MPCBB3_PRIVCFGR13 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR13_SPEC>`
pub type MPCBB3_PRIVCFGR13 = crate::Reg<mpcbb3_privcfgr13::MPCBB3_PRIVCFGR13_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr13;
///MPCBB3_PRIVCFGR14 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR14_SPEC>`
pub type MPCBB3_PRIVCFGR14 = crate::Reg<mpcbb3_privcfgr14::MPCBB3_PRIVCFGR14_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr14;
///MPCBB3_PRIVCFGR15 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR15_SPEC>`
pub type MPCBB3_PRIVCFGR15 = crate::Reg<mpcbb3_privcfgr15::MPCBB3_PRIVCFGR15_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr15;
///MPCBB3_PRIVCFGR16 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR16_SPEC>`
pub type MPCBB3_PRIVCFGR16 = crate::Reg<mpcbb3_privcfgr16::MPCBB3_PRIVCFGR16_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr16;
///MPCBB3_PRIVCFGR17 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR17_SPEC>`
pub type MPCBB3_PRIVCFGR17 = crate::Reg<mpcbb3_privcfgr17::MPCBB3_PRIVCFGR17_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr17;
///MPCBB3_PRIVCFGR18 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR18_SPEC>`
pub type MPCBB3_PRIVCFGR18 = crate::Reg<mpcbb3_privcfgr18::MPCBB3_PRIVCFGR18_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr18;
///MPCBB3_PRIVCFGR19 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR19_SPEC>`
pub type MPCBB3_PRIVCFGR19 = crate::Reg<mpcbb3_privcfgr19::MPCBB3_PRIVCFGR19_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr19;
///MPCBB3_PRIVCFGR20 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR20_SPEC>`
pub type MPCBB3_PRIVCFGR20 = crate::Reg<mpcbb3_privcfgr20::MPCBB3_PRIVCFGR20_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr20;
///MPCBB3_PRIVCFGR21 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR21_SPEC>`
pub type MPCBB3_PRIVCFGR21 = crate::Reg<mpcbb3_privcfgr21::MPCBB3_PRIVCFGR21_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr21;
///MPCBB3_PRIVCFGR22 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR22_SPEC>`
pub type MPCBB3_PRIVCFGR22 = crate::Reg<mpcbb3_privcfgr22::MPCBB3_PRIVCFGR22_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr22;
///MPCBB3_PRIVCFGR23 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR23_SPEC>`
pub type MPCBB3_PRIVCFGR23 = crate::Reg<mpcbb3_privcfgr23::MPCBB3_PRIVCFGR23_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr23;
///MPCBB3_PRIVCFGR24 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR24_SPEC>`
pub type MPCBB3_PRIVCFGR24 = crate::Reg<mpcbb3_privcfgr24::MPCBB3_PRIVCFGR24_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr24;
///MPCBB3_PRIVCFGR25 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR25_SPEC>`
pub type MPCBB3_PRIVCFGR25 = crate::Reg<mpcbb3_privcfgr25::MPCBB3_PRIVCFGR25_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr25;
///MPCBB3_PRIVCFGR26 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR26_SPEC>`
pub type MPCBB3_PRIVCFGR26 = crate::Reg<mpcbb3_privcfgr26::MPCBB3_PRIVCFGR26_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr26;
///MPCBB3_PRIVCFGR27 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR27_SPEC>`
pub type MPCBB3_PRIVCFGR27 = crate::Reg<mpcbb3_privcfgr27::MPCBB3_PRIVCFGR27_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr27;
///MPCBB3_PRIVCFGR28 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR28_SPEC>`
pub type MPCBB3_PRIVCFGR28 = crate::Reg<mpcbb3_privcfgr28::MPCBB3_PRIVCFGR28_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr28;
///MPCBB3_PRIVCFGR29 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR29_SPEC>`
pub type MPCBB3_PRIVCFGR29 = crate::Reg<mpcbb3_privcfgr29::MPCBB3_PRIVCFGR29_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr29;
///MPCBB3_PRIVCFGR30 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR30_SPEC>`
pub type MPCBB3_PRIVCFGR30 = crate::Reg<mpcbb3_privcfgr30::MPCBB3_PRIVCFGR30_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr30;
///MPCBB3_PRIVCFGR31 (rw) register accessor: an alias for `Reg<MPCBB3_PRIVCFGR31_SPEC>`
pub type MPCBB3_PRIVCFGR31 = crate::Reg<mpcbb3_privcfgr31::MPCBB3_PRIVCFGR31_SPEC>;
///MPCBB privileged configuration for super-block x register
pub mod mpcbb3_privcfgr31;
