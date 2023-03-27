///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SRAM/NOR-Flash chip-select control register 1
    pub bcr1: BCR1,
    ///0x04 - SRAM/NOR-Flash chip-select timing register 1
    pub btr1: BTR1,
    ///0x08 - SRAM/NOR-Flash chip-select control register 2
    pub bcr2: BCR2,
    ///0x0c - SRAM/NOR-Flash chip-select timing register 2
    pub btr2: BTR2,
    ///0x10 - SRAM/NOR-Flash chip-select control register 3
    pub bcr3: BCR3,
    ///0x14 - SRAM/NOR-Flash chip-select timing register 3
    pub btr3: BTR3,
    ///0x18 - SRAM/NOR-Flash chip-select control register 4
    pub bcr4: BCR4,
    ///0x1c - SRAM/NOR-Flash chip-select timing register 4
    pub btr4: BTR4,
    ///0x20 - PSRAM chip select counter register
    pub pcscntr: PCSCNTR,
    _reserved9: [u8; 0x5c],
    ///0x80 - PC Card/NAND Flash control register 3
    pub pcr: PCR,
    ///0x84 - FIFO status and interrupt register 3
    pub sr: SR,
    ///0x88 - Common memory space timing register 3
    pub pmem: PMEM,
    ///0x8c - Attribute memory space timing register 3
    pub patt: PATT,
    _reserved13: [u8; 0x04],
    ///0x94 - ECC result register 3
    pub eccr: ECCR,
    _reserved14: [u8; 0x6c],
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    pub bwtr1: BWTR1,
    _reserved15: [u8; 0x04],
    ///0x10c - SRAM/NOR-Flash write timing registers 2
    pub bwtr2: BWTR2,
    _reserved16: [u8; 0x04],
    ///0x114 - SRAM/NOR-Flash write timing registers 3
    pub bwtr3: BWTR3,
    _reserved17: [u8; 0x04],
    ///0x11c - SRAM/NOR-Flash write timing registers 4
    pub bwtr4: BWTR4,
}
///BCR1 (rw) register accessor: an alias for `Reg<BCR1_SPEC>`
pub type BCR1 = crate::Reg<bcr1::BCR1_SPEC>;
///SRAM/NOR-Flash chip-select control register 1
pub mod bcr1;
///BTR1 (rw) register accessor: an alias for `Reg<BTR1_SPEC>`
pub type BTR1 = crate::Reg<btr1::BTR1_SPEC>;
///SRAM/NOR-Flash chip-select timing register 1
pub mod btr1;
///BCR2 (rw) register accessor: an alias for `Reg<BCR2_SPEC>`
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
///SRAM/NOR-Flash chip-select control register 2
pub mod bcr2;
///BTR2 (rw) register accessor: an alias for `Reg<BTR2_SPEC>`
pub type BTR2 = crate::Reg<btr2::BTR2_SPEC>;
///SRAM/NOR-Flash chip-select timing register 2
pub mod btr2;
///BCR3 (rw) register accessor: an alias for `Reg<BCR3_SPEC>`
pub type BCR3 = crate::Reg<bcr3::BCR3_SPEC>;
///SRAM/NOR-Flash chip-select control register 3
pub mod bcr3;
///BTR3 (rw) register accessor: an alias for `Reg<BTR3_SPEC>`
pub type BTR3 = crate::Reg<btr3::BTR3_SPEC>;
///SRAM/NOR-Flash chip-select timing register 3
pub mod btr3;
///BCR4 (rw) register accessor: an alias for `Reg<BCR4_SPEC>`
pub type BCR4 = crate::Reg<bcr4::BCR4_SPEC>;
///SRAM/NOR-Flash chip-select control register 4
pub mod bcr4;
///BTR4 (rw) register accessor: an alias for `Reg<BTR4_SPEC>`
pub type BTR4 = crate::Reg<btr4::BTR4_SPEC>;
///SRAM/NOR-Flash chip-select timing register 4
pub mod btr4;
///PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
///PC Card/NAND Flash control register 3
pub mod pcr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///FIFO status and interrupt register 3
pub mod sr;
///PMEM (rw) register accessor: an alias for `Reg<PMEM_SPEC>`
pub type PMEM = crate::Reg<pmem::PMEM_SPEC>;
///Common memory space timing register 3
pub mod pmem;
///PATT (rw) register accessor: an alias for `Reg<PATT_SPEC>`
pub type PATT = crate::Reg<patt::PATT_SPEC>;
///Attribute memory space timing register 3
pub mod patt;
///ECCR (r) register accessor: an alias for `Reg<ECCR_SPEC>`
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
///ECC result register 3
pub mod eccr;
///BWTR1 (rw) register accessor: an alias for `Reg<BWTR1_SPEC>`
pub type BWTR1 = crate::Reg<bwtr1::BWTR1_SPEC>;
///SRAM/NOR-Flash write timing registers 1
pub mod bwtr1;
///BWTR2 (rw) register accessor: an alias for `Reg<BWTR2_SPEC>`
pub type BWTR2 = crate::Reg<bwtr2::BWTR2_SPEC>;
///SRAM/NOR-Flash write timing registers 2
pub mod bwtr2;
///BWTR3 (rw) register accessor: an alias for `Reg<BWTR3_SPEC>`
pub type BWTR3 = crate::Reg<bwtr3::BWTR3_SPEC>;
///SRAM/NOR-Flash write timing registers 3
pub mod bwtr3;
///BWTR4 (rw) register accessor: an alias for `Reg<BWTR4_SPEC>`
pub type BWTR4 = crate::Reg<bwtr4::BWTR4_SPEC>;
///SRAM/NOR-Flash write timing registers 4
pub mod bwtr4;
///PCSCNTR (rw) register accessor: an alias for `Reg<PCSCNTR_SPEC>`
pub type PCSCNTR = crate::Reg<pcscntr::PCSCNTR_SPEC>;
///PSRAM chip select counter register
pub mod pcscntr;
