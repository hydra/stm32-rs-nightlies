///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SYSCFG configuration register 1
    pub syscfg_cfgr1: SYSCFG_CFGR1,
    _reserved1: [u8; 0x14],
    ///0x18 - SYSCFG configuration register 2
    pub syscfg_cfgr2: SYSCFG_CFGR2,
    _reserved2: [u8; 0x20],
    ///0x3c - SYSCFG configuration register 3
    pub syscfg_cfgr3: SYSCFG_CFGR3,
    _reserved3: [u8; 0x40],
    ///0x80 - SYSCFG interrupt line 0 status register
    pub syscfg_itline0: SYSCFG_ITLINE0,
    _reserved4: [u8; 0x04],
    ///0x88 - SYSCFG interrupt line 2 status register
    pub syscfg_itline2: SYSCFG_ITLINE2,
    ///0x8c - SYSCFG interrupt line 3 status register
    pub syscfg_itline3: SYSCFG_ITLINE3,
    ///0x90 - SYSCFG interrupt line 4 status register
    pub syscfg_itline4: SYSCFG_ITLINE4,
    ///0x94 - SYSCFG interrupt line 5 status register
    pub syscfg_itline5: SYSCFG_ITLINE5,
    ///0x98 - SYSCFG interrupt line 6 status register
    pub syscfg_itline6: SYSCFG_ITLINE6,
    ///0x9c - SYSCFG interrupt line 7 status register
    pub syscfg_itline7: SYSCFG_ITLINE7,
    _reserved10: [u8; 0x04],
    ///0xa4 - SYSCFG interrupt line 9 status register
    pub syscfg_itline9: SYSCFG_ITLINE9,
    ///0xa8 - SYSCFG interrupt line 10 status register
    pub syscfg_itline10: SYSCFG_ITLINE10,
    ///0xac - SYSCFG interrupt line 11 status register
    pub syscfg_itline11: SYSCFG_ITLINE11,
    ///0xb0 - SYSCFG interrupt line 12 status register
    pub syscfg_itline12: SYSCFG_ITLINE12,
    ///0xb4 - SYSCFG interrupt line 13 status register
    pub syscfg_itline13: SYSCFG_ITLINE13,
    ///0xb8 - SYSCFG interrupt line 14 status register
    pub syscfg_itline14: SYSCFG_ITLINE14,
    _reserved16: [u8; 0x04],
    ///0xc0 - SYSCFG interrupt line 16 status register
    pub syscfg_itline16: SYSCFG_ITLINE16,
    _reserved17: [u8; 0x08],
    ///0xcc - SYSCFG interrupt line 19 status register
    pub syscfg_itline19: SYSCFG_ITLINE19,
    _reserved18: [u8; 0x04],
    ///0xd4 - SYSCFG interrupt line 21 status register
    pub syscfg_itline21: SYSCFG_ITLINE21,
    ///0xd8 - SYSCFG interrupt line 22 status register
    pub syscfg_itline22: SYSCFG_ITLINE22,
    ///0xdc - SYSCFG interrupt line 23 status register
    pub syscfg_itline23: SYSCFG_ITLINE23,
    _reserved21: [u8; 0x04],
    ///0xe4 - SYSCFG interrupt line 25 status register
    pub syscfg_itline25: SYSCFG_ITLINE25,
    _reserved22: [u8; 0x04],
    ///0xec - SYSCFG interrupt line 27 status register
    pub syscfg_itline27: SYSCFG_ITLINE27,
    ///0xf0 - SYSCFG interrupt line 28 status register
    pub syscfg_itline28: SYSCFG_ITLINE28,
}
///SYSCFG_CFGR1 (rw) register accessor: an alias for `Reg<SYSCFG_CFGR1_SPEC>`
pub type SYSCFG_CFGR1 = crate::Reg<syscfg_cfgr1::SYSCFG_CFGR1_SPEC>;
///SYSCFG configuration register 1
pub mod syscfg_cfgr1;
///SYSCFG_CFGR2 (rw) register accessor: an alias for `Reg<SYSCFG_CFGR2_SPEC>`
pub type SYSCFG_CFGR2 = crate::Reg<syscfg_cfgr2::SYSCFG_CFGR2_SPEC>;
///SYSCFG configuration register 2
pub mod syscfg_cfgr2;
///SYSCFG_CFGR3 (rw) register accessor: an alias for `Reg<SYSCFG_CFGR3_SPEC>`
pub type SYSCFG_CFGR3 = crate::Reg<syscfg_cfgr3::SYSCFG_CFGR3_SPEC>;
///SYSCFG configuration register 3
pub mod syscfg_cfgr3;
///SYSCFG_ITLINE0 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE0_SPEC>`
pub type SYSCFG_ITLINE0 = crate::Reg<syscfg_itline0::SYSCFG_ITLINE0_SPEC>;
///SYSCFG interrupt line 0 status register
pub mod syscfg_itline0;
///SYSCFG_ITLINE2 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE2_SPEC>`
pub type SYSCFG_ITLINE2 = crate::Reg<syscfg_itline2::SYSCFG_ITLINE2_SPEC>;
///SYSCFG interrupt line 2 status register
pub mod syscfg_itline2;
///SYSCFG_ITLINE3 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE3_SPEC>`
pub type SYSCFG_ITLINE3 = crate::Reg<syscfg_itline3::SYSCFG_ITLINE3_SPEC>;
///SYSCFG interrupt line 3 status register
pub mod syscfg_itline3;
///SYSCFG_ITLINE4 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE4_SPEC>`
pub type SYSCFG_ITLINE4 = crate::Reg<syscfg_itline4::SYSCFG_ITLINE4_SPEC>;
///SYSCFG interrupt line 4 status register
pub mod syscfg_itline4;
///SYSCFG_ITLINE5 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE5_SPEC>`
pub type SYSCFG_ITLINE5 = crate::Reg<syscfg_itline5::SYSCFG_ITLINE5_SPEC>;
///SYSCFG interrupt line 5 status register
pub mod syscfg_itline5;
///SYSCFG_ITLINE6 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE6_SPEC>`
pub type SYSCFG_ITLINE6 = crate::Reg<syscfg_itline6::SYSCFG_ITLINE6_SPEC>;
///SYSCFG interrupt line 6 status register
pub mod syscfg_itline6;
///SYSCFG_ITLINE7 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE7_SPEC>`
pub type SYSCFG_ITLINE7 = crate::Reg<syscfg_itline7::SYSCFG_ITLINE7_SPEC>;
///SYSCFG interrupt line 7 status register
pub mod syscfg_itline7;
///SYSCFG_ITLINE9 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE9_SPEC>`
pub type SYSCFG_ITLINE9 = crate::Reg<syscfg_itline9::SYSCFG_ITLINE9_SPEC>;
///SYSCFG interrupt line 9 status register
pub mod syscfg_itline9;
///SYSCFG_ITLINE10 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE10_SPEC>`
pub type SYSCFG_ITLINE10 = crate::Reg<syscfg_itline10::SYSCFG_ITLINE10_SPEC>;
///SYSCFG interrupt line 10 status register
pub mod syscfg_itline10;
///SYSCFG_ITLINE11 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE11_SPEC>`
pub type SYSCFG_ITLINE11 = crate::Reg<syscfg_itline11::SYSCFG_ITLINE11_SPEC>;
///SYSCFG interrupt line 11 status register
pub mod syscfg_itline11;
///SYSCFG_ITLINE12 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE12_SPEC>`
pub type SYSCFG_ITLINE12 = crate::Reg<syscfg_itline12::SYSCFG_ITLINE12_SPEC>;
///SYSCFG interrupt line 12 status register
pub mod syscfg_itline12;
///SYSCFG_ITLINE13 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE13_SPEC>`
pub type SYSCFG_ITLINE13 = crate::Reg<syscfg_itline13::SYSCFG_ITLINE13_SPEC>;
///SYSCFG interrupt line 13 status register
pub mod syscfg_itline13;
///SYSCFG_ITLINE14 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE14_SPEC>`
pub type SYSCFG_ITLINE14 = crate::Reg<syscfg_itline14::SYSCFG_ITLINE14_SPEC>;
///SYSCFG interrupt line 14 status register
pub mod syscfg_itline14;
///SYSCFG_ITLINE16 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE16_SPEC>`
pub type SYSCFG_ITLINE16 = crate::Reg<syscfg_itline16::SYSCFG_ITLINE16_SPEC>;
///SYSCFG interrupt line 16 status register
pub mod syscfg_itline16;
///SYSCFG_ITLINE19 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE19_SPEC>`
pub type SYSCFG_ITLINE19 = crate::Reg<syscfg_itline19::SYSCFG_ITLINE19_SPEC>;
///SYSCFG interrupt line 19 status register
pub mod syscfg_itline19;
///SYSCFG_ITLINE21 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE21_SPEC>`
pub type SYSCFG_ITLINE21 = crate::Reg<syscfg_itline21::SYSCFG_ITLINE21_SPEC>;
///SYSCFG interrupt line 21 status register
pub mod syscfg_itline21;
///SYSCFG_ITLINE22 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE22_SPEC>`
pub type SYSCFG_ITLINE22 = crate::Reg<syscfg_itline22::SYSCFG_ITLINE22_SPEC>;
///SYSCFG interrupt line 22 status register
pub mod syscfg_itline22;
///SYSCFG_ITLINE23 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE23_SPEC>`
pub type SYSCFG_ITLINE23 = crate::Reg<syscfg_itline23::SYSCFG_ITLINE23_SPEC>;
///SYSCFG interrupt line 23 status register
pub mod syscfg_itline23;
///SYSCFG_ITLINE25 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE25_SPEC>`
pub type SYSCFG_ITLINE25 = crate::Reg<syscfg_itline25::SYSCFG_ITLINE25_SPEC>;
///SYSCFG interrupt line 25 status register
pub mod syscfg_itline25;
///SYSCFG_ITLINE27 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE27_SPEC>`
pub type SYSCFG_ITLINE27 = crate::Reg<syscfg_itline27::SYSCFG_ITLINE27_SPEC>;
///SYSCFG interrupt line 27 status register
pub mod syscfg_itline27;
///SYSCFG_ITLINE28 (r) register accessor: an alias for `Reg<SYSCFG_ITLINE28_SPEC>`
pub type SYSCFG_ITLINE28 = crate::Reg<syscfg_itline28::SYSCFG_ITLINE28_SPEC>;
///SYSCFG interrupt line 28 status register
pub mod syscfg_itline28;
