///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MPCBB control register
    pub cr: CR,
    _reserved1: [u8; 0x0c],
    ///0x10 - MPCBB control register
    pub lckvtr1: LCKVTR1,
    ///0x14 - MPCBB control register
    pub lckvtr2: LCKVTR2,
    _reserved3: [u8; 0xe8],
    ///0x100 - MPCBBx vector register
    pub vctr0: VCTR0,
    ///0x104 - MPCBBx vector register
    pub vctr1: VCTR1,
    ///0x108 - MPCBBx vector register
    pub vctr2: VCTR2,
    ///0x10c - MPCBBx vector register
    pub vctr3: VCTR3,
    ///0x110 - MPCBBx vector register
    pub vctr4: VCTR4,
    ///0x114 - MPCBBx vector register
    pub vctr5: VCTR5,
    ///0x118 - MPCBBx vector register
    pub vctr6: VCTR6,
    ///0x11c - MPCBBx vector register
    pub vctr7: VCTR7,
    ///0x120 - MPCBBx vector register
    pub vctr8: VCTR8,
    ///0x124 - MPCBBx vector register
    pub vctr9: VCTR9,
    ///0x128 - MPCBBx vector register
    pub vctr10: VCTR10,
    ///0x12c - MPCBBx vector register
    pub vctr11: VCTR11,
    ///0x130 - MPCBBx vector register
    pub vctr12: VCTR12,
    ///0x134 - MPCBBx vector register
    pub vctr13: VCTR13,
    ///0x138 - MPCBBx vector register
    pub vctr14: VCTR14,
    ///0x13c - MPCBBx vector register
    pub vctr15: VCTR15,
    ///0x140 - MPCBBx vector register
    pub vctr16: VCTR16,
    ///0x144 - MPCBBx vector register
    pub vctr17: VCTR17,
    ///0x148 - MPCBBx vector register
    pub vctr18: VCTR18,
    ///0x14c - MPCBBx vector register
    pub vctr19: VCTR19,
    ///0x150 - MPCBBx vector register
    pub vctr20: VCTR20,
    ///0x154 - MPCBBx vector register
    pub vctr21: VCTR21,
    ///0x158 - MPCBBx vector register
    pub vctr22: VCTR22,
    ///0x15c - MPCBBx vector register
    pub vctr23: VCTR23,
    ///0x160 - MPCBBx vector register
    pub vctr24: VCTR24,
    ///0x164 - MPCBBx vector register
    pub vctr25: VCTR25,
    ///0x168 - MPCBBx vector register
    pub vctr26: VCTR26,
    ///0x16c - MPCBBx vector register
    pub vctr27: VCTR27,
    ///0x170 - MPCBBx vector register
    pub vctr28: VCTR28,
    ///0x174 - MPCBBx vector register
    pub vctr29: VCTR29,
    ///0x178 - MPCBBx vector register
    pub vctr30: VCTR30,
    ///0x17c - MPCBBx vector register
    pub vctr31: VCTR31,
    ///0x180 - MPCBBx vector register
    pub vctr32: VCTR32,
    ///0x184 - MPCBBx vector register
    pub vctr33: VCTR33,
    ///0x188 - MPCBBx vector register
    pub vctr34: VCTR34,
    ///0x18c - MPCBBx vector register
    pub vctr35: VCTR35,
    ///0x190 - MPCBBx vector register
    pub vctr36: VCTR36,
    ///0x194 - MPCBBx vector register
    pub vctr37: VCTR37,
    ///0x198 - MPCBBx vector register
    pub vctr38: VCTR38,
    ///0x19c - MPCBBx vector register
    pub vctr39: VCTR39,
    ///0x1a0 - MPCBBx vector register
    pub vctr40: VCTR40,
    ///0x1a4 - MPCBBx vector register
    pub vctr41: VCTR41,
    ///0x1a8 - MPCBBx vector register
    pub vctr42: VCTR42,
    ///0x1ac - MPCBBx vector register
    pub vctr43: VCTR43,
    ///0x1b0 - MPCBBx vector register
    pub vctr44: VCTR44,
    ///0x1b4 - MPCBBx vector register
    pub vctr45: VCTR45,
    ///0x1b8 - MPCBBx vector register
    pub vctr46: VCTR46,
    ///0x1bc - MPCBBx vector register
    pub vctr47: VCTR47,
    ///0x1c0 - MPCBBx vector register
    pub vctr48: VCTR48,
    ///0x1c4 - MPCBBx vector register
    pub vctr49: VCTR49,
    ///0x1c8 - MPCBBx vector register
    pub vctr50: VCTR50,
    ///0x1cc - MPCBBx vector register
    pub vctr51: VCTR51,
    ///0x1d0 - MPCBBx vector register
    pub vctr52: VCTR52,
    ///0x1d4 - MPCBBx vector register
    pub vctr53: VCTR53,
    ///0x1d8 - MPCBBx vector register
    pub vctr54: VCTR54,
    ///0x1dc - MPCBBx vector register
    pub vctr55: VCTR55,
    ///0x1e0 - MPCBBx vector register
    pub vctr56: VCTR56,
    ///0x1e4 - MPCBBx vector register
    pub vctr57: VCTR57,
    ///0x1e8 - MPCBBx vector register
    pub vctr58: VCTR58,
    ///0x1ec - MPCBBx vector register
    pub vctr59: VCTR59,
    ///0x1f0 - MPCBBx vector register
    pub vctr60: VCTR60,
    ///0x1f4 - MPCBBx vector register
    pub vctr61: VCTR61,
    ///0x1f8 - MPCBBx vector register
    pub vctr62: VCTR62,
    ///0x1fc - MPCBBx vector register
    pub vctr63: VCTR63,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///MPCBB control register
pub mod cr;
///LCKVTR1 (rw) register accessor: an alias for `Reg<LCKVTR1_SPEC>`
pub type LCKVTR1 = crate::Reg<lckvtr1::LCKVTR1_SPEC>;
///MPCBB control register
pub mod lckvtr1;
///LCKVTR2 (rw) register accessor: an alias for `Reg<LCKVTR2_SPEC>`
pub type LCKVTR2 = crate::Reg<lckvtr2::LCKVTR2_SPEC>;
///MPCBB control register
pub mod lckvtr2;
///VCTR0 (rw) register accessor: an alias for `Reg<VCTR0_SPEC>`
pub type VCTR0 = crate::Reg<vctr0::VCTR0_SPEC>;
///MPCBBx vector register
pub mod vctr0;
///VCTR1 (rw) register accessor: an alias for `Reg<VCTR1_SPEC>`
pub type VCTR1 = crate::Reg<vctr1::VCTR1_SPEC>;
///MPCBBx vector register
pub mod vctr1;
///VCTR2 (rw) register accessor: an alias for `Reg<VCTR2_SPEC>`
pub type VCTR2 = crate::Reg<vctr2::VCTR2_SPEC>;
///MPCBBx vector register
pub mod vctr2;
///VCTR3 (rw) register accessor: an alias for `Reg<VCTR3_SPEC>`
pub type VCTR3 = crate::Reg<vctr3::VCTR3_SPEC>;
///MPCBBx vector register
pub mod vctr3;
///VCTR4 (rw) register accessor: an alias for `Reg<VCTR4_SPEC>`
pub type VCTR4 = crate::Reg<vctr4::VCTR4_SPEC>;
///MPCBBx vector register
pub mod vctr4;
///VCTR5 (rw) register accessor: an alias for `Reg<VCTR5_SPEC>`
pub type VCTR5 = crate::Reg<vctr5::VCTR5_SPEC>;
///MPCBBx vector register
pub mod vctr5;
///VCTR6 (rw) register accessor: an alias for `Reg<VCTR6_SPEC>`
pub type VCTR6 = crate::Reg<vctr6::VCTR6_SPEC>;
///MPCBBx vector register
pub mod vctr6;
///VCTR7 (rw) register accessor: an alias for `Reg<VCTR7_SPEC>`
pub type VCTR7 = crate::Reg<vctr7::VCTR7_SPEC>;
///MPCBBx vector register
pub mod vctr7;
///VCTR8 (rw) register accessor: an alias for `Reg<VCTR8_SPEC>`
pub type VCTR8 = crate::Reg<vctr8::VCTR8_SPEC>;
///MPCBBx vector register
pub mod vctr8;
///VCTR9 (rw) register accessor: an alias for `Reg<VCTR9_SPEC>`
pub type VCTR9 = crate::Reg<vctr9::VCTR9_SPEC>;
///MPCBBx vector register
pub mod vctr9;
///VCTR10 (rw) register accessor: an alias for `Reg<VCTR10_SPEC>`
pub type VCTR10 = crate::Reg<vctr10::VCTR10_SPEC>;
///MPCBBx vector register
pub mod vctr10;
///VCTR11 (rw) register accessor: an alias for `Reg<VCTR11_SPEC>`
pub type VCTR11 = crate::Reg<vctr11::VCTR11_SPEC>;
///MPCBBx vector register
pub mod vctr11;
///VCTR12 (rw) register accessor: an alias for `Reg<VCTR12_SPEC>`
pub type VCTR12 = crate::Reg<vctr12::VCTR12_SPEC>;
///MPCBBx vector register
pub mod vctr12;
///VCTR13 (rw) register accessor: an alias for `Reg<VCTR13_SPEC>`
pub type VCTR13 = crate::Reg<vctr13::VCTR13_SPEC>;
///MPCBBx vector register
pub mod vctr13;
///VCTR14 (rw) register accessor: an alias for `Reg<VCTR14_SPEC>`
pub type VCTR14 = crate::Reg<vctr14::VCTR14_SPEC>;
///MPCBBx vector register
pub mod vctr14;
///VCTR15 (rw) register accessor: an alias for `Reg<VCTR15_SPEC>`
pub type VCTR15 = crate::Reg<vctr15::VCTR15_SPEC>;
///MPCBBx vector register
pub mod vctr15;
///VCTR16 (rw) register accessor: an alias for `Reg<VCTR16_SPEC>`
pub type VCTR16 = crate::Reg<vctr16::VCTR16_SPEC>;
///MPCBBx vector register
pub mod vctr16;
///VCTR17 (rw) register accessor: an alias for `Reg<VCTR17_SPEC>`
pub type VCTR17 = crate::Reg<vctr17::VCTR17_SPEC>;
///MPCBBx vector register
pub mod vctr17;
///VCTR18 (rw) register accessor: an alias for `Reg<VCTR18_SPEC>`
pub type VCTR18 = crate::Reg<vctr18::VCTR18_SPEC>;
///MPCBBx vector register
pub mod vctr18;
///VCTR19 (rw) register accessor: an alias for `Reg<VCTR19_SPEC>`
pub type VCTR19 = crate::Reg<vctr19::VCTR19_SPEC>;
///MPCBBx vector register
pub mod vctr19;
///VCTR20 (rw) register accessor: an alias for `Reg<VCTR20_SPEC>`
pub type VCTR20 = crate::Reg<vctr20::VCTR20_SPEC>;
///MPCBBx vector register
pub mod vctr20;
///VCTR21 (rw) register accessor: an alias for `Reg<VCTR21_SPEC>`
pub type VCTR21 = crate::Reg<vctr21::VCTR21_SPEC>;
///MPCBBx vector register
pub mod vctr21;
///VCTR22 (rw) register accessor: an alias for `Reg<VCTR22_SPEC>`
pub type VCTR22 = crate::Reg<vctr22::VCTR22_SPEC>;
///MPCBBx vector register
pub mod vctr22;
///VCTR23 (rw) register accessor: an alias for `Reg<VCTR23_SPEC>`
pub type VCTR23 = crate::Reg<vctr23::VCTR23_SPEC>;
///MPCBBx vector register
pub mod vctr23;
///VCTR24 (rw) register accessor: an alias for `Reg<VCTR24_SPEC>`
pub type VCTR24 = crate::Reg<vctr24::VCTR24_SPEC>;
///MPCBBx vector register
pub mod vctr24;
///VCTR25 (rw) register accessor: an alias for `Reg<VCTR25_SPEC>`
pub type VCTR25 = crate::Reg<vctr25::VCTR25_SPEC>;
///MPCBBx vector register
pub mod vctr25;
///VCTR26 (rw) register accessor: an alias for `Reg<VCTR26_SPEC>`
pub type VCTR26 = crate::Reg<vctr26::VCTR26_SPEC>;
///MPCBBx vector register
pub mod vctr26;
///VCTR27 (rw) register accessor: an alias for `Reg<VCTR27_SPEC>`
pub type VCTR27 = crate::Reg<vctr27::VCTR27_SPEC>;
///MPCBBx vector register
pub mod vctr27;
///VCTR28 (rw) register accessor: an alias for `Reg<VCTR28_SPEC>`
pub type VCTR28 = crate::Reg<vctr28::VCTR28_SPEC>;
///MPCBBx vector register
pub mod vctr28;
///VCTR29 (rw) register accessor: an alias for `Reg<VCTR29_SPEC>`
pub type VCTR29 = crate::Reg<vctr29::VCTR29_SPEC>;
///MPCBBx vector register
pub mod vctr29;
///VCTR30 (rw) register accessor: an alias for `Reg<VCTR30_SPEC>`
pub type VCTR30 = crate::Reg<vctr30::VCTR30_SPEC>;
///MPCBBx vector register
pub mod vctr30;
///VCTR31 (rw) register accessor: an alias for `Reg<VCTR31_SPEC>`
pub type VCTR31 = crate::Reg<vctr31::VCTR31_SPEC>;
///MPCBBx vector register
pub mod vctr31;
///VCTR32 (rw) register accessor: an alias for `Reg<VCTR32_SPEC>`
pub type VCTR32 = crate::Reg<vctr32::VCTR32_SPEC>;
///MPCBBx vector register
pub mod vctr32;
///VCTR33 (rw) register accessor: an alias for `Reg<VCTR33_SPEC>`
pub type VCTR33 = crate::Reg<vctr33::VCTR33_SPEC>;
///MPCBBx vector register
pub mod vctr33;
///VCTR34 (rw) register accessor: an alias for `Reg<VCTR34_SPEC>`
pub type VCTR34 = crate::Reg<vctr34::VCTR34_SPEC>;
///MPCBBx vector register
pub mod vctr34;
///VCTR35 (rw) register accessor: an alias for `Reg<VCTR35_SPEC>`
pub type VCTR35 = crate::Reg<vctr35::VCTR35_SPEC>;
///MPCBBx vector register
pub mod vctr35;
///VCTR36 (rw) register accessor: an alias for `Reg<VCTR36_SPEC>`
pub type VCTR36 = crate::Reg<vctr36::VCTR36_SPEC>;
///MPCBBx vector register
pub mod vctr36;
///VCTR37 (rw) register accessor: an alias for `Reg<VCTR37_SPEC>`
pub type VCTR37 = crate::Reg<vctr37::VCTR37_SPEC>;
///MPCBBx vector register
pub mod vctr37;
///VCTR38 (rw) register accessor: an alias for `Reg<VCTR38_SPEC>`
pub type VCTR38 = crate::Reg<vctr38::VCTR38_SPEC>;
///MPCBBx vector register
pub mod vctr38;
///VCTR39 (rw) register accessor: an alias for `Reg<VCTR39_SPEC>`
pub type VCTR39 = crate::Reg<vctr39::VCTR39_SPEC>;
///MPCBBx vector register
pub mod vctr39;
///VCTR40 (rw) register accessor: an alias for `Reg<VCTR40_SPEC>`
pub type VCTR40 = crate::Reg<vctr40::VCTR40_SPEC>;
///MPCBBx vector register
pub mod vctr40;
///VCTR41 (rw) register accessor: an alias for `Reg<VCTR41_SPEC>`
pub type VCTR41 = crate::Reg<vctr41::VCTR41_SPEC>;
///MPCBBx vector register
pub mod vctr41;
///VCTR42 (rw) register accessor: an alias for `Reg<VCTR42_SPEC>`
pub type VCTR42 = crate::Reg<vctr42::VCTR42_SPEC>;
///MPCBBx vector register
pub mod vctr42;
///VCTR43 (rw) register accessor: an alias for `Reg<VCTR43_SPEC>`
pub type VCTR43 = crate::Reg<vctr43::VCTR43_SPEC>;
///MPCBBx vector register
pub mod vctr43;
///VCTR44 (rw) register accessor: an alias for `Reg<VCTR44_SPEC>`
pub type VCTR44 = crate::Reg<vctr44::VCTR44_SPEC>;
///MPCBBx vector register
pub mod vctr44;
///VCTR45 (rw) register accessor: an alias for `Reg<VCTR45_SPEC>`
pub type VCTR45 = crate::Reg<vctr45::VCTR45_SPEC>;
///MPCBBx vector register
pub mod vctr45;
///VCTR46 (rw) register accessor: an alias for `Reg<VCTR46_SPEC>`
pub type VCTR46 = crate::Reg<vctr46::VCTR46_SPEC>;
///MPCBBx vector register
pub mod vctr46;
///VCTR47 (rw) register accessor: an alias for `Reg<VCTR47_SPEC>`
pub type VCTR47 = crate::Reg<vctr47::VCTR47_SPEC>;
///MPCBBx vector register
pub mod vctr47;
///VCTR48 (rw) register accessor: an alias for `Reg<VCTR48_SPEC>`
pub type VCTR48 = crate::Reg<vctr48::VCTR48_SPEC>;
///MPCBBx vector register
pub mod vctr48;
///VCTR49 (rw) register accessor: an alias for `Reg<VCTR49_SPEC>`
pub type VCTR49 = crate::Reg<vctr49::VCTR49_SPEC>;
///MPCBBx vector register
pub mod vctr49;
///VCTR50 (rw) register accessor: an alias for `Reg<VCTR50_SPEC>`
pub type VCTR50 = crate::Reg<vctr50::VCTR50_SPEC>;
///MPCBBx vector register
pub mod vctr50;
///VCTR51 (rw) register accessor: an alias for `Reg<VCTR51_SPEC>`
pub type VCTR51 = crate::Reg<vctr51::VCTR51_SPEC>;
///MPCBBx vector register
pub mod vctr51;
///VCTR52 (rw) register accessor: an alias for `Reg<VCTR52_SPEC>`
pub type VCTR52 = crate::Reg<vctr52::VCTR52_SPEC>;
///MPCBBx vector register
pub mod vctr52;
///VCTR53 (rw) register accessor: an alias for `Reg<VCTR53_SPEC>`
pub type VCTR53 = crate::Reg<vctr53::VCTR53_SPEC>;
///MPCBBx vector register
pub mod vctr53;
///VCTR54 (rw) register accessor: an alias for `Reg<VCTR54_SPEC>`
pub type VCTR54 = crate::Reg<vctr54::VCTR54_SPEC>;
///MPCBBx vector register
pub mod vctr54;
///VCTR55 (rw) register accessor: an alias for `Reg<VCTR55_SPEC>`
pub type VCTR55 = crate::Reg<vctr55::VCTR55_SPEC>;
///MPCBBx vector register
pub mod vctr55;
///VCTR56 (rw) register accessor: an alias for `Reg<VCTR56_SPEC>`
pub type VCTR56 = crate::Reg<vctr56::VCTR56_SPEC>;
///MPCBBx vector register
pub mod vctr56;
///VCTR57 (rw) register accessor: an alias for `Reg<VCTR57_SPEC>`
pub type VCTR57 = crate::Reg<vctr57::VCTR57_SPEC>;
///MPCBBx vector register
pub mod vctr57;
///VCTR58 (rw) register accessor: an alias for `Reg<VCTR58_SPEC>`
pub type VCTR58 = crate::Reg<vctr58::VCTR58_SPEC>;
///MPCBBx vector register
pub mod vctr58;
///VCTR59 (rw) register accessor: an alias for `Reg<VCTR59_SPEC>`
pub type VCTR59 = crate::Reg<vctr59::VCTR59_SPEC>;
///MPCBBx vector register
pub mod vctr59;
///VCTR60 (rw) register accessor: an alias for `Reg<VCTR60_SPEC>`
pub type VCTR60 = crate::Reg<vctr60::VCTR60_SPEC>;
///MPCBBx vector register
pub mod vctr60;
///VCTR61 (rw) register accessor: an alias for `Reg<VCTR61_SPEC>`
pub type VCTR61 = crate::Reg<vctr61::VCTR61_SPEC>;
///MPCBBx vector register
pub mod vctr61;
///VCTR62 (rw) register accessor: an alias for `Reg<VCTR62_SPEC>`
pub type VCTR62 = crate::Reg<vctr62::VCTR62_SPEC>;
///MPCBBx vector register
pub mod vctr62;
///VCTR63 (rw) register accessor: an alias for `Reg<VCTR63_SPEC>`
pub type VCTR63 = crate::Reg<vctr63::VCTR63_SPEC>;
///MPCBBx vector register
pub mod vctr63;
