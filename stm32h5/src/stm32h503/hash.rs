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
    _reserved64: [u8; 0x0140],
    ///0x310 - HASH digest register 0
    pub hr0: HR0,
    ///0x314 - HASH digest register 1
    pub hr1: HR1,
    ///0x318 - HASH digest register 2
    pub hr2: HR2,
    ///0x31c - HASH digest register 3
    pub hr3: HR3,
    ///0x320 - HASH digest register 4
    pub hr4: HR4,
    ///0x324 - HASH supplementary digest register 5
    pub hr5: HR5,
    ///0x328 - HASH supplementary digest register 6
    pub hr6: HR6,
    ///0x32c - HASH supplementary digest register 7
    pub hr7: HR7,
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
///HR0 (r) register accessor: an alias for `Reg<HR0_SPEC>`
pub type HR0 = crate::Reg<hr0::HR0_SPEC>;
///HASH digest register 0
pub mod hr0;
///HR1 (r) register accessor: an alias for `Reg<HR1_SPEC>`
pub type HR1 = crate::Reg<hr1::HR1_SPEC>;
///HASH digest register 1
pub mod hr1;
///HR2 (r) register accessor: an alias for `Reg<HR2_SPEC>`
pub type HR2 = crate::Reg<hr2::HR2_SPEC>;
///HASH digest register 2
pub mod hr2;
///HR3 (r) register accessor: an alias for `Reg<HR3_SPEC>`
pub type HR3 = crate::Reg<hr3::HR3_SPEC>;
///HASH digest register 3
pub mod hr3;
///HR4 (r) register accessor: an alias for `Reg<HR4_SPEC>`
pub type HR4 = crate::Reg<hr4::HR4_SPEC>;
///HASH digest register 4
pub mod hr4;
///HR5 (r) register accessor: an alias for `Reg<HR5_SPEC>`
pub type HR5 = crate::Reg<hr5::HR5_SPEC>;
///HASH supplementary digest register 5
pub mod hr5;
///HR6 (r) register accessor: an alias for `Reg<HR6_SPEC>`
pub type HR6 = crate::Reg<hr6::HR6_SPEC>;
///HASH supplementary digest register 6
pub mod hr6;
///HR7 (r) register accessor: an alias for `Reg<HR7_SPEC>`
pub type HR7 = crate::Reg<hr7::HR7_SPEC>;
///HASH supplementary digest register 7
pub mod hr7;
