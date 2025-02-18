///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SRAM/NOR-Flash chip-select control register 1
    pub bcr1: BCR1,
    ///0x04 - SRAM/NOR-Flash chip-select timing register 1
    pub btr1: BTR,
    ///0x08 - SRAM/NOR-Flash chip-select control register 2
    pub bcr2: BCR,
    ///0x0c - SRAM/NOR-Flash chip-select timing register 1
    pub btr2: BTR,
    ///0x10 - SRAM/NOR-Flash chip-select control register 2
    pub bcr3: BCR,
    ///0x14 - SRAM/NOR-Flash chip-select timing register 1
    pub btr3: BTR,
    ///0x18 - SRAM/NOR-Flash chip-select control register 2
    pub bcr4: BCR,
    ///0x1c - SRAM/NOR-Flash chip-select timing register 1
    pub btr4: BTR,
    _reserved8: [u8; 0xe4],
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    pub bwtr1: BWTR,
    _reserved9: [u8; 0x04],
    ///0x10c - SRAM/NOR-Flash write timing registers 1
    pub bwtr2: BWTR,
    _reserved10: [u8; 0x04],
    ///0x114 - SRAM/NOR-Flash write timing registers 1
    pub bwtr3: BWTR,
    _reserved11: [u8; 0x04],
    ///0x11c - SRAM/NOR-Flash write timing registers 1
    pub bwtr4: BWTR,
}
///BCR1 (rw) register accessor: an alias for `Reg<BCR1_SPEC>`
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
///SRAM/NOR-Flash chip-select control register 1
pub mod bcr1;
///BTR (rw) register accessor: an alias for `Reg<BTR_SPEC>`
pub type BTR = crate::Reg<btr::BTR_SPEC>;
///SRAM/NOR-Flash chip-select timing register 1
pub mod btr;
///BCR (rw) register accessor: an alias for `Reg<BCR_SPEC>`
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
///SRAM/NOR-Flash chip-select control register 2
pub mod bcr;
///BWTR (rw) register accessor: an alias for `Reg<BWTR_SPEC>`
pub type BWTR = crate::Reg<bwtr::BWTR_SPEC>;
///SRAM/NOR-Flash write timing registers 1
pub mod bwtr;
