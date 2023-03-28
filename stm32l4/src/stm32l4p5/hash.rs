///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - data input register
    pub din: DIN,
    ///0x08 - start register
    pub str: STR,
    ///0x0c - digest registers
    pub hra0: HRA0,
    ///0x10 - digest registers
    pub hra1: HRA1,
    ///0x14 - digest registers
    pub hra2: HRA2,
    ///0x18 - digest registers
    pub hra3: HRA3,
    ///0x1c - digest registers
    pub hra4: HRA4,
    ///0x20 - interrupt enable register
    pub imr: IMR,
    ///0x24 - status register
    pub sr: SR,
    _reserved10: [u8; 0xd0],
    ///0xf8 - context swap registers
    pub csr0: CSR0,
    ///0xfc - context swap registers
    pub csr1: CSR1,
    ///0x100 - context swap registers
    pub csr2: CSR2,
    ///0x104 - context swap registers
    pub csr3: CSR3,
    ///0x108 - context swap registers
    pub csr4: CSR4,
    ///0x10c - context swap registers
    pub csr5: CSR5,
    ///0x110 - context swap registers
    pub csr6: CSR6,
    ///0x114 - context swap registers
    pub csr7: CSR7,
    ///0x118 - context swap registers
    pub csr8: CSR8,
    ///0x11c - context swap registers
    pub csr9: CSR9,
    ///0x120 - context swap registers
    pub csr10: CSR10,
    ///0x124 - context swap registers
    pub csr11: CSR11,
    ///0x128 - context swap registers
    pub csr12: CSR12,
    ///0x12c - context swap registers
    pub csr13: CSR13,
    ///0x130 - context swap registers
    pub csr14: CSR14,
    ///0x134 - context swap registers
    pub csr15: CSR15,
    ///0x138 - context swap registers
    pub csr16: CSR16,
    ///0x13c - context swap registers
    pub csr17: CSR17,
    ///0x140 - context swap registers
    pub csr18: CSR18,
    ///0x144 - context swap registers
    pub csr19: CSR19,
    ///0x148 - context swap registers
    pub csr20: CSR20,
    ///0x14c - context swap registers
    pub csr21: CSR21,
    ///0x150 - context swap registers
    pub csr22: CSR22,
    ///0x154 - context swap registers
    pub csr23: CSR23,
    ///0x158 - context swap registers
    pub csr24: CSR24,
    ///0x15c - context swap registers
    pub csr25: CSR25,
    ///0x160 - context swap registers
    pub csr26: CSR26,
    ///0x164 - context swap registers
    pub csr27: CSR27,
    ///0x168 - context swap registers
    pub csr28: CSR28,
    ///0x16c - context swap registers
    pub csr29: CSR29,
    ///0x170 - context swap registers
    pub csr30: CSR30,
    ///0x174 - context swap registers
    pub csr31: CSR31,
    ///0x178 - context swap registers
    pub csr32: CSR32,
    ///0x17c - context swap registers
    pub csr33: CSR33,
    ///0x180 - context swap registers
    pub csr34: CSR34,
    ///0x184 - context swap registers
    pub csr35: CSR35,
    ///0x188 - context swap registers
    pub csr36: CSR36,
    ///0x18c - context swap registers
    pub csr37: CSR37,
    ///0x190 - context swap registers
    pub csr38: CSR38,
    ///0x194 - context swap registers
    pub csr39: CSR39,
    ///0x198 - context swap registers
    pub csr40: CSR40,
    ///0x19c - context swap registers
    pub csr41: CSR41,
    ///0x1a0 - context swap registers
    pub csr42: CSR42,
    ///0x1a4 - context swap registers
    pub csr43: CSR43,
    ///0x1a8 - context swap registers
    pub csr44: CSR44,
    ///0x1ac - context swap registers
    pub csr45: CSR45,
    ///0x1b0 - context swap registers
    pub csr46: CSR46,
    ///0x1b4 - context swap registers
    pub csr47: CSR47,
    ///0x1b8 - context swap registers
    pub csr48: CSR48,
    ///0x1bc - context swap registers
    pub csr49: CSR49,
    ///0x1c0 - context swap registers
    pub csr50: CSR50,
    ///0x1c4 - context swap registers
    pub csr51: CSR51,
    ///0x1c8 - context swap registers
    pub csr52: CSR52,
    ///0x1cc - context swap registers
    pub csr53: CSR53,
    _reserved64: [u8; 0x0140],
    ///0x310 - HASH digest register
    pub hr0: HR0,
    ///0x314 - read-only
    pub hr1: HR1,
    ///0x318 - read-only
    pub hr2: HR2,
    ///0x31c - read-only
    pub hr3: HR3,
    ///0x320 - read-only
    pub hr4: HR4,
    ///0x324 - read-only
    pub hr5: HR5,
    ///0x328 - read-only
    pub hr6: HR6,
    ///0x32c - read-only
    pub hr7: HR7,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///DIN (rw) register accessor: an alias for `Reg<DIN_SPEC>`
pub type DIN = crate::Reg<din::DIN_SPEC>;
///data input register
pub mod din;
///STR (rw) register accessor: an alias for `Reg<STR_SPEC>`
pub type STR = crate::Reg<str::STR_SPEC>;
///start register
pub mod str;
///HRA0 (r) register accessor: an alias for `Reg<HRA0_SPEC>`
pub type HRA0 = crate::Reg<hra0::HRA0_SPEC>;
///digest registers
pub mod hra0;
///IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`
pub type IMR = crate::Reg<imr::IMR_SPEC>;
///interrupt enable register
pub mod imr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///CSR0 (rw) register accessor: an alias for `Reg<CSR0_SPEC>`
pub type CSR0 = crate::Reg<csr0::CSR0_SPEC>;
///context swap registers
pub mod csr0;
///CSR1 (rw) register accessor: an alias for `Reg<CSR1_SPEC>`
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
///context swap registers
pub mod csr1;
///CSR2 (rw) register accessor: an alias for `Reg<CSR2_SPEC>`
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
///context swap registers
pub mod csr2;
///CSR3 (rw) register accessor: an alias for `Reg<CSR3_SPEC>`
pub type CSR3 = crate::Reg<csr3::CSR3_SPEC>;
///context swap registers
pub mod csr3;
///CSR4 (rw) register accessor: an alias for `Reg<CSR4_SPEC>`
pub type CSR4 = crate::Reg<csr4::CSR4_SPEC>;
///context swap registers
pub mod csr4;
///CSR5 (rw) register accessor: an alias for `Reg<CSR5_SPEC>`
pub type CSR5 = crate::Reg<csr5::CSR5_SPEC>;
///context swap registers
pub mod csr5;
///CSR6 (rw) register accessor: an alias for `Reg<CSR6_SPEC>`
pub type CSR6 = crate::Reg<csr6::CSR6_SPEC>;
///context swap registers
pub mod csr6;
///CSR7 (rw) register accessor: an alias for `Reg<CSR7_SPEC>`
pub type CSR7 = crate::Reg<csr7::CSR7_SPEC>;
///context swap registers
pub mod csr7;
///CSR8 (rw) register accessor: an alias for `Reg<CSR8_SPEC>`
pub type CSR8 = crate::Reg<csr8::CSR8_SPEC>;
///context swap registers
pub mod csr8;
///CSR9 (rw) register accessor: an alias for `Reg<CSR9_SPEC>`
pub type CSR9 = crate::Reg<csr9::CSR9_SPEC>;
///context swap registers
pub mod csr9;
///CSR10 (rw) register accessor: an alias for `Reg<CSR10_SPEC>`
pub type CSR10 = crate::Reg<csr10::CSR10_SPEC>;
///context swap registers
pub mod csr10;
///CSR11 (rw) register accessor: an alias for `Reg<CSR11_SPEC>`
pub type CSR11 = crate::Reg<csr11::CSR11_SPEC>;
///context swap registers
pub mod csr11;
///CSR12 (rw) register accessor: an alias for `Reg<CSR12_SPEC>`
pub type CSR12 = crate::Reg<csr12::CSR12_SPEC>;
///context swap registers
pub mod csr12;
///CSR13 (rw) register accessor: an alias for `Reg<CSR13_SPEC>`
pub type CSR13 = crate::Reg<csr13::CSR13_SPEC>;
///context swap registers
pub mod csr13;
///CSR14 (rw) register accessor: an alias for `Reg<CSR14_SPEC>`
pub type CSR14 = crate::Reg<csr14::CSR14_SPEC>;
///context swap registers
pub mod csr14;
///CSR15 (rw) register accessor: an alias for `Reg<CSR15_SPEC>`
pub type CSR15 = crate::Reg<csr15::CSR15_SPEC>;
///context swap registers
pub mod csr15;
///CSR16 (rw) register accessor: an alias for `Reg<CSR16_SPEC>`
pub type CSR16 = crate::Reg<csr16::CSR16_SPEC>;
///context swap registers
pub mod csr16;
///CSR17 (rw) register accessor: an alias for `Reg<CSR17_SPEC>`
pub type CSR17 = crate::Reg<csr17::CSR17_SPEC>;
///context swap registers
pub mod csr17;
///CSR18 (rw) register accessor: an alias for `Reg<CSR18_SPEC>`
pub type CSR18 = crate::Reg<csr18::CSR18_SPEC>;
///context swap registers
pub mod csr18;
///CSR19 (rw) register accessor: an alias for `Reg<CSR19_SPEC>`
pub type CSR19 = crate::Reg<csr19::CSR19_SPEC>;
///context swap registers
pub mod csr19;
///CSR20 (rw) register accessor: an alias for `Reg<CSR20_SPEC>`
pub type CSR20 = crate::Reg<csr20::CSR20_SPEC>;
///context swap registers
pub mod csr20;
///CSR21 (rw) register accessor: an alias for `Reg<CSR21_SPEC>`
pub type CSR21 = crate::Reg<csr21::CSR21_SPEC>;
///context swap registers
pub mod csr21;
///CSR22 (rw) register accessor: an alias for `Reg<CSR22_SPEC>`
pub type CSR22 = crate::Reg<csr22::CSR22_SPEC>;
///context swap registers
pub mod csr22;
///CSR23 (rw) register accessor: an alias for `Reg<CSR23_SPEC>`
pub type CSR23 = crate::Reg<csr23::CSR23_SPEC>;
///context swap registers
pub mod csr23;
///CSR24 (rw) register accessor: an alias for `Reg<CSR24_SPEC>`
pub type CSR24 = crate::Reg<csr24::CSR24_SPEC>;
///context swap registers
pub mod csr24;
///CSR25 (rw) register accessor: an alias for `Reg<CSR25_SPEC>`
pub type CSR25 = crate::Reg<csr25::CSR25_SPEC>;
///context swap registers
pub mod csr25;
///CSR26 (rw) register accessor: an alias for `Reg<CSR26_SPEC>`
pub type CSR26 = crate::Reg<csr26::CSR26_SPEC>;
///context swap registers
pub mod csr26;
///CSR27 (rw) register accessor: an alias for `Reg<CSR27_SPEC>`
pub type CSR27 = crate::Reg<csr27::CSR27_SPEC>;
///context swap registers
pub mod csr27;
///CSR28 (rw) register accessor: an alias for `Reg<CSR28_SPEC>`
pub type CSR28 = crate::Reg<csr28::CSR28_SPEC>;
///context swap registers
pub mod csr28;
///CSR29 (rw) register accessor: an alias for `Reg<CSR29_SPEC>`
pub type CSR29 = crate::Reg<csr29::CSR29_SPEC>;
///context swap registers
pub mod csr29;
///CSR30 (rw) register accessor: an alias for `Reg<CSR30_SPEC>`
pub type CSR30 = crate::Reg<csr30::CSR30_SPEC>;
///context swap registers
pub mod csr30;
///CSR31 (rw) register accessor: an alias for `Reg<CSR31_SPEC>`
pub type CSR31 = crate::Reg<csr31::CSR31_SPEC>;
///context swap registers
pub mod csr31;
///CSR32 (rw) register accessor: an alias for `Reg<CSR32_SPEC>`
pub type CSR32 = crate::Reg<csr32::CSR32_SPEC>;
///context swap registers
pub mod csr32;
///CSR33 (rw) register accessor: an alias for `Reg<CSR33_SPEC>`
pub type CSR33 = crate::Reg<csr33::CSR33_SPEC>;
///context swap registers
pub mod csr33;
///CSR34 (rw) register accessor: an alias for `Reg<CSR34_SPEC>`
pub type CSR34 = crate::Reg<csr34::CSR34_SPEC>;
///context swap registers
pub mod csr34;
///CSR35 (rw) register accessor: an alias for `Reg<CSR35_SPEC>`
pub type CSR35 = crate::Reg<csr35::CSR35_SPEC>;
///context swap registers
pub mod csr35;
///CSR36 (rw) register accessor: an alias for `Reg<CSR36_SPEC>`
pub type CSR36 = crate::Reg<csr36::CSR36_SPEC>;
///context swap registers
pub mod csr36;
///CSR37 (rw) register accessor: an alias for `Reg<CSR37_SPEC>`
pub type CSR37 = crate::Reg<csr37::CSR37_SPEC>;
///context swap registers
pub mod csr37;
///CSR38 (rw) register accessor: an alias for `Reg<CSR38_SPEC>`
pub type CSR38 = crate::Reg<csr38::CSR38_SPEC>;
///context swap registers
pub mod csr38;
///CSR39 (rw) register accessor: an alias for `Reg<CSR39_SPEC>`
pub type CSR39 = crate::Reg<csr39::CSR39_SPEC>;
///context swap registers
pub mod csr39;
///CSR40 (rw) register accessor: an alias for `Reg<CSR40_SPEC>`
pub type CSR40 = crate::Reg<csr40::CSR40_SPEC>;
///context swap registers
pub mod csr40;
///CSR41 (rw) register accessor: an alias for `Reg<CSR41_SPEC>`
pub type CSR41 = crate::Reg<csr41::CSR41_SPEC>;
///context swap registers
pub mod csr41;
///CSR42 (rw) register accessor: an alias for `Reg<CSR42_SPEC>`
pub type CSR42 = crate::Reg<csr42::CSR42_SPEC>;
///context swap registers
pub mod csr42;
///CSR43 (rw) register accessor: an alias for `Reg<CSR43_SPEC>`
pub type CSR43 = crate::Reg<csr43::CSR43_SPEC>;
///context swap registers
pub mod csr43;
///CSR44 (rw) register accessor: an alias for `Reg<CSR44_SPEC>`
pub type CSR44 = crate::Reg<csr44::CSR44_SPEC>;
///context swap registers
pub mod csr44;
///CSR45 (rw) register accessor: an alias for `Reg<CSR45_SPEC>`
pub type CSR45 = crate::Reg<csr45::CSR45_SPEC>;
///context swap registers
pub mod csr45;
///CSR46 (rw) register accessor: an alias for `Reg<CSR46_SPEC>`
pub type CSR46 = crate::Reg<csr46::CSR46_SPEC>;
///context swap registers
pub mod csr46;
///CSR47 (rw) register accessor: an alias for `Reg<CSR47_SPEC>`
pub type CSR47 = crate::Reg<csr47::CSR47_SPEC>;
///context swap registers
pub mod csr47;
///CSR48 (rw) register accessor: an alias for `Reg<CSR48_SPEC>`
pub type CSR48 = crate::Reg<csr48::CSR48_SPEC>;
///context swap registers
pub mod csr48;
///CSR49 (rw) register accessor: an alias for `Reg<CSR49_SPEC>`
pub type CSR49 = crate::Reg<csr49::CSR49_SPEC>;
///context swap registers
pub mod csr49;
///CSR50 (rw) register accessor: an alias for `Reg<CSR50_SPEC>`
pub type CSR50 = crate::Reg<csr50::CSR50_SPEC>;
///context swap registers
pub mod csr50;
///CSR51 (rw) register accessor: an alias for `Reg<CSR51_SPEC>`
pub type CSR51 = crate::Reg<csr51::CSR51_SPEC>;
///context swap registers
pub mod csr51;
///CSR52 (rw) register accessor: an alias for `Reg<CSR52_SPEC>`
pub type CSR52 = crate::Reg<csr52::CSR52_SPEC>;
///context swap registers
pub mod csr52;
///CSR53 (rw) register accessor: an alias for `Reg<CSR53_SPEC>`
pub type CSR53 = crate::Reg<csr53::CSR53_SPEC>;
///context swap registers
pub mod csr53;
///HR0 (r) register accessor: an alias for `Reg<HR0_SPEC>`
pub type HR0 = crate::Reg<hr0::HR0_SPEC>;
///HASH digest register
pub mod hr0;
///HR1 (r) register accessor: an alias for `Reg<HR1_SPEC>`
pub type HR1 = crate::Reg<hr1::HR1_SPEC>;
///read-only
pub mod hr1;
///HR2 (r) register accessor: an alias for `Reg<HR2_SPEC>`
pub type HR2 = crate::Reg<hr2::HR2_SPEC>;
///read-only
pub mod hr2;
///HR3 (r) register accessor: an alias for `Reg<HR3_SPEC>`
pub type HR3 = crate::Reg<hr3::HR3_SPEC>;
///read-only
pub mod hr3;
///HR4 (r) register accessor: an alias for `Reg<HR4_SPEC>`
pub type HR4 = crate::Reg<hr4::HR4_SPEC>;
///read-only
pub mod hr4;
///HR5 (r) register accessor: an alias for `Reg<HR5_SPEC>`
pub type HR5 = crate::Reg<hr5::HR5_SPEC>;
///read-only
pub mod hr5;
///HR6 (r) register accessor: an alias for `Reg<HR6_SPEC>`
pub type HR6 = crate::Reg<hr6::HR6_SPEC>;
///read-only
pub mod hr6;
///HR7 (r) register accessor: an alias for `Reg<HR7_SPEC>`
pub type HR7 = crate::Reg<hr7::HR7_SPEC>;
///read-only
pub mod hr7;
///HRA1 (rw) register accessor: an alias for `Reg<HRA1_SPEC>`
pub type HRA1 = crate::Reg<hra1::HRA1_SPEC>;
///digest registers
pub mod hra1;
///HRA2 (rw) register accessor: an alias for `Reg<HRA2_SPEC>`
pub type HRA2 = crate::Reg<hra2::HRA2_SPEC>;
///digest registers
pub mod hra2;
///HRA3 (rw) register accessor: an alias for `Reg<HRA3_SPEC>`
pub type HRA3 = crate::Reg<hra3::HRA3_SPEC>;
///digest registers
pub mod hra3;
///HRA4 (rw) register accessor: an alias for `Reg<HRA4_SPEC>`
pub type HRA4 = crate::Reg<hra4::HRA4_SPEC>;
///digest registers
pub mod hra4;
