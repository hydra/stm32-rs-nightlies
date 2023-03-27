///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    ///0x80 - interrupt line 0 status register
    pub itline0: ITLINE0,
    ///0x84 - interrupt line 1 status register
    pub itline1: ITLINE1,
    ///0x88 - interrupt line 2 status register
    pub itline2: ITLINE2,
    ///0x8c - interrupt line 3 status register
    pub itline3: ITLINE3,
    ///0x90 - interrupt line 4 status register
    pub itline4: ITLINE4,
    ///0x94 - interrupt line 5 status register
    pub itline5: ITLINE5,
    ///0x98 - interrupt line 6 status register
    pub itline6: ITLINE6,
    ///0x9c - interrupt line 7 status register
    pub itline7: ITLINE7,
    _reserved8: [u8; 0x04],
    ///0xa4 - interrupt line 9 status register
    pub itline9: ITLINE9,
    ///0xa8 - interrupt line 10 status register
    pub itline10: ITLINE10,
    ///0xac - interrupt line 11 status register
    pub itline11: ITLINE11,
    ///0xb0 - interrupt line 12 status register
    pub itline12: ITLINE12,
    ///0xb4 - interrupt line 13 status register
    pub itline13: ITLINE13,
    ///0xb8 - interrupt line 14 status register
    pub itline14: ITLINE14,
    ///0xbc - interrupt line 15 status register
    pub itline15: ITLINE15,
    ///0xc0 - interrupt line 16 status register
    pub itline16: ITLINE16,
    _reserved16: [u8; 0x08],
    ///0xcc - interrupt line 19 status register
    pub itline19: ITLINE19,
    _reserved17: [u8; 0x04],
    ///0xd4 - interrupt line 21 status register
    pub itline21: ITLINE21,
    ///0xd8 - interrupt line 22 status register
    pub itline22: ITLINE22,
    ///0xdc - interrupt line 23 status register
    pub itline23: ITLINE23,
    ///0xe0 - interrupt line 24 status register
    pub itline24: ITLINE24,
    ///0xe4 - interrupt line 25 status register
    pub itline25: ITLINE25,
    ///0xe8 - interrupt line 26 status register
    pub itline26: ITLINE26,
    ///0xec - interrupt line 27 status register
    pub itline27: ITLINE27,
    ///0xf0 - interrupt line 28 status register
    pub itline28: ITLINE28,
    ///0xf4 - interrupt line 29 status register
    pub itline29: ITLINE29,
}
///ITLINE0 (r) register accessor: an alias for `Reg<ITLINE0_SPEC>`
pub type ITLINE0 = crate::Reg<itline0::ITLINE0_SPEC>;
///interrupt line 0 status register
pub mod itline0;
///ITLINE1 (r) register accessor: an alias for `Reg<ITLINE1_SPEC>`
pub type ITLINE1 = crate::Reg<itline1::ITLINE1_SPEC>;
///interrupt line 1 status register
pub mod itline1;
///ITLINE2 (r) register accessor: an alias for `Reg<ITLINE2_SPEC>`
pub type ITLINE2 = crate::Reg<itline2::ITLINE2_SPEC>;
///interrupt line 2 status register
pub mod itline2;
///ITLINE3 (r) register accessor: an alias for `Reg<ITLINE3_SPEC>`
pub type ITLINE3 = crate::Reg<itline3::ITLINE3_SPEC>;
///interrupt line 3 status register
pub mod itline3;
///ITLINE4 (r) register accessor: an alias for `Reg<ITLINE4_SPEC>`
pub type ITLINE4 = crate::Reg<itline4::ITLINE4_SPEC>;
///interrupt line 4 status register
pub mod itline4;
///ITLINE5 (r) register accessor: an alias for `Reg<ITLINE5_SPEC>`
pub type ITLINE5 = crate::Reg<itline5::ITLINE5_SPEC>;
///interrupt line 5 status register
pub mod itline5;
///ITLINE6 (r) register accessor: an alias for `Reg<ITLINE6_SPEC>`
pub type ITLINE6 = crate::Reg<itline6::ITLINE6_SPEC>;
///interrupt line 6 status register
pub mod itline6;
///ITLINE7 (r) register accessor: an alias for `Reg<ITLINE7_SPEC>`
pub type ITLINE7 = crate::Reg<itline7::ITLINE7_SPEC>;
///interrupt line 7 status register
pub mod itline7;
///ITLINE9 (r) register accessor: an alias for `Reg<ITLINE9_SPEC>`
pub type ITLINE9 = crate::Reg<itline9::ITLINE9_SPEC>;
///interrupt line 9 status register
pub mod itline9;
///ITLINE10 (r) register accessor: an alias for `Reg<ITLINE10_SPEC>`
pub type ITLINE10 = crate::Reg<itline10::ITLINE10_SPEC>;
///interrupt line 10 status register
pub mod itline10;
///ITLINE11 (r) register accessor: an alias for `Reg<ITLINE11_SPEC>`
pub type ITLINE11 = crate::Reg<itline11::ITLINE11_SPEC>;
///interrupt line 11 status register
pub mod itline11;
///ITLINE12 (r) register accessor: an alias for `Reg<ITLINE12_SPEC>`
pub type ITLINE12 = crate::Reg<itline12::ITLINE12_SPEC>;
///interrupt line 12 status register
pub mod itline12;
///ITLINE13 (r) register accessor: an alias for `Reg<ITLINE13_SPEC>`
pub type ITLINE13 = crate::Reg<itline13::ITLINE13_SPEC>;
///interrupt line 13 status register
pub mod itline13;
///ITLINE14 (r) register accessor: an alias for `Reg<ITLINE14_SPEC>`
pub type ITLINE14 = crate::Reg<itline14::ITLINE14_SPEC>;
///interrupt line 14 status register
pub mod itline14;
///ITLINE15 (r) register accessor: an alias for `Reg<ITLINE15_SPEC>`
pub type ITLINE15 = crate::Reg<itline15::ITLINE15_SPEC>;
///interrupt line 15 status register
pub mod itline15;
///ITLINE16 (r) register accessor: an alias for `Reg<ITLINE16_SPEC>`
pub type ITLINE16 = crate::Reg<itline16::ITLINE16_SPEC>;
///interrupt line 16 status register
pub mod itline16;
///ITLINE19 (r) register accessor: an alias for `Reg<ITLINE19_SPEC>`
pub type ITLINE19 = crate::Reg<itline19::ITLINE19_SPEC>;
///interrupt line 19 status register
pub mod itline19;
///ITLINE21 (r) register accessor: an alias for `Reg<ITLINE21_SPEC>`
pub type ITLINE21 = crate::Reg<itline21::ITLINE21_SPEC>;
///interrupt line 21 status register
pub mod itline21;
///ITLINE22 (r) register accessor: an alias for `Reg<ITLINE22_SPEC>`
pub type ITLINE22 = crate::Reg<itline22::ITLINE22_SPEC>;
///interrupt line 22 status register
pub mod itline22;
///ITLINE23 (r) register accessor: an alias for `Reg<ITLINE23_SPEC>`
pub type ITLINE23 = crate::Reg<itline23::ITLINE23_SPEC>;
///interrupt line 23 status register
pub mod itline23;
///ITLINE24 (r) register accessor: an alias for `Reg<ITLINE24_SPEC>`
pub type ITLINE24 = crate::Reg<itline24::ITLINE24_SPEC>;
///interrupt line 24 status register
pub mod itline24;
///ITLINE25 (r) register accessor: an alias for `Reg<ITLINE25_SPEC>`
pub type ITLINE25 = crate::Reg<itline25::ITLINE25_SPEC>;
///interrupt line 25 status register
pub mod itline25;
///ITLINE26 (r) register accessor: an alias for `Reg<ITLINE26_SPEC>`
pub type ITLINE26 = crate::Reg<itline26::ITLINE26_SPEC>;
///interrupt line 26 status register
pub mod itline26;
///ITLINE27 (r) register accessor: an alias for `Reg<ITLINE27_SPEC>`
pub type ITLINE27 = crate::Reg<itline27::ITLINE27_SPEC>;
///interrupt line 27 status register
pub mod itline27;
///ITLINE28 (r) register accessor: an alias for `Reg<ITLINE28_SPEC>`
pub type ITLINE28 = crate::Reg<itline28::ITLINE28_SPEC>;
///interrupt line 28 status register
pub mod itline28;
///ITLINE29 (r) register accessor: an alias for `Reg<ITLINE29_SPEC>`
pub type ITLINE29 = crate::Reg<itline29::ITLINE29_SPEC>;
///interrupt line 29 status register
pub mod itline29;
