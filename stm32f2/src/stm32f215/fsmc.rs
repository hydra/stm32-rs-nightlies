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
    _reserved8: [u8; 0x40],
    ///0x60 - PC Card/NAND Flash control register 2
    pub pcr2: PCR,
    ///0x64 - FIFO status and interrupt register 2
    pub sr2: SR,
    ///0x68 - Common memory space timing register 2
    pub pmem2: PMEM2,
    ///0x6c - Attribute memory space timing register 2
    pub patt2: PATT2,
    _reserved12: [u8; 0x04],
    ///0x74 - ECC result register 2
    pub eccr2: ECCR2,
    _reserved13: [u8; 0x08],
    ///0x80 - PC Card/NAND Flash control register 2
    pub pcr3: PCR,
    ///0x84 - FIFO status and interrupt register 2
    pub sr3: SR,
    ///0x88 - Common memory space timing register 3
    pub pmem3: PMEM3,
    ///0x8c - Attribute memory space timing register 3
    pub patt3: PATT3,
    _reserved17: [u8; 0x04],
    ///0x94 - ECC result register 3
    pub eccr3: ECCR3,
    _reserved18: [u8; 0x08],
    ///0xa0 - PC Card/NAND Flash control register 2
    pub pcr4: PCR,
    ///0xa4 - FIFO status and interrupt register 2
    pub sr4: SR,
    ///0xa8 - Common memory space timing register 4
    pub pmem4: PMEM4,
    ///0xac - Attribute memory space timing register 4
    pub patt4: PATT4,
    ///0xb0 - I/O space timing register 4
    pub pio4: PIO4,
    _reserved23: [u8; 0x50],
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    pub bwtr1: BWTR,
    _reserved24: [u8; 0x04],
    ///0x10c - SRAM/NOR-Flash write timing registers 1
    pub bwtr2: BWTR,
    _reserved25: [u8; 0x04],
    ///0x114 - SRAM/NOR-Flash write timing registers 1
    pub bwtr3: BWTR,
    _reserved26: [u8; 0x04],
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
///PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
///PC Card/NAND Flash control register 2
pub mod pcr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///FIFO status and interrupt register 2
pub mod sr;
///PMEM2 (rw) register accessor: an alias for `Reg<PMEM2_SPEC>`
pub type PMEM2 = crate::Reg<pmem2::PMEM2_SPEC>;
///Common memory space timing register 2
pub mod pmem2;
///PATT2 (rw) register accessor: an alias for `Reg<PATT2_SPEC>`
pub type PATT2 = crate::Reg<patt2::PATT2_SPEC>;
///Attribute memory space timing register 2
pub mod patt2;
///ECCR2 (r) register accessor: an alias for `Reg<ECCR2_SPEC>`
pub type ECCR2 = crate::Reg<eccr2::ECCR2_SPEC>;
///ECC result register 2
pub mod eccr2;
///PMEM3 (rw) register accessor: an alias for `Reg<PMEM3_SPEC>`
pub type PMEM3 = crate::Reg<pmem3::PMEM3_SPEC>;
///Common memory space timing register 3
pub mod pmem3;
///PATT3 (rw) register accessor: an alias for `Reg<PATT3_SPEC>`
pub type PATT3 = crate::Reg<patt3::PATT3_SPEC>;
///Attribute memory space timing register 3
pub mod patt3;
///ECCR3 (r) register accessor: an alias for `Reg<ECCR3_SPEC>`
pub type ECCR3 = crate::Reg<eccr3::ECCR3_SPEC>;
///ECC result register 3
pub mod eccr3;
///PMEM4 (rw) register accessor: an alias for `Reg<PMEM4_SPEC>`
pub type PMEM4 = crate::Reg<pmem4::PMEM4_SPEC>;
///Common memory space timing register 4
pub mod pmem4;
///PATT4 (rw) register accessor: an alias for `Reg<PATT4_SPEC>`
pub type PATT4 = crate::Reg<patt4::PATT4_SPEC>;
///Attribute memory space timing register 4
pub mod patt4;
///PIO4 (rw) register accessor: an alias for `Reg<PIO4_SPEC>`
pub type PIO4 = crate::Reg<pio4::PIO4_SPEC>;
///I/O space timing register 4
pub mod pio4;
///BWTR (rw) register accessor: an alias for `Reg<BWTR_SPEC>`
pub type BWTR = crate::Reg<bwtr::BWTR_SPEC>;
///SRAM/NOR-Flash write timing registers 1
pub mod bwtr;
