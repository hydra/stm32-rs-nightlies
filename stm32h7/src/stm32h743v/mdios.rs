///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MDIOS configuration register
    pub cr: CR,
    ///0x04 - MDIOS write flag register
    pub wrfr: WRFR,
    ///0x08 - MDIOS clear write flag register
    pub cwrfr: CWRFR,
    ///0x0c - MDIOS read flag register
    pub rdfr: RDFR,
    ///0x10 - MDIOS clear read flag register
    pub crdfr: CRDFR,
    ///0x14 - MDIOS status register
    pub sr: SR,
    ///0x18 - MDIOS clear flag register
    pub clrfr: CLRFR,
    ///0x1c - MDIOS input data register 0
    pub dinr0: DINR0,
    ///0x20 - MDIOS input data register 1
    pub dinr1: DINR1,
    ///0x24 - MDIOS input data register 2
    pub dinr2: DINR2,
    ///0x28 - MDIOS input data register 3
    pub dinr3: DINR3,
    ///0x2c - MDIOS input data register 4
    pub dinr4: DINR4,
    ///0x30 - MDIOS input data register 5
    pub dinr5: DINR5,
    ///0x34 - MDIOS input data register 6
    pub dinr6: DINR6,
    ///0x38 - MDIOS input data register 7
    pub dinr7: DINR7,
    ///0x3c - MDIOS input data register 8
    pub dinr8: DINR8,
    ///0x40 - MDIOS input data register 9
    pub dinr9: DINR9,
    ///0x44 - MDIOS input data register 10
    pub dinr10: DINR10,
    ///0x48 - MDIOS input data register 11
    pub dinr11: DINR11,
    ///0x4c - MDIOS input data register 12
    pub dinr12: DINR12,
    ///0x50 - MDIOS input data register 13
    pub dinr13: DINR13,
    ///0x54 - MDIOS input data register 14
    pub dinr14: DINR14,
    ///0x58 - MDIOS input data register 15
    pub dinr15: DINR15,
    ///0x5c - MDIOS input data register 16
    pub dinr16: DINR16,
    ///0x60 - MDIOS input data register 17
    pub dinr17: DINR17,
    ///0x64 - MDIOS input data register 18
    pub dinr18: DINR18,
    ///0x68 - MDIOS input data register 19
    pub dinr19: DINR19,
    ///0x6c - MDIOS input data register 20
    pub dinr20: DINR20,
    ///0x70 - MDIOS input data register 21
    pub dinr21: DINR21,
    ///0x74 - MDIOS input data register 22
    pub dinr22: DINR22,
    ///0x78 - MDIOS input data register 23
    pub dinr23: DINR23,
    ///0x7c - MDIOS input data register 24
    pub dinr24: DINR24,
    ///0x80 - MDIOS input data register 25
    pub dinr25: DINR25,
    ///0x84 - MDIOS input data register 26
    pub dinr26: DINR26,
    ///0x88 - MDIOS input data register 27
    pub dinr27: DINR27,
    ///0x8c - MDIOS input data register 28
    pub dinr28: DINR28,
    ///0x90 - MDIOS input data register 29
    pub dinr29: DINR29,
    ///0x94 - MDIOS input data register 30
    pub dinr30: DINR30,
    ///0x98 - MDIOS input data register 31
    pub dinr31: DINR31,
    ///0x9c - MDIOS output data register 0
    pub doutr0: DOUTR0,
    ///0xa0 - MDIOS output data register 1
    pub doutr1: DOUTR1,
    ///0xa4 - MDIOS output data register 2
    pub doutr2: DOUTR2,
    ///0xa8 - MDIOS output data register 3
    pub doutr3: DOUTR3,
    ///0xac - MDIOS output data register 4
    pub doutr4: DOUTR4,
    ///0xb0 - MDIOS output data register 5
    pub doutr5: DOUTR5,
    ///0xb4 - MDIOS output data register 6
    pub doutr6: DOUTR6,
    ///0xb8 - MDIOS output data register 7
    pub doutr7: DOUTR7,
    ///0xbc - MDIOS output data register 8
    pub doutr8: DOUTR8,
    ///0xc0 - MDIOS output data register 9
    pub doutr9: DOUTR9,
    ///0xc4 - MDIOS output data register 10
    pub doutr10: DOUTR10,
    ///0xc8 - MDIOS output data register 11
    pub doutr11: DOUTR11,
    ///0xcc - MDIOS output data register 12
    pub doutr12: DOUTR12,
    ///0xd0 - MDIOS output data register 13
    pub doutr13: DOUTR13,
    ///0xd4 - MDIOS output data register 14
    pub doutr14: DOUTR14,
    ///0xd8 - MDIOS output data register 15
    pub doutr15: DOUTR15,
    ///0xdc - MDIOS output data register 16
    pub doutr16: DOUTR16,
    ///0xe0 - MDIOS output data register 17
    pub doutr17: DOUTR17,
    ///0xe4 - MDIOS output data register 18
    pub doutr18: DOUTR18,
    ///0xe8 - MDIOS output data register 19
    pub doutr19: DOUTR19,
    ///0xec - MDIOS output data register 20
    pub doutr20: DOUTR20,
    ///0xf0 - MDIOS output data register 21
    pub doutr21: DOUTR21,
    ///0xf4 - MDIOS output data register 22
    pub doutr22: DOUTR22,
    ///0xf8 - MDIOS output data register 23
    pub doutr23: DOUTR23,
    ///0xfc - MDIOS output data register 24
    pub doutr24: DOUTR24,
    ///0x100 - MDIOS output data register 25
    pub doutr25: DOUTR25,
    ///0x104 - MDIOS output data register 26
    pub doutr26: DOUTR26,
    ///0x108 - MDIOS output data register 27
    pub doutr27: DOUTR27,
    ///0x10c - MDIOS output data register 28
    pub doutr28: DOUTR28,
    ///0x110 - MDIOS output data register 29
    pub doutr29: DOUTR29,
    ///0x114 - MDIOS output data register 30
    pub doutr30: DOUTR30,
    ///0x118 - MDIOS output data register 31
    pub doutr31: DOUTR31,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///MDIOS configuration register
pub mod cr;
///WRFR (r) register accessor: an alias for `Reg<WRFR_SPEC>`
pub type WRFR = crate::Reg<wrfr::WRFR_SPEC>;
///MDIOS write flag register
pub mod wrfr;
///CWRFR (rw) register accessor: an alias for `Reg<CWRFR_SPEC>`
pub type CWRFR = crate::Reg<cwrfr::CWRFR_SPEC>;
///MDIOS clear write flag register
pub mod cwrfr;
///RDFR (r) register accessor: an alias for `Reg<RDFR_SPEC>`
pub type RDFR = crate::Reg<rdfr::RDFR_SPEC>;
///MDIOS read flag register
pub mod rdfr;
///CRDFR (rw) register accessor: an alias for `Reg<CRDFR_SPEC>`
pub type CRDFR = crate::Reg<crdfr::CRDFR_SPEC>;
///MDIOS clear read flag register
pub mod crdfr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///MDIOS status register
pub mod sr;
///CLRFR (rw) register accessor: an alias for `Reg<CLRFR_SPEC>`
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
///MDIOS clear flag register
pub mod clrfr;
///DINR0 (r) register accessor: an alias for `Reg<DINR0_SPEC>`
pub type DINR0 = crate::Reg<dinr0::DINR0_SPEC>;
///MDIOS input data register 0
pub mod dinr0;
///DINR1 (r) register accessor: an alias for `Reg<DINR1_SPEC>`
pub type DINR1 = crate::Reg<dinr1::DINR1_SPEC>;
///MDIOS input data register 1
pub mod dinr1;
///DINR2 (r) register accessor: an alias for `Reg<DINR2_SPEC>`
pub type DINR2 = crate::Reg<dinr2::DINR2_SPEC>;
///MDIOS input data register 2
pub mod dinr2;
///DINR3 (r) register accessor: an alias for `Reg<DINR3_SPEC>`
pub type DINR3 = crate::Reg<dinr3::DINR3_SPEC>;
///MDIOS input data register 3
pub mod dinr3;
///DINR4 (r) register accessor: an alias for `Reg<DINR4_SPEC>`
pub type DINR4 = crate::Reg<dinr4::DINR4_SPEC>;
///MDIOS input data register 4
pub mod dinr4;
///DINR5 (r) register accessor: an alias for `Reg<DINR5_SPEC>`
pub type DINR5 = crate::Reg<dinr5::DINR5_SPEC>;
///MDIOS input data register 5
pub mod dinr5;
///DINR6 (r) register accessor: an alias for `Reg<DINR6_SPEC>`
pub type DINR6 = crate::Reg<dinr6::DINR6_SPEC>;
///MDIOS input data register 6
pub mod dinr6;
///DINR7 (r) register accessor: an alias for `Reg<DINR7_SPEC>`
pub type DINR7 = crate::Reg<dinr7::DINR7_SPEC>;
///MDIOS input data register 7
pub mod dinr7;
///DINR8 (r) register accessor: an alias for `Reg<DINR8_SPEC>`
pub type DINR8 = crate::Reg<dinr8::DINR8_SPEC>;
///MDIOS input data register 8
pub mod dinr8;
///DINR9 (r) register accessor: an alias for `Reg<DINR9_SPEC>`
pub type DINR9 = crate::Reg<dinr9::DINR9_SPEC>;
///MDIOS input data register 9
pub mod dinr9;
///DINR10 (r) register accessor: an alias for `Reg<DINR10_SPEC>`
pub type DINR10 = crate::Reg<dinr10::DINR10_SPEC>;
///MDIOS input data register 10
pub mod dinr10;
///DINR11 (r) register accessor: an alias for `Reg<DINR11_SPEC>`
pub type DINR11 = crate::Reg<dinr11::DINR11_SPEC>;
///MDIOS input data register 11
pub mod dinr11;
///DINR12 (r) register accessor: an alias for `Reg<DINR12_SPEC>`
pub type DINR12 = crate::Reg<dinr12::DINR12_SPEC>;
///MDIOS input data register 12
pub mod dinr12;
///DINR13 (r) register accessor: an alias for `Reg<DINR13_SPEC>`
pub type DINR13 = crate::Reg<dinr13::DINR13_SPEC>;
///MDIOS input data register 13
pub mod dinr13;
///DINR14 (r) register accessor: an alias for `Reg<DINR14_SPEC>`
pub type DINR14 = crate::Reg<dinr14::DINR14_SPEC>;
///MDIOS input data register 14
pub mod dinr14;
///DINR15 (r) register accessor: an alias for `Reg<DINR15_SPEC>`
pub type DINR15 = crate::Reg<dinr15::DINR15_SPEC>;
///MDIOS input data register 15
pub mod dinr15;
///DINR16 (r) register accessor: an alias for `Reg<DINR16_SPEC>`
pub type DINR16 = crate::Reg<dinr16::DINR16_SPEC>;
///MDIOS input data register 16
pub mod dinr16;
///DINR17 (r) register accessor: an alias for `Reg<DINR17_SPEC>`
pub type DINR17 = crate::Reg<dinr17::DINR17_SPEC>;
///MDIOS input data register 17
pub mod dinr17;
///DINR18 (r) register accessor: an alias for `Reg<DINR18_SPEC>`
pub type DINR18 = crate::Reg<dinr18::DINR18_SPEC>;
///MDIOS input data register 18
pub mod dinr18;
///DINR19 (r) register accessor: an alias for `Reg<DINR19_SPEC>`
pub type DINR19 = crate::Reg<dinr19::DINR19_SPEC>;
///MDIOS input data register 19
pub mod dinr19;
///DINR20 (r) register accessor: an alias for `Reg<DINR20_SPEC>`
pub type DINR20 = crate::Reg<dinr20::DINR20_SPEC>;
///MDIOS input data register 20
pub mod dinr20;
///DINR21 (r) register accessor: an alias for `Reg<DINR21_SPEC>`
pub type DINR21 = crate::Reg<dinr21::DINR21_SPEC>;
///MDIOS input data register 21
pub mod dinr21;
///DINR22 (r) register accessor: an alias for `Reg<DINR22_SPEC>`
pub type DINR22 = crate::Reg<dinr22::DINR22_SPEC>;
///MDIOS input data register 22
pub mod dinr22;
///DINR23 (r) register accessor: an alias for `Reg<DINR23_SPEC>`
pub type DINR23 = crate::Reg<dinr23::DINR23_SPEC>;
///MDIOS input data register 23
pub mod dinr23;
///DINR24 (r) register accessor: an alias for `Reg<DINR24_SPEC>`
pub type DINR24 = crate::Reg<dinr24::DINR24_SPEC>;
///MDIOS input data register 24
pub mod dinr24;
///DINR25 (r) register accessor: an alias for `Reg<DINR25_SPEC>`
pub type DINR25 = crate::Reg<dinr25::DINR25_SPEC>;
///MDIOS input data register 25
pub mod dinr25;
///DINR26 (r) register accessor: an alias for `Reg<DINR26_SPEC>`
pub type DINR26 = crate::Reg<dinr26::DINR26_SPEC>;
///MDIOS input data register 26
pub mod dinr26;
///DINR27 (r) register accessor: an alias for `Reg<DINR27_SPEC>`
pub type DINR27 = crate::Reg<dinr27::DINR27_SPEC>;
///MDIOS input data register 27
pub mod dinr27;
///DINR28 (r) register accessor: an alias for `Reg<DINR28_SPEC>`
pub type DINR28 = crate::Reg<dinr28::DINR28_SPEC>;
///MDIOS input data register 28
pub mod dinr28;
///DINR29 (r) register accessor: an alias for `Reg<DINR29_SPEC>`
pub type DINR29 = crate::Reg<dinr29::DINR29_SPEC>;
///MDIOS input data register 29
pub mod dinr29;
///DINR30 (r) register accessor: an alias for `Reg<DINR30_SPEC>`
pub type DINR30 = crate::Reg<dinr30::DINR30_SPEC>;
///MDIOS input data register 30
pub mod dinr30;
///DINR31 (r) register accessor: an alias for `Reg<DINR31_SPEC>`
pub type DINR31 = crate::Reg<dinr31::DINR31_SPEC>;
///MDIOS input data register 31
pub mod dinr31;
///DOUTR0 (rw) register accessor: an alias for `Reg<DOUTR0_SPEC>`
pub type DOUTR0 = crate::Reg<doutr0::DOUTR0_SPEC>;
///MDIOS output data register 0
pub mod doutr0;
///DOUTR1 (rw) register accessor: an alias for `Reg<DOUTR1_SPEC>`
pub type DOUTR1 = crate::Reg<doutr1::DOUTR1_SPEC>;
///MDIOS output data register 1
pub mod doutr1;
///DOUTR2 (rw) register accessor: an alias for `Reg<DOUTR2_SPEC>`
pub type DOUTR2 = crate::Reg<doutr2::DOUTR2_SPEC>;
///MDIOS output data register 2
pub mod doutr2;
///DOUTR3 (rw) register accessor: an alias for `Reg<DOUTR3_SPEC>`
pub type DOUTR3 = crate::Reg<doutr3::DOUTR3_SPEC>;
///MDIOS output data register 3
pub mod doutr3;
///DOUTR4 (rw) register accessor: an alias for `Reg<DOUTR4_SPEC>`
pub type DOUTR4 = crate::Reg<doutr4::DOUTR4_SPEC>;
///MDIOS output data register 4
pub mod doutr4;
///DOUTR5 (rw) register accessor: an alias for `Reg<DOUTR5_SPEC>`
pub type DOUTR5 = crate::Reg<doutr5::DOUTR5_SPEC>;
///MDIOS output data register 5
pub mod doutr5;
///DOUTR6 (rw) register accessor: an alias for `Reg<DOUTR6_SPEC>`
pub type DOUTR6 = crate::Reg<doutr6::DOUTR6_SPEC>;
///MDIOS output data register 6
pub mod doutr6;
///DOUTR7 (rw) register accessor: an alias for `Reg<DOUTR7_SPEC>`
pub type DOUTR7 = crate::Reg<doutr7::DOUTR7_SPEC>;
///MDIOS output data register 7
pub mod doutr7;
///DOUTR8 (rw) register accessor: an alias for `Reg<DOUTR8_SPEC>`
pub type DOUTR8 = crate::Reg<doutr8::DOUTR8_SPEC>;
///MDIOS output data register 8
pub mod doutr8;
///DOUTR9 (rw) register accessor: an alias for `Reg<DOUTR9_SPEC>`
pub type DOUTR9 = crate::Reg<doutr9::DOUTR9_SPEC>;
///MDIOS output data register 9
pub mod doutr9;
///DOUTR10 (rw) register accessor: an alias for `Reg<DOUTR10_SPEC>`
pub type DOUTR10 = crate::Reg<doutr10::DOUTR10_SPEC>;
///MDIOS output data register 10
pub mod doutr10;
///DOUTR11 (rw) register accessor: an alias for `Reg<DOUTR11_SPEC>`
pub type DOUTR11 = crate::Reg<doutr11::DOUTR11_SPEC>;
///MDIOS output data register 11
pub mod doutr11;
///DOUTR12 (rw) register accessor: an alias for `Reg<DOUTR12_SPEC>`
pub type DOUTR12 = crate::Reg<doutr12::DOUTR12_SPEC>;
///MDIOS output data register 12
pub mod doutr12;
///DOUTR13 (rw) register accessor: an alias for `Reg<DOUTR13_SPEC>`
pub type DOUTR13 = crate::Reg<doutr13::DOUTR13_SPEC>;
///MDIOS output data register 13
pub mod doutr13;
///DOUTR14 (rw) register accessor: an alias for `Reg<DOUTR14_SPEC>`
pub type DOUTR14 = crate::Reg<doutr14::DOUTR14_SPEC>;
///MDIOS output data register 14
pub mod doutr14;
///DOUTR15 (rw) register accessor: an alias for `Reg<DOUTR15_SPEC>`
pub type DOUTR15 = crate::Reg<doutr15::DOUTR15_SPEC>;
///MDIOS output data register 15
pub mod doutr15;
///DOUTR16 (rw) register accessor: an alias for `Reg<DOUTR16_SPEC>`
pub type DOUTR16 = crate::Reg<doutr16::DOUTR16_SPEC>;
///MDIOS output data register 16
pub mod doutr16;
///DOUTR17 (rw) register accessor: an alias for `Reg<DOUTR17_SPEC>`
pub type DOUTR17 = crate::Reg<doutr17::DOUTR17_SPEC>;
///MDIOS output data register 17
pub mod doutr17;
///DOUTR18 (rw) register accessor: an alias for `Reg<DOUTR18_SPEC>`
pub type DOUTR18 = crate::Reg<doutr18::DOUTR18_SPEC>;
///MDIOS output data register 18
pub mod doutr18;
///DOUTR19 (rw) register accessor: an alias for `Reg<DOUTR19_SPEC>`
pub type DOUTR19 = crate::Reg<doutr19::DOUTR19_SPEC>;
///MDIOS output data register 19
pub mod doutr19;
///DOUTR20 (rw) register accessor: an alias for `Reg<DOUTR20_SPEC>`
pub type DOUTR20 = crate::Reg<doutr20::DOUTR20_SPEC>;
///MDIOS output data register 20
pub mod doutr20;
///DOUTR21 (rw) register accessor: an alias for `Reg<DOUTR21_SPEC>`
pub type DOUTR21 = crate::Reg<doutr21::DOUTR21_SPEC>;
///MDIOS output data register 21
pub mod doutr21;
///DOUTR22 (rw) register accessor: an alias for `Reg<DOUTR22_SPEC>`
pub type DOUTR22 = crate::Reg<doutr22::DOUTR22_SPEC>;
///MDIOS output data register 22
pub mod doutr22;
///DOUTR23 (rw) register accessor: an alias for `Reg<DOUTR23_SPEC>`
pub type DOUTR23 = crate::Reg<doutr23::DOUTR23_SPEC>;
///MDIOS output data register 23
pub mod doutr23;
///DOUTR24 (rw) register accessor: an alias for `Reg<DOUTR24_SPEC>`
pub type DOUTR24 = crate::Reg<doutr24::DOUTR24_SPEC>;
///MDIOS output data register 24
pub mod doutr24;
///DOUTR25 (rw) register accessor: an alias for `Reg<DOUTR25_SPEC>`
pub type DOUTR25 = crate::Reg<doutr25::DOUTR25_SPEC>;
///MDIOS output data register 25
pub mod doutr25;
///DOUTR26 (rw) register accessor: an alias for `Reg<DOUTR26_SPEC>`
pub type DOUTR26 = crate::Reg<doutr26::DOUTR26_SPEC>;
///MDIOS output data register 26
pub mod doutr26;
///DOUTR27 (rw) register accessor: an alias for `Reg<DOUTR27_SPEC>`
pub type DOUTR27 = crate::Reg<doutr27::DOUTR27_SPEC>;
///MDIOS output data register 27
pub mod doutr27;
///DOUTR28 (rw) register accessor: an alias for `Reg<DOUTR28_SPEC>`
pub type DOUTR28 = crate::Reg<doutr28::DOUTR28_SPEC>;
///MDIOS output data register 28
pub mod doutr28;
///DOUTR29 (rw) register accessor: an alias for `Reg<DOUTR29_SPEC>`
pub type DOUTR29 = crate::Reg<doutr29::DOUTR29_SPEC>;
///MDIOS output data register 29
pub mod doutr29;
///DOUTR30 (rw) register accessor: an alias for `Reg<DOUTR30_SPEC>`
pub type DOUTR30 = crate::Reg<doutr30::DOUTR30_SPEC>;
///MDIOS output data register 30
pub mod doutr30;
///DOUTR31 (rw) register accessor: an alias for `Reg<DOUTR31_SPEC>`
pub type DOUTR31 = crate::Reg<doutr31::DOUTR31_SPEC>;
///MDIOS output data register 31
pub mod doutr31;
