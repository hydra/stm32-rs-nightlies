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
    _reserved8: [u8; 0x60],
    ///0x80 - PC Card/NAND Flash control register 3
    pub pcr: PCR,
    ///0x84 - FIFO status and interrupt register 3
    pub sr: SR,
    ///0x88 - Common memory space timing register 3
    pub pmem: PMEM,
    ///0x8c - Attribute memory space timing register 3
    pub patt: PATT,
    _reserved12: [u8; 0x04],
    ///0x94 - ECC result register 3
    pub eccr: ECCR,
    _reserved13: [u8; 0x6c],
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    pub bwtr1: BWTR,
    _reserved14: [u8; 0x04],
    ///0x10c - SRAM/NOR-Flash write timing registers 1
    pub bwtr2: BWTR,
    _reserved15: [u8; 0x04],
    ///0x114 - SRAM/NOR-Flash write timing registers 1
    pub bwtr3: BWTR,
    _reserved16: [u8; 0x04],
    ///0x11c - SRAM/NOR-Flash write timing registers 1
    pub bwtr4: BWTR,
    _reserved17: [u8; 0x20],
    ///0x140..0x148 - SDRAM Control Register 1
    pub sdcr: [SDCR; 2],
    ///0x148..0x150 - SDRAM Timing register 1
    pub sdtr: [SDTR; 2],
    ///0x150 - SDRAM Command Mode register
    pub sdcmr: SDCMR,
    ///0x154 - SDRAM Refresh Timer register
    pub sdrtr: SDRTR,
    ///0x158 - SDRAM Status register
    pub sdsr: SDSR,
}
impl RegisterBlock {
    ///0x140 - SDRAM Control Register 1
    #[inline(always)]
    pub fn sdcr1(&self) -> &SDCR {
        &self.sdcr[0]
    }
    ///0x144 - SDRAM Control Register 1
    #[inline(always)]
    pub fn sdcr2(&self) -> &SDCR {
        &self.sdcr[1]
    }
    ///0x148 - SDRAM Timing register 1
    #[inline(always)]
    pub fn sdtr1(&self) -> &SDTR {
        &self.sdtr[0]
    }
    ///0x14c - SDRAM Timing register 1
    #[inline(always)]
    pub fn sdtr2(&self) -> &SDTR {
        &self.sdtr[1]
    }
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
///BWTR (rw) register accessor: an alias for `Reg<BWTR_SPEC>`
pub type BWTR = crate::Reg<bwtr::BWTR_SPEC>;
///SRAM/NOR-Flash write timing registers 1
pub mod bwtr;
///SDCR (rw) register accessor: an alias for `Reg<SDCR_SPEC>`
pub type SDCR = crate::Reg<sdcr::SDCR_SPEC>;
///SDRAM Control Register 1
pub mod sdcr;
///SDTR (rw) register accessor: an alias for `Reg<SDTR_SPEC>`
pub type SDTR = crate::Reg<sdtr::SDTR_SPEC>;
///SDRAM Timing register 1
pub mod sdtr;
///SDCMR (rw) register accessor: an alias for `Reg<SDCMR_SPEC>`
pub type SDCMR = crate::Reg<sdcmr::SDCMR_SPEC>;
///SDRAM Command Mode register
pub mod sdcmr;
///SDRTR (rw) register accessor: an alias for `Reg<SDRTR_SPEC>`
pub type SDRTR = crate::Reg<sdrtr::SDRTR_SPEC>;
///SDRAM Refresh Timer register
pub mod sdrtr;
///SDSR (r) register accessor: an alias for `Reg<SDSR_SPEC>`
pub type SDSR = crate::Reg<sdsr::SDSR_SPEC>;
///SDRAM Status register
pub mod sdsr;
