///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - HASH control register
    pub cr: CR,
    ///0x04 - HASH data input register
    pub din: DIN,
    ///0x08 - HASH start register
    pub str: STR,
    ///0x0c - HASH aliased digest register 0
    pub hra0: HRA0,
    ///0x10 - HASH aliased digest register 1
    pub hra1: HRA1,
    ///0x14 - HASH aliased digest register 2
    pub hra2: HRA2,
    ///0x18 - HASH aliased digest register 3
    pub hra3: HRA3,
    ///0x1c - HASH aliased digest register 4
    pub hra4: HRA4,
    ///0x20 - HASH interrupt enable register
    pub imr: IMR,
    ///0x24 - HASH status register
    pub sr: SR,
    _reserved10: [u8; 0xd0],
    ///0xf8 - HASH context swap register 0
    pub csr0: CSR0,
    ///0xfc - HASH context swap register 1
    pub csr1: CSR1,
    ///0x100 - HASH context swap register 2
    pub csr2: CSR2,
    ///0x104 - HASH context swap register 3
    pub csr3: CSR3,
    ///0x108 - HASH context swap register 4
    pub csr4: CSR4,
    ///0x10c - HASH context swap register 5
    pub csr5: CSR5,
    ///0x110 - HASH context swap register 6
    pub csr6: CSR6,
    ///0x114 - HASH context swap register 7
    pub csr7: CSR7,
    ///0x118 - HASH context swap register 8
    pub csr8: CSR8,
    ///0x11c - HASH context swap register 9
    pub csr9: CSR9,
    ///0x120 - HASH context swap register 10
    pub csr10: CSR10,
    ///0x124 - HASH context swap register 11
    pub csr11: CSR11,
    ///0x128 - HASH context swap register 12
    pub csr12: CSR12,
    ///0x12c - HASH context swap register 13
    pub csr13: CSR13,
    ///0x130 - HASH context swap register 14
    pub csr14: CSR14,
    ///0x134 - HASH context swap register 15
    pub csr15: CSR15,
    ///0x138 - HASH context swap register 16
    pub csr16: CSR16,
    ///0x13c - HASH context swap register 17
    pub csr17: CSR17,
    ///0x140 - HASH context swap register 18
    pub csr18: CSR18,
    ///0x144 - HASH context swap register 19
    pub csr19: CSR19,
    ///0x148 - HASH context swap register 20
    pub csr20: CSR20,
    ///0x14c - HASH context swap register 21
    pub csr21: CSR21,
    ///0x150 - HASH context swap register 22
    pub csr22: CSR22,
    ///0x154 - HASH context swap register 23
    pub csr23: CSR23,
    ///0x158 - HASH context swap register 24
    pub csr24: CSR24,
    ///0x15c - HASH context swap register 25
    pub csr25: CSR25,
    ///0x160 - HASH context swap register 26
    pub csr26: CSR26,
    ///0x164 - HASH context swap register 27
    pub csr27: CSR27,
    ///0x168 - HASH context swap register 28
    pub csr28: CSR28,
    ///0x16c - HASH context swap register 29
    pub csr29: CSR29,
    ///0x170 - HASH context swap register 30
    pub csr30: CSR30,
    ///0x174 - HASH context swap register 31
    pub csr31: CSR31,
    ///0x178 - HASH context swap register 32
    pub csr32: CSR32,
    ///0x17c - HASH context swap register 33
    pub csr33: CSR33,
    ///0x180 - HASH context swap register 34
    pub csr34: CSR34,
    ///0x184 - HASH context swap register 35
    pub csr35: CSR35,
    ///0x188 - HASH context swap register 36
    pub csr36: CSR36,
    ///0x18c - HASH context swap register 37
    pub csr37: CSR37,
    ///0x190 - HASH context swap register 38
    pub csr38: CSR38,
    ///0x194 - HASH context swap register 39
    pub csr39: CSR39,
    ///0x198 - HASH context swap register 40
    pub csr40: CSR40,
    ///0x19c - HASH context swap register 41
    pub csr41: CSR41,
    ///0x1a0 - HASH context swap register 42
    pub csr42: CSR42,
    ///0x1a4 - HASH context swap register 43
    pub csr43: CSR43,
    ///0x1a8 - HASH context swap register 44
    pub csr44: CSR44,
    ///0x1ac - HASH context swap register 45
    pub csr45: CSR45,
    ///0x1b0 - HASH context swap register 46
    pub csr46: CSR46,
    ///0x1b4 - HASH context swap register 47
    pub csr47: CSR47,
    ///0x1b8 - HASH context swap register 48
    pub csr48: CSR48,
    ///0x1bc - HASH context swap register 49
    pub csr49: CSR49,
    ///0x1c0 - HASH context swap register 50
    pub csr50: CSR50,
    ///0x1c4 - HASH context swap register 51
    pub csr51: CSR51,
    ///0x1c8 - HASH context swap register 52
    pub csr52: CSR52,
    ///0x1cc - HASH context swap register 53
    pub csr53: CSR53,
    ///0x1d0 - HASH context swap register 54
    pub csr54: CSR54,
    ///0x1d4 - HASH context swap register 55
    pub csr55: CSR55,
    ///0x1d8 - HASH context swap register 56
    pub csr56: CSR56,
    ///0x1dc - HASH context swap register 57
    pub csr57: CSR57,
    ///0x1e0 - HASH context swap register 58
    pub csr58: CSR58,
    ///0x1e4 - HASH context swap register 59
    pub csr59: CSR59,
    ///0x1e8 - HASH context swap register 60
    pub csr60: CSR60,
    ///0x1ec - HASH context swap register 61
    pub csr61: CSR61,
    ///0x1f0 - HASH context swap register 62
    pub csr62: CSR62,
    ///0x1f4 - HASH context swap register 63
    pub csr63: CSR63,
    ///0x1f8 - HASH context swap register 64
    pub csr64: CSR64,
    ///0x1fc - HASH context swap register 65
    pub csr65: CSR65,
    ///0x200 - HASH context swap register 66
    pub csr66: CSR66,
    ///0x204 - HASH context swap register 67
    pub csr67: CSR67,
    ///0x208 - HASH context swap register 68
    pub csr68: CSR68,
    ///0x20c - HASH context swap register 69
    pub csr69: CSR69,
    ///0x210 - HASH context swap register 70
    pub csr70: CSR70,
    ///0x214 - HASH context swap register 71
    pub csr71: CSR71,
    ///0x218 - HASH context swap register 72
    pub csr72: CSR72,
    ///0x21c - HASH context swap register 73
    pub csr73: CSR73,
    ///0x220 - HASH context swap register 74
    pub csr74: CSR74,
    ///0x224 - HASH context swap register 75
    pub csr75: CSR75,
    ///0x228 - HASH context swap register 76
    pub csr76: CSR76,
    ///0x22c - HASH context swap register 77
    pub csr77: CSR77,
    ///0x230 - HASH context swap register 78
    pub csr78: CSR78,
    ///0x234 - HASH context swap register 79
    pub csr79: CSR79,
    ///0x238 - HASH context swap register 80
    pub csr80: CSR80,
    ///0x23c - HASH context swap register 81
    pub csr81: CSR81,
    ///0x240 - HASH context swap register 82
    pub csr82: CSR82,
    ///0x244 - HASH context swap register 83
    pub csr83: CSR83,
    ///0x248 - HASH context swap register 84
    pub csr84: CSR84,
    ///0x24c - HASH context swap register 85
    pub csr85: CSR85,
    ///0x250 - HASH context swap register 86
    pub csr86: CSR86,
    ///0x254 - HASH context swap register 87
    pub csr87: CSR87,
    ///0x258 - HASH context swap register 88
    pub csr88: CSR88,
    ///0x25c - HASH context swap register 89
    pub csr89: CSR89,
    ///0x260 - HASH context swap register 90
    pub csr90: CSR90,
    ///0x264 - HASH context swap register 91
    pub csr91: CSR91,
    ///0x268 - HASH context swap register 92
    pub csr92: CSR92,
    ///0x26c - HASH context swap register 93
    pub csr93: CSR93,
    ///0x270 - HASH context swap register 94
    pub csr94: CSR94,
    ///0x274 - HASH context swap register 95
    pub csr95: CSR95,
    ///0x278 - HASH context swap register 96
    pub csr96: CSR96,
    ///0x27c - HASH context swap register 97
    pub csr97: CSR97,
    ///0x280 - HASH context swap register 98
    pub csr98: CSR98,
    ///0x284 - HASH context swap register 99
    pub csr99: CSR99,
    ///0x288 - HASH context swap register 100
    pub csr100: CSR100,
    ///0x28c - HASH context swap register 101
    pub csr101: CSR101,
    ///0x290 - HASH context swap register 102
    pub csr102: CSR102,
    _reserved113: [u8; 0x7c],
    ///0x310 - HASH digest register
    pub hr0: HR0,
    ///0x314 - HASH digest register
    pub hr1: HR1,
    ///0x318 - HASH digest register
    pub hr2: HR2,
    ///0x31c - HASH digest register
    pub hr3: HR3,
    ///0x320 - HASH digest register
    pub hr4: HR4,
    ///0x324 - HASH digest register
    pub hr5: HR5,
    ///0x328 - HASH digest register
    pub hr6: HR6,
    ///0x32c - HASH digest register
    pub hr7: HR7,
    ///0x330 - HASH digest register
    pub hr8: HR8,
    ///0x334 - HASH digest register
    pub hr9: HR9,
    ///0x338 - HASH digest register
    pub hr10: HR10,
    ///0x33c - HASH digest register
    pub hr11: HR11,
    ///0x340 - HASH digest register
    pub hr12: HR12,
    ///0x344 - HASH digest register
    pub hr13: HR13,
    ///0x348 - HASH digest register
    pub hr14: HR14,
    ///0x34c - HASH digest register
    pub hr15: HR15,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///HASH control register
pub mod cr;
///DIN (w) register accessor: an alias for `Reg<DIN_SPEC>`
pub type DIN = crate::Reg<din::DIN_SPEC>;
///HASH data input register
pub mod din;
///STR (rw) register accessor: an alias for `Reg<STR_SPEC>`
pub type STR = crate::Reg<str::STR_SPEC>;
///HASH start register
pub mod str;
///HRA0 (r) register accessor: an alias for `Reg<HRA0_SPEC>`
pub type HRA0 = crate::Reg<hra0::HRA0_SPEC>;
///HASH aliased digest register 0
pub mod hra0;
///HRA1 (r) register accessor: an alias for `Reg<HRA1_SPEC>`
pub type HRA1 = crate::Reg<hra1::HRA1_SPEC>;
///HASH aliased digest register 1
pub mod hra1;
///HRA2 (r) register accessor: an alias for `Reg<HRA2_SPEC>`
pub type HRA2 = crate::Reg<hra2::HRA2_SPEC>;
///HASH aliased digest register 2
pub mod hra2;
///HRA3 (r) register accessor: an alias for `Reg<HRA3_SPEC>`
pub type HRA3 = crate::Reg<hra3::HRA3_SPEC>;
///HASH aliased digest register 3
pub mod hra3;
///HRA4 (r) register accessor: an alias for `Reg<HRA4_SPEC>`
pub type HRA4 = crate::Reg<hra4::HRA4_SPEC>;
///HASH aliased digest register 4
pub mod hra4;
///IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`
pub type IMR = crate::Reg<imr::IMR_SPEC>;
///HASH interrupt enable register
pub mod imr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///HASH status register
pub mod sr;
///CSR0 (rw) register accessor: an alias for `Reg<CSR0_SPEC>`
pub type CSR0 = crate::Reg<csr0::CSR0_SPEC>;
///HASH context swap register 0
pub mod csr0;
///CSR1 (rw) register accessor: an alias for `Reg<CSR1_SPEC>`
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
///HASH context swap register 1
pub mod csr1;
///CSR2 (rw) register accessor: an alias for `Reg<CSR2_SPEC>`
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
///HASH context swap register 2
pub mod csr2;
///CSR3 (rw) register accessor: an alias for `Reg<CSR3_SPEC>`
pub type CSR3 = crate::Reg<csr3::CSR3_SPEC>;
///HASH context swap register 3
pub mod csr3;
///CSR4 (rw) register accessor: an alias for `Reg<CSR4_SPEC>`
pub type CSR4 = crate::Reg<csr4::CSR4_SPEC>;
///HASH context swap register 4
pub mod csr4;
///CSR5 (rw) register accessor: an alias for `Reg<CSR5_SPEC>`
pub type CSR5 = crate::Reg<csr5::CSR5_SPEC>;
///HASH context swap register 5
pub mod csr5;
///CSR6 (rw) register accessor: an alias for `Reg<CSR6_SPEC>`
pub type CSR6 = crate::Reg<csr6::CSR6_SPEC>;
///HASH context swap register 6
pub mod csr6;
///CSR7 (rw) register accessor: an alias for `Reg<CSR7_SPEC>`
pub type CSR7 = crate::Reg<csr7::CSR7_SPEC>;
///HASH context swap register 7
pub mod csr7;
///CSR8 (rw) register accessor: an alias for `Reg<CSR8_SPEC>`
pub type CSR8 = crate::Reg<csr8::CSR8_SPEC>;
///HASH context swap register 8
pub mod csr8;
///CSR9 (rw) register accessor: an alias for `Reg<CSR9_SPEC>`
pub type CSR9 = crate::Reg<csr9::CSR9_SPEC>;
///HASH context swap register 9
pub mod csr9;
///CSR10 (rw) register accessor: an alias for `Reg<CSR10_SPEC>`
pub type CSR10 = crate::Reg<csr10::CSR10_SPEC>;
///HASH context swap register 10
pub mod csr10;
///CSR11 (rw) register accessor: an alias for `Reg<CSR11_SPEC>`
pub type CSR11 = crate::Reg<csr11::CSR11_SPEC>;
///HASH context swap register 11
pub mod csr11;
///CSR12 (rw) register accessor: an alias for `Reg<CSR12_SPEC>`
pub type CSR12 = crate::Reg<csr12::CSR12_SPEC>;
///HASH context swap register 12
pub mod csr12;
///CSR13 (rw) register accessor: an alias for `Reg<CSR13_SPEC>`
pub type CSR13 = crate::Reg<csr13::CSR13_SPEC>;
///HASH context swap register 13
pub mod csr13;
///CSR14 (rw) register accessor: an alias for `Reg<CSR14_SPEC>`
pub type CSR14 = crate::Reg<csr14::CSR14_SPEC>;
///HASH context swap register 14
pub mod csr14;
///CSR15 (rw) register accessor: an alias for `Reg<CSR15_SPEC>`
pub type CSR15 = crate::Reg<csr15::CSR15_SPEC>;
///HASH context swap register 15
pub mod csr15;
///CSR16 (rw) register accessor: an alias for `Reg<CSR16_SPEC>`
pub type CSR16 = crate::Reg<csr16::CSR16_SPEC>;
///HASH context swap register 16
pub mod csr16;
///CSR17 (rw) register accessor: an alias for `Reg<CSR17_SPEC>`
pub type CSR17 = crate::Reg<csr17::CSR17_SPEC>;
///HASH context swap register 17
pub mod csr17;
///CSR18 (rw) register accessor: an alias for `Reg<CSR18_SPEC>`
pub type CSR18 = crate::Reg<csr18::CSR18_SPEC>;
///HASH context swap register 18
pub mod csr18;
///CSR19 (rw) register accessor: an alias for `Reg<CSR19_SPEC>`
pub type CSR19 = crate::Reg<csr19::CSR19_SPEC>;
///HASH context swap register 19
pub mod csr19;
///CSR20 (rw) register accessor: an alias for `Reg<CSR20_SPEC>`
pub type CSR20 = crate::Reg<csr20::CSR20_SPEC>;
///HASH context swap register 20
pub mod csr20;
///CSR21 (rw) register accessor: an alias for `Reg<CSR21_SPEC>`
pub type CSR21 = crate::Reg<csr21::CSR21_SPEC>;
///HASH context swap register 21
pub mod csr21;
///CSR22 (rw) register accessor: an alias for `Reg<CSR22_SPEC>`
pub type CSR22 = crate::Reg<csr22::CSR22_SPEC>;
///HASH context swap register 22
pub mod csr22;
///CSR23 (rw) register accessor: an alias for `Reg<CSR23_SPEC>`
pub type CSR23 = crate::Reg<csr23::CSR23_SPEC>;
///HASH context swap register 23
pub mod csr23;
///CSR24 (rw) register accessor: an alias for `Reg<CSR24_SPEC>`
pub type CSR24 = crate::Reg<csr24::CSR24_SPEC>;
///HASH context swap register 24
pub mod csr24;
///CSR25 (rw) register accessor: an alias for `Reg<CSR25_SPEC>`
pub type CSR25 = crate::Reg<csr25::CSR25_SPEC>;
///HASH context swap register 25
pub mod csr25;
///CSR26 (rw) register accessor: an alias for `Reg<CSR26_SPEC>`
pub type CSR26 = crate::Reg<csr26::CSR26_SPEC>;
///HASH context swap register 26
pub mod csr26;
///CSR27 (rw) register accessor: an alias for `Reg<CSR27_SPEC>`
pub type CSR27 = crate::Reg<csr27::CSR27_SPEC>;
///HASH context swap register 27
pub mod csr27;
///CSR28 (rw) register accessor: an alias for `Reg<CSR28_SPEC>`
pub type CSR28 = crate::Reg<csr28::CSR28_SPEC>;
///HASH context swap register 28
pub mod csr28;
///CSR29 (rw) register accessor: an alias for `Reg<CSR29_SPEC>`
pub type CSR29 = crate::Reg<csr29::CSR29_SPEC>;
///HASH context swap register 29
pub mod csr29;
///CSR30 (rw) register accessor: an alias for `Reg<CSR30_SPEC>`
pub type CSR30 = crate::Reg<csr30::CSR30_SPEC>;
///HASH context swap register 30
pub mod csr30;
///CSR31 (rw) register accessor: an alias for `Reg<CSR31_SPEC>`
pub type CSR31 = crate::Reg<csr31::CSR31_SPEC>;
///HASH context swap register 31
pub mod csr31;
///CSR32 (rw) register accessor: an alias for `Reg<CSR32_SPEC>`
pub type CSR32 = crate::Reg<csr32::CSR32_SPEC>;
///HASH context swap register 32
pub mod csr32;
///CSR33 (rw) register accessor: an alias for `Reg<CSR33_SPEC>`
pub type CSR33 = crate::Reg<csr33::CSR33_SPEC>;
///HASH context swap register 33
pub mod csr33;
///CSR34 (rw) register accessor: an alias for `Reg<CSR34_SPEC>`
pub type CSR34 = crate::Reg<csr34::CSR34_SPEC>;
///HASH context swap register 34
pub mod csr34;
///CSR35 (rw) register accessor: an alias for `Reg<CSR35_SPEC>`
pub type CSR35 = crate::Reg<csr35::CSR35_SPEC>;
///HASH context swap register 35
pub mod csr35;
///CSR36 (rw) register accessor: an alias for `Reg<CSR36_SPEC>`
pub type CSR36 = crate::Reg<csr36::CSR36_SPEC>;
///HASH context swap register 36
pub mod csr36;
///CSR37 (rw) register accessor: an alias for `Reg<CSR37_SPEC>`
pub type CSR37 = crate::Reg<csr37::CSR37_SPEC>;
///HASH context swap register 37
pub mod csr37;
///CSR38 (rw) register accessor: an alias for `Reg<CSR38_SPEC>`
pub type CSR38 = crate::Reg<csr38::CSR38_SPEC>;
///HASH context swap register 38
pub mod csr38;
///CSR39 (rw) register accessor: an alias for `Reg<CSR39_SPEC>`
pub type CSR39 = crate::Reg<csr39::CSR39_SPEC>;
///HASH context swap register 39
pub mod csr39;
///CSR40 (rw) register accessor: an alias for `Reg<CSR40_SPEC>`
pub type CSR40 = crate::Reg<csr40::CSR40_SPEC>;
///HASH context swap register 40
pub mod csr40;
///CSR41 (rw) register accessor: an alias for `Reg<CSR41_SPEC>`
pub type CSR41 = crate::Reg<csr41::CSR41_SPEC>;
///HASH context swap register 41
pub mod csr41;
///CSR42 (rw) register accessor: an alias for `Reg<CSR42_SPEC>`
pub type CSR42 = crate::Reg<csr42::CSR42_SPEC>;
///HASH context swap register 42
pub mod csr42;
///CSR43 (rw) register accessor: an alias for `Reg<CSR43_SPEC>`
pub type CSR43 = crate::Reg<csr43::CSR43_SPEC>;
///HASH context swap register 43
pub mod csr43;
///CSR44 (rw) register accessor: an alias for `Reg<CSR44_SPEC>`
pub type CSR44 = crate::Reg<csr44::CSR44_SPEC>;
///HASH context swap register 44
pub mod csr44;
///CSR45 (rw) register accessor: an alias for `Reg<CSR45_SPEC>`
pub type CSR45 = crate::Reg<csr45::CSR45_SPEC>;
///HASH context swap register 45
pub mod csr45;
///CSR46 (rw) register accessor: an alias for `Reg<CSR46_SPEC>`
pub type CSR46 = crate::Reg<csr46::CSR46_SPEC>;
///HASH context swap register 46
pub mod csr46;
///CSR47 (rw) register accessor: an alias for `Reg<CSR47_SPEC>`
pub type CSR47 = crate::Reg<csr47::CSR47_SPEC>;
///HASH context swap register 47
pub mod csr47;
///CSR48 (rw) register accessor: an alias for `Reg<CSR48_SPEC>`
pub type CSR48 = crate::Reg<csr48::CSR48_SPEC>;
///HASH context swap register 48
pub mod csr48;
///CSR49 (rw) register accessor: an alias for `Reg<CSR49_SPEC>`
pub type CSR49 = crate::Reg<csr49::CSR49_SPEC>;
///HASH context swap register 49
pub mod csr49;
///CSR50 (rw) register accessor: an alias for `Reg<CSR50_SPEC>`
pub type CSR50 = crate::Reg<csr50::CSR50_SPEC>;
///HASH context swap register 50
pub mod csr50;
///CSR51 (rw) register accessor: an alias for `Reg<CSR51_SPEC>`
pub type CSR51 = crate::Reg<csr51::CSR51_SPEC>;
///HASH context swap register 51
pub mod csr51;
///CSR52 (rw) register accessor: an alias for `Reg<CSR52_SPEC>`
pub type CSR52 = crate::Reg<csr52::CSR52_SPEC>;
///HASH context swap register 52
pub mod csr52;
///CSR53 (rw) register accessor: an alias for `Reg<CSR53_SPEC>`
pub type CSR53 = crate::Reg<csr53::CSR53_SPEC>;
///HASH context swap register 53
pub mod csr53;
///CSR54 (rw) register accessor: an alias for `Reg<CSR54_SPEC>`
pub type CSR54 = crate::Reg<csr54::CSR54_SPEC>;
///HASH context swap register 54
pub mod csr54;
///CSR55 (rw) register accessor: an alias for `Reg<CSR55_SPEC>`
pub type CSR55 = crate::Reg<csr55::CSR55_SPEC>;
///HASH context swap register 55
pub mod csr55;
///CSR56 (rw) register accessor: an alias for `Reg<CSR56_SPEC>`
pub type CSR56 = crate::Reg<csr56::CSR56_SPEC>;
///HASH context swap register 56
pub mod csr56;
///CSR57 (rw) register accessor: an alias for `Reg<CSR57_SPEC>`
pub type CSR57 = crate::Reg<csr57::CSR57_SPEC>;
///HASH context swap register 57
pub mod csr57;
///CSR58 (rw) register accessor: an alias for `Reg<CSR58_SPEC>`
pub type CSR58 = crate::Reg<csr58::CSR58_SPEC>;
///HASH context swap register 58
pub mod csr58;
///CSR59 (rw) register accessor: an alias for `Reg<CSR59_SPEC>`
pub type CSR59 = crate::Reg<csr59::CSR59_SPEC>;
///HASH context swap register 59
pub mod csr59;
///CSR60 (rw) register accessor: an alias for `Reg<CSR60_SPEC>`
pub type CSR60 = crate::Reg<csr60::CSR60_SPEC>;
///HASH context swap register 60
pub mod csr60;
///CSR61 (rw) register accessor: an alias for `Reg<CSR61_SPEC>`
pub type CSR61 = crate::Reg<csr61::CSR61_SPEC>;
///HASH context swap register 61
pub mod csr61;
///CSR62 (rw) register accessor: an alias for `Reg<CSR62_SPEC>`
pub type CSR62 = crate::Reg<csr62::CSR62_SPEC>;
///HASH context swap register 62
pub mod csr62;
///CSR63 (rw) register accessor: an alias for `Reg<CSR63_SPEC>`
pub type CSR63 = crate::Reg<csr63::CSR63_SPEC>;
///HASH context swap register 63
pub mod csr63;
///CSR64 (rw) register accessor: an alias for `Reg<CSR64_SPEC>`
pub type CSR64 = crate::Reg<csr64::CSR64_SPEC>;
///HASH context swap register 64
pub mod csr64;
///CSR65 (rw) register accessor: an alias for `Reg<CSR65_SPEC>`
pub type CSR65 = crate::Reg<csr65::CSR65_SPEC>;
///HASH context swap register 65
pub mod csr65;
///CSR66 (rw) register accessor: an alias for `Reg<CSR66_SPEC>`
pub type CSR66 = crate::Reg<csr66::CSR66_SPEC>;
///HASH context swap register 66
pub mod csr66;
///CSR67 (rw) register accessor: an alias for `Reg<CSR67_SPEC>`
pub type CSR67 = crate::Reg<csr67::CSR67_SPEC>;
///HASH context swap register 67
pub mod csr67;
///CSR68 (rw) register accessor: an alias for `Reg<CSR68_SPEC>`
pub type CSR68 = crate::Reg<csr68::CSR68_SPEC>;
///HASH context swap register 68
pub mod csr68;
///CSR69 (rw) register accessor: an alias for `Reg<CSR69_SPEC>`
pub type CSR69 = crate::Reg<csr69::CSR69_SPEC>;
///HASH context swap register 69
pub mod csr69;
///CSR70 (rw) register accessor: an alias for `Reg<CSR70_SPEC>`
pub type CSR70 = crate::Reg<csr70::CSR70_SPEC>;
///HASH context swap register 70
pub mod csr70;
///CSR71 (rw) register accessor: an alias for `Reg<CSR71_SPEC>`
pub type CSR71 = crate::Reg<csr71::CSR71_SPEC>;
///HASH context swap register 71
pub mod csr71;
///CSR72 (rw) register accessor: an alias for `Reg<CSR72_SPEC>`
pub type CSR72 = crate::Reg<csr72::CSR72_SPEC>;
///HASH context swap register 72
pub mod csr72;
///CSR73 (rw) register accessor: an alias for `Reg<CSR73_SPEC>`
pub type CSR73 = crate::Reg<csr73::CSR73_SPEC>;
///HASH context swap register 73
pub mod csr73;
///CSR74 (rw) register accessor: an alias for `Reg<CSR74_SPEC>`
pub type CSR74 = crate::Reg<csr74::CSR74_SPEC>;
///HASH context swap register 74
pub mod csr74;
///CSR75 (rw) register accessor: an alias for `Reg<CSR75_SPEC>`
pub type CSR75 = crate::Reg<csr75::CSR75_SPEC>;
///HASH context swap register 75
pub mod csr75;
///CSR76 (rw) register accessor: an alias for `Reg<CSR76_SPEC>`
pub type CSR76 = crate::Reg<csr76::CSR76_SPEC>;
///HASH context swap register 76
pub mod csr76;
///CSR77 (rw) register accessor: an alias for `Reg<CSR77_SPEC>`
pub type CSR77 = crate::Reg<csr77::CSR77_SPEC>;
///HASH context swap register 77
pub mod csr77;
///CSR78 (rw) register accessor: an alias for `Reg<CSR78_SPEC>`
pub type CSR78 = crate::Reg<csr78::CSR78_SPEC>;
///HASH context swap register 78
pub mod csr78;
///CSR79 (rw) register accessor: an alias for `Reg<CSR79_SPEC>`
pub type CSR79 = crate::Reg<csr79::CSR79_SPEC>;
///HASH context swap register 79
pub mod csr79;
///CSR80 (rw) register accessor: an alias for `Reg<CSR80_SPEC>`
pub type CSR80 = crate::Reg<csr80::CSR80_SPEC>;
///HASH context swap register 80
pub mod csr80;
///CSR81 (rw) register accessor: an alias for `Reg<CSR81_SPEC>`
pub type CSR81 = crate::Reg<csr81::CSR81_SPEC>;
///HASH context swap register 81
pub mod csr81;
///CSR82 (rw) register accessor: an alias for `Reg<CSR82_SPEC>`
pub type CSR82 = crate::Reg<csr82::CSR82_SPEC>;
///HASH context swap register 82
pub mod csr82;
///CSR83 (rw) register accessor: an alias for `Reg<CSR83_SPEC>`
pub type CSR83 = crate::Reg<csr83::CSR83_SPEC>;
///HASH context swap register 83
pub mod csr83;
///CSR84 (rw) register accessor: an alias for `Reg<CSR84_SPEC>`
pub type CSR84 = crate::Reg<csr84::CSR84_SPEC>;
///HASH context swap register 84
pub mod csr84;
///CSR85 (rw) register accessor: an alias for `Reg<CSR85_SPEC>`
pub type CSR85 = crate::Reg<csr85::CSR85_SPEC>;
///HASH context swap register 85
pub mod csr85;
///CSR86 (rw) register accessor: an alias for `Reg<CSR86_SPEC>`
pub type CSR86 = crate::Reg<csr86::CSR86_SPEC>;
///HASH context swap register 86
pub mod csr86;
///CSR87 (rw) register accessor: an alias for `Reg<CSR87_SPEC>`
pub type CSR87 = crate::Reg<csr87::CSR87_SPEC>;
///HASH context swap register 87
pub mod csr87;
///CSR88 (rw) register accessor: an alias for `Reg<CSR88_SPEC>`
pub type CSR88 = crate::Reg<csr88::CSR88_SPEC>;
///HASH context swap register 88
pub mod csr88;
///CSR89 (rw) register accessor: an alias for `Reg<CSR89_SPEC>`
pub type CSR89 = crate::Reg<csr89::CSR89_SPEC>;
///HASH context swap register 89
pub mod csr89;
///CSR90 (rw) register accessor: an alias for `Reg<CSR90_SPEC>`
pub type CSR90 = crate::Reg<csr90::CSR90_SPEC>;
///HASH context swap register 90
pub mod csr90;
///CSR91 (rw) register accessor: an alias for `Reg<CSR91_SPEC>`
pub type CSR91 = crate::Reg<csr91::CSR91_SPEC>;
///HASH context swap register 91
pub mod csr91;
///CSR92 (rw) register accessor: an alias for `Reg<CSR92_SPEC>`
pub type CSR92 = crate::Reg<csr92::CSR92_SPEC>;
///HASH context swap register 92
pub mod csr92;
///CSR93 (rw) register accessor: an alias for `Reg<CSR93_SPEC>`
pub type CSR93 = crate::Reg<csr93::CSR93_SPEC>;
///HASH context swap register 93
pub mod csr93;
///CSR94 (rw) register accessor: an alias for `Reg<CSR94_SPEC>`
pub type CSR94 = crate::Reg<csr94::CSR94_SPEC>;
///HASH context swap register 94
pub mod csr94;
///CSR95 (rw) register accessor: an alias for `Reg<CSR95_SPEC>`
pub type CSR95 = crate::Reg<csr95::CSR95_SPEC>;
///HASH context swap register 95
pub mod csr95;
///CSR96 (rw) register accessor: an alias for `Reg<CSR96_SPEC>`
pub type CSR96 = crate::Reg<csr96::CSR96_SPEC>;
///HASH context swap register 96
pub mod csr96;
///CSR97 (rw) register accessor: an alias for `Reg<CSR97_SPEC>`
pub type CSR97 = crate::Reg<csr97::CSR97_SPEC>;
///HASH context swap register 97
pub mod csr97;
///CSR98 (rw) register accessor: an alias for `Reg<CSR98_SPEC>`
pub type CSR98 = crate::Reg<csr98::CSR98_SPEC>;
///HASH context swap register 98
pub mod csr98;
///CSR99 (rw) register accessor: an alias for `Reg<CSR99_SPEC>`
pub type CSR99 = crate::Reg<csr99::CSR99_SPEC>;
///HASH context swap register 99
pub mod csr99;
///CSR100 (rw) register accessor: an alias for `Reg<CSR100_SPEC>`
pub type CSR100 = crate::Reg<csr100::CSR100_SPEC>;
///HASH context swap register 100
pub mod csr100;
///CSR101 (rw) register accessor: an alias for `Reg<CSR101_SPEC>`
pub type CSR101 = crate::Reg<csr101::CSR101_SPEC>;
///HASH context swap register 101
pub mod csr101;
///CSR102 (rw) register accessor: an alias for `Reg<CSR102_SPEC>`
pub type CSR102 = crate::Reg<csr102::CSR102_SPEC>;
///HASH context swap register 102
pub mod csr102;
///HR0 (r) register accessor: an alias for `Reg<HR0_SPEC>`
pub type HR0 = crate::Reg<hr0::HR0_SPEC>;
///HASH digest register
pub mod hr0;
///HR1 (r) register accessor: an alias for `Reg<HR1_SPEC>`
pub type HR1 = crate::Reg<hr1::HR1_SPEC>;
///HASH digest register
pub mod hr1;
///HR2 (r) register accessor: an alias for `Reg<HR2_SPEC>`
pub type HR2 = crate::Reg<hr2::HR2_SPEC>;
///HASH digest register
pub mod hr2;
///HR3 (r) register accessor: an alias for `Reg<HR3_SPEC>`
pub type HR3 = crate::Reg<hr3::HR3_SPEC>;
///HASH digest register
pub mod hr3;
///HR4 (r) register accessor: an alias for `Reg<HR4_SPEC>`
pub type HR4 = crate::Reg<hr4::HR4_SPEC>;
///HASH digest register
pub mod hr4;
///HR5 (r) register accessor: an alias for `Reg<HR5_SPEC>`
pub type HR5 = crate::Reg<hr5::HR5_SPEC>;
///HASH digest register
pub mod hr5;
///HR6 (r) register accessor: an alias for `Reg<HR6_SPEC>`
pub type HR6 = crate::Reg<hr6::HR6_SPEC>;
///HASH digest register
pub mod hr6;
///HR7 (r) register accessor: an alias for `Reg<HR7_SPEC>`
pub type HR7 = crate::Reg<hr7::HR7_SPEC>;
///HASH digest register
pub mod hr7;
///HR8 (r) register accessor: an alias for `Reg<HR8_SPEC>`
pub type HR8 = crate::Reg<hr8::HR8_SPEC>;
///HASH digest register
pub mod hr8;
///HR9 (r) register accessor: an alias for `Reg<HR9_SPEC>`
pub type HR9 = crate::Reg<hr9::HR9_SPEC>;
///HASH digest register
pub mod hr9;
///HR10 (r) register accessor: an alias for `Reg<HR10_SPEC>`
pub type HR10 = crate::Reg<hr10::HR10_SPEC>;
///HASH digest register
pub mod hr10;
///HR11 (r) register accessor: an alias for `Reg<HR11_SPEC>`
pub type HR11 = crate::Reg<hr11::HR11_SPEC>;
///HASH digest register
pub mod hr11;
///HR12 (r) register accessor: an alias for `Reg<HR12_SPEC>`
pub type HR12 = crate::Reg<hr12::HR12_SPEC>;
///HASH digest register
pub mod hr12;
///HR13 (r) register accessor: an alias for `Reg<HR13_SPEC>`
pub type HR13 = crate::Reg<hr13::HR13_SPEC>;
///HASH digest register
pub mod hr13;
///HR14 (r) register accessor: an alias for `Reg<HR14_SPEC>`
pub type HR14 = crate::Reg<hr14::HR14_SPEC>;
///HASH digest register
pub mod hr14;
///HR15 (r) register accessor: an alias for `Reg<HR15_SPEC>`
pub type HR15 = crate::Reg<hr15::HR15_SPEC>;
///HASH digest register
pub mod hr15;
