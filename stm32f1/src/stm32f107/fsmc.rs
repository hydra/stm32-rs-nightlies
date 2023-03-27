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
    _reserved8: [u8; 0x40],
    ///0x60 - PC Card/NAND Flash control register 2
    pub pcr2: PCR2,
    ///0x64 - FIFO status and interrupt register 2
    pub sr2: SR2,
    ///0x68 - Common memory space timing register 2
    pub pmem2: PMEM2,
    ///0x6c - Attribute memory space timing register 2
    pub patt2: PATT2,
    _reserved12: [u8; 0x04],
    ///0x74 - ECC result register 2
    pub eccr2: ECCR2,
    _reserved13: [u8; 0x08],
    ///0x80 - PC Card/NAND Flash control register 3
    pub pcr3: PCR3,
    ///0x84 - FIFO status and interrupt register 3
    pub sr3: SR3,
    ///0x88 - Common memory space timing register 3
    pub pmem3: PMEM3,
    ///0x8c - Attribute memory space timing register 3
    pub patt3: PATT3,
    _reserved17: [u8; 0x04],
    ///0x94 - ECC result register 3
    pub eccr3: ECCR3,
    _reserved18: [u8; 0x08],
    ///0xa0 - PC Card/NAND Flash control register 4
    pub pcr4: PCR4,
    ///0xa4 - FIFO status and interrupt register 4
    pub sr4: SR4,
    ///0xa8 - Common memory space timing register 4
    pub pmem4: PMEM4,
    ///0xac - Attribute memory space timing register 4
    pub patt4: PATT4,
    ///0xb0 - I/O space timing register 4
    pub pio4: PIO4,
    _reserved23: [u8; 0x50],
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    pub bwtr1: BWTR1,
    _reserved24: [u8; 0x04],
    ///0x10c - SRAM/NOR-Flash write timing registers 2
    pub bwtr2: BWTR2,
    _reserved25: [u8; 0x04],
    ///0x114 - SRAM/NOR-Flash write timing registers 3
    pub bwtr3: BWTR3,
    _reserved26: [u8; 0x04],
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
///PCR2 (rw) register accessor: an alias for `Reg<PCR2_SPEC>`
pub type PCR2 = crate::Reg<pcr2::PCR2_SPEC>;
///PC Card/NAND Flash control register 2
pub mod pcr2;
///SR2 (rw) register accessor: an alias for `Reg<SR2_SPEC>`
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
///FIFO status and interrupt register 2
pub mod sr2;
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
///PCR3 (rw) register accessor: an alias for `Reg<PCR3_SPEC>`
pub type PCR3 = crate::Reg<pcr3::PCR3_SPEC>;
///PC Card/NAND Flash control register 3
pub mod pcr3;
///SR3 (rw) register accessor: an alias for `Reg<SR3_SPEC>`
pub type SR3 = crate::Reg<sr3::SR3_SPEC>;
///FIFO status and interrupt register 3
pub mod sr3;
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
///PCR4 (rw) register accessor: an alias for `Reg<PCR4_SPEC>`
pub type PCR4 = crate::Reg<pcr4::PCR4_SPEC>;
///PC Card/NAND Flash control register 4
pub mod pcr4;
///SR4 (rw) register accessor: an alias for `Reg<SR4_SPEC>`
pub type SR4 = crate::Reg<sr4::SR4_SPEC>;
///FIFO status and interrupt register 4
pub mod sr4;
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
