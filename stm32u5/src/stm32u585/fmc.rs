///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SRAM/NOR-Flash chip-select control register for bank 1
    pub bcr1: BCR1,
    ///0x04 - SRAM/NOR-Flash chip-select timing register for bank 1
    pub btr1: BTR1,
    ///0x08 - SRAM/NOR-Flash chip-select control register for bank 2
    pub bcr2: BCR2,
    ///0x0c - SRAM/NOR-Flash chip-select timing register for bank 2
    pub btr2: BTR2,
    ///0x10 - SRAM/NOR-Flash chip-select control register for bank 3
    pub bcr3: BCR3,
    ///0x14 - SRAM/NOR-Flash chip-select timing register for bank 3
    pub btr3: BTR3,
    ///0x18 - SRAM/NOR-Flash chip-select control register for bank 4
    pub bcr4: BCR4,
    ///0x1c - SRAM/NOR-Flash chip-select timing register for bank 4
    pub btr4: BTR4,
    ///0x20 - PSRAM chip select counter register
    pub pcscntr: PCSCNTR,
    _reserved9: [u8; 0x5c],
    ///0x80 - NAND Flash control registers
    pub pcr: PCR,
    ///0x84 - status and interrupt register
    pub sr: SR,
    ///0x88 - Common memory space timing register
    pub pmem: PMEM,
    ///0x8c - The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature).
    pub patt: PATT,
    _reserved13: [u8; 0x04],
    ///0x94 - This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.
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
///SRAM/NOR-Flash chip-select control register for bank 1
pub mod bcr1;
///BCR2 (rw) register accessor: an alias for `Reg<BCR2_SPEC>`
pub type BCR2 = crate::Reg<bcr2::BCR2_SPEC>;
///SRAM/NOR-Flash chip-select control register for bank 2
pub mod bcr2;
///BCR3 (rw) register accessor: an alias for `Reg<BCR3_SPEC>`
pub type BCR3 = crate::Reg<bcr3::BCR3_SPEC>;
///SRAM/NOR-Flash chip-select control register for bank 3
pub mod bcr3;
///BCR4 (rw) register accessor: an alias for `Reg<BCR4_SPEC>`
pub type BCR4 = crate::Reg<bcr4::BCR4_SPEC>;
///SRAM/NOR-Flash chip-select control register for bank 4
pub mod bcr4;
///BTR1 (rw) register accessor: an alias for `Reg<BTR1_SPEC>`
pub type BTR1 = crate::Reg<btr1::BTR1_SPEC>;
///SRAM/NOR-Flash chip-select timing register for bank 1
pub mod btr1;
///BTR2 (rw) register accessor: an alias for `Reg<BTR2_SPEC>`
pub type BTR2 = crate::Reg<btr2::BTR2_SPEC>;
///SRAM/NOR-Flash chip-select timing register for bank 2
pub mod btr2;
///BTR3 (rw) register accessor: an alias for `Reg<BTR3_SPEC>`
pub type BTR3 = crate::Reg<btr3::BTR3_SPEC>;
///SRAM/NOR-Flash chip-select timing register for bank 3
pub mod btr3;
///BTR4 (rw) register accessor: an alias for `Reg<BTR4_SPEC>`
pub type BTR4 = crate::Reg<btr4::BTR4_SPEC>;
///SRAM/NOR-Flash chip-select timing register for bank 4
pub mod btr4;
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
///PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
///NAND Flash control registers
pub mod pcr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status and interrupt register
pub mod sr;
///PMEM (rw) register accessor: an alias for `Reg<PMEM_SPEC>`
pub type PMEM = crate::Reg<pmem::PMEM_SPEC>;
///Common memory space timing register
pub mod pmem;
///PATT (rw) register accessor: an alias for `Reg<PATT_SPEC>`
pub type PATT = crate::Reg<patt::PATT_SPEC>;
///The FMC_PATT read/write register contains the timing information for NAND Flash memory bank. It is used for 8-bit accesses to the attribute memory space of the NAND Flash for the last address write access if the timing must differ from that of previous accesses (for Ready/Busy management, refer to Section20.8.5: NAND Flash prewait feature).
pub mod patt;
///ECCR (r) register accessor: an alias for `Reg<ECCR_SPEC>`
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
///This register contain the current error correction code value computed by the ECC computation modules of the FMC NAND controller. When the CPU reads/writes the data from a NAND Flash memory page at the correct address (refer to Section20.8.6: Computation of the error correction code (ECC) in NAND Flash memory), the data read/written from/to the NAND Flash memory are processed automatically by the ECC computation module. When X bytes have been read (according to the ECCPS field in the FMC_PCR registers), the CPU must read the computed ECC value from the FMC_ECC registers. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and, to correct it otherwise. The FMC_ECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.
pub mod eccr;
